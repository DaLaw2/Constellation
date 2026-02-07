//! USN Refresh Service
//!
//! On-demand file index refresh using the NTFS USN Change Journal.
//! Supports same-volume path updates and cross-volume move detection.

use crate::application::dto::{DriveUsnStatusDto, RefreshResultDto, RefreshedItemDto};
use crate::application::services::SettingsService;
use crate::domain::errors::DomainError;
use crate::domain::repositories::ItemRepository;
use deadpool_sqlite::Pool;
use rusqlite::{Connection, OptionalExtension};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

#[cfg(windows)]
use crate::infrastructure::usn_journal::{
    is_ntfs, read_journal_records, resolve_path_by_frn, RawUsnRecord, VolumeHandle,
};

/// USN reason flags for matching.
#[cfg(windows)]
const USN_REASON_FILE_CREATE: u32 = 0x0000_0100;
#[cfg(windows)]
const USN_REASON_FILE_DELETE: u32 = 0x0000_0200;
#[cfg(windows)]
const USN_REASON_RENAME_NEW_NAME: u32 = 0x0000_2000;

/// Per-drive data collected in phase 1, kept alive for cross-volume resolution.
#[cfg(windows)]
struct DriveContext {
    drive: char,
    volume: VolumeHandle,
    records: Vec<RawUsnRecord>,
    final_usn: i64,
    journal_id: u64,
}

/// An item whose file was not found on its original volume.
#[cfg(windows)]
struct PendingDelete {
    item_id: i64,
    old_path: String,
}

/// Service for on-demand file index refresh via USN Journal.
pub struct UsnRefreshService {
    pool: Arc<Pool>,
    item_repo: Arc<dyn ItemRepository>,
    settings_service: Arc<SettingsService>,
}

impl UsnRefreshService {
    pub fn new(
        pool: Arc<Pool>,
        item_repo: Arc<dyn ItemRepository>,
        settings_service: Arc<SettingsService>,
    ) -> Self {
        Self {
            pool,
            item_repo,
            settings_service,
        }
    }

    /// Refreshes the file index for the specified drives using USN Journal.
    ///
    /// Two-phase process:
    /// 1. Read USN records per drive, resolve same-volume renames, collect missing items
    /// 2. Cross-volume matching: search other drives' records for missing items by filename
    #[cfg(windows)]
    pub async fn refresh(&self, drives: &[char]) -> Result<RefreshResultDto, DomainError> {
        let mut result = RefreshResultDto::default();
        let refresh_on_missing = self.get_setting_bool("usn_refresh_on_missing", true).await;
        let cross_volume = self.get_setting_bool("usn_cross_volume_match", true).await;

        // Phase 1: process each drive, collect pending deletes and drive contexts
        let mut drive_contexts: Vec<DriveContext> = Vec::new();
        let mut all_pending_deletes: Vec<PendingDelete> = Vec::new();

        for &drive in drives {
            match self
                .process_drive(
                    drive,
                    refresh_on_missing,
                    &mut result,
                    &mut drive_contexts,
                    &mut all_pending_deletes,
                )
                .await
            {
                Ok(()) => {}
                Err(e) => {
                    eprintln!("[USN] refresh: drive {} error: {}", drive, e);
                    result.errors.push(format!("{}: {}", drive, e));
                }
            }
        }

        // Phase 2: cross-volume matching for pending deletes
        if cross_volume && !all_pending_deletes.is_empty() && drive_contexts.len() > 1 {
            self.cross_volume_match(&drive_contexts, &mut all_pending_deletes, &mut result)
                .await?;
        }

        // Phase 3: mark remaining pending deletes as deleted
        for pending in &all_pending_deletes {
            self.mark_item_deleted(pending.item_id).await?;
            result.items_updated.push(RefreshedItemDto {
                item_id: pending.item_id,
                old_path: pending.old_path.clone(),
                new_path: None,
                action: "deleted".to_string(),
            });
        }

        // Save USN state for all drives that had records processed
        for ctx in &drive_contexts {
            save_usn_state(&self.pool, ctx.drive, ctx.final_usn, ctx.journal_id).await?;
        }

        Ok(result)
    }

    #[cfg(not(windows))]
    pub async fn refresh(&self, _drives: &[char]) -> Result<RefreshResultDto, DomainError> {
        Err(DomainError::UsnJournalError(
            "USN Journal is only supported on Windows".to_string(),
        ))
    }

    /// Processes a single drive: reads USN records, resolves same-volume renames,
    /// and collects items whose files were not found (for cross-volume matching later).
    #[cfg(windows)]
    async fn process_drive(
        &self,
        drive: char,
        refresh_on_missing: bool,
        result: &mut RefreshResultDto,
        drive_contexts: &mut Vec<DriveContext>,
        pending_deletes: &mut Vec<PendingDelete>,
    ) -> Result<(), DomainError> {
        if !is_ntfs(drive)? {
            return Ok(());
        }
        result.drives_scanned.push(format!("{}:", drive));

        let volume = VolumeHandle::open(drive)?;
        let journal = match volume.query_journal() {
            Ok(j) => j,
            Err(e) if e.to_string().contains("not active") => {
                result.journal_inactive.push(format!("{}:", drive));
                return Ok(());
            }
            Err(e) => return Err(e),
        };

        let saved_state = load_usn_state(&self.pool, drive).await?;
        let first_time = saved_state.is_none();

        if first_time {
            result.first_time_drives.push(format!("{}:", drive));
            save_usn_state(&self.pool, drive, journal.next_usn, journal.journal_id).await?;
            // Still push context so volume handle is available for cross-volume FRN resolution
            drive_contexts.push(DriveContext {
                drive,
                volume,
                records: Vec::new(),
                final_usn: journal.next_usn,
                journal_id: journal.journal_id,
            });
            return Ok(());
        }

        let (saved_usn, saved_journal_id) = saved_state.unwrap();

        // Stale detection
        if saved_journal_id != journal.journal_id || saved_usn < journal.first_usn {
            result.journal_stale.push(format!("{}:", drive));
            save_usn_state(&self.pool, drive, journal.next_usn, journal.journal_id).await?;
            drive_contexts.push(DriveContext {
                drive,
                volume,
                records: Vec::new(),
                final_usn: journal.next_usn,
                journal_id: journal.journal_id,
            });
            return Ok(());
        }

        // Already caught up? Still push drive context for cross-volume FRN resolution.
        if saved_usn >= journal.next_usn {
            drive_contexts.push(DriveContext {
                drive,
                volume,
                records: Vec::new(),
                final_usn: journal.next_usn,
                journal_id: journal.journal_id,
            });
            return Ok(());
        }

        // Read USN records
        let (final_usn, records) =
            read_journal_records(volume.raw_handle(), journal.journal_id, saved_usn)?;

        if records.is_empty() {
            // Still push context for cross-volume FRN resolution
            drive_contexts.push(DriveContext {
                drive,
                volume,
                records: Vec::new(),
                final_usn,
                journal_id: journal.journal_id,
            });
            return Ok(());
        }

        // Load tracked items for this drive
        let drive_prefix = format!("{}:\\", drive.to_ascii_uppercase());
        let tracked_items = self
            .item_repo
            .find_active_by_path_prefix(&drive_prefix)
            .await?;

        if tracked_items.is_empty() {
            // No tracked items but keep drive context for cross-volume resolution
            drive_contexts.push(DriveContext {
                drive,
                volume,
                records,
                final_usn,
                journal_id: journal.journal_id,
            });
            return Ok(());
        }

        // Build FRN → Item map
        let frn_map: HashMap<u64, _> = tracked_items
            .iter()
            .filter(|item| item.file_reference_number() != 0)
            .map(|item| (item.file_reference_number(), item))
            .collect();

        // Collect FRNs from USN records
        let usn_frns: HashSet<u64> = records
            .iter()
            .filter(|r| r.reason & (USN_REASON_RENAME_NEW_NAME | USN_REASON_FILE_DELETE) != 0)
            .map(|r| r.file_reference_number)
            .collect();

        // Intersection
        let tracked_frn_set: HashSet<u64> = frn_map.keys().copied().collect();
        let matched_frns: Vec<u64> = usn_frns.intersection(&tracked_frn_set).copied().collect();

        // Process matches
        for frn in matched_frns {
            let item = frn_map[&frn];
            let item_id = item.id().unwrap_or(0);
            let old_path = item.path().to_string();

            let has_delete = records
                .iter()
                .filter(|r| r.file_reference_number == frn)
                .any(|r| r.reason & USN_REASON_FILE_DELETE != 0);

            match resolve_path_by_frn(volume.raw_handle(), frn)? {
                Some(current_path) => {
                    if current_path != old_path {
                        self.update_item_path(item_id, &current_path).await?;
                        result.items_updated.push(RefreshedItemDto {
                            item_id,
                            old_path,
                            new_path: Some(current_path),
                            action: "renamed".to_string(),
                        });
                    }
                }
                None => {
                    // File not found on this volume — defer decision
                    if has_delete || !refresh_on_missing {
                        pending_deletes.push(PendingDelete { item_id, old_path });
                    }
                }
            }
        }

        // Keep drive context alive for cross-volume resolution
        drive_contexts.push(DriveContext {
            drive,
            volume,
            records,
            final_usn,
            journal_id: journal.journal_id,
        });

        Ok(())
    }

    /// Cross-volume matching for pending deletes.
    ///
    /// FSCTL_READ_UNPRIVILEGED_USN_JOURNAL does NOT include filenames in records,
    /// so we resolve FRNs from FILE_CREATE records to full paths via the filesystem,
    /// then match by filename component.
    #[cfg(windows)]
    async fn cross_volume_match(
        &self,
        drive_contexts: &[DriveContext],
        pending_deletes: &mut Vec<PendingDelete>,
        result: &mut RefreshResultDto,
    ) -> Result<(), DomainError> {
        // Collect pending delete filenames for quick lookup
        let pending_filenames: HashSet<String> = pending_deletes
            .iter()
            .map(|p| {
                p.old_path
                    .rsplit_once('\\')
                    .map(|(_, n)| n)
                    .unwrap_or(&p.old_path)
                    .to_string()
            })
            .collect();

        // Build name_index by resolving FRNs from FILE_CREATE/RENAME_NEW records
        // on other drives to full paths, then extracting filename components.
        // filename → Vec<(full_path, frn, drive_index)>
        let mut name_index: HashMap<String, Vec<(String, u64, usize)>> = HashMap::new();

        for (idx, ctx) in drive_contexts.iter().enumerate() {
            // Collect unique FRNs from create/rename records
            let create_frns: HashSet<u64> = ctx
                .records
                .iter()
                .filter(|r| {
                    r.reason & (USN_REASON_FILE_CREATE | USN_REASON_RENAME_NEW_NAME) != 0
                })
                .map(|r| r.file_reference_number)
                .collect();

            if create_frns.is_empty() {
                continue;
            }

            for frn in &create_frns {
                match resolve_path_by_frn(ctx.volume.raw_handle(), *frn) {
                    Ok(Some(path)) => {
                        let filename = path
                            .rsplit_once('\\')
                            .map(|(_, n)| n)
                            .unwrap_or(&path)
                            .to_string();

                        // Only index if the filename matches a pending delete
                        if pending_filenames.contains(&filename) {
                            name_index
                                .entry(filename)
                                .or_default()
                                .push((path, *frn, idx));
                        }
                    }
                    Ok(None) => {} // file no longer exists
                    Err(_) => {}   // skip errors
                }
            }
        }

        // Match pending deletes against resolved paths
        let mut resolved_indices: Vec<usize> = Vec::new();

        for (i, pending) in pending_deletes.iter().enumerate() {
            let filename = pending
                .old_path
                .rsplit_once('\\')
                .map(|(_, n)| n)
                .unwrap_or(&pending.old_path);

            let source_drive = pending
                .old_path
                .chars()
                .next()
                .unwrap_or('\0')
                .to_ascii_uppercase();

            let candidates = match name_index.get(filename) {
                Some(c) => c,
                None => continue,
            };

            for (new_path, new_frn, ctx_idx) in candidates {
                let ctx = &drive_contexts[*ctx_idx];
                if ctx.drive.to_ascii_uppercase() == source_drive {
                    continue;
                }

                self.update_item_path_and_frn(pending.item_id, new_path, *new_frn)
                    .await?;
                result.items_updated.push(RefreshedItemDto {
                    item_id: pending.item_id,
                    old_path: pending.old_path.clone(),
                    new_path: Some(new_path.clone()),
                    action: "moved".to_string(),
                });
                resolved_indices.push(i);
                break;
            }
        }

        // Remove resolved items from pending_deletes (reverse order to preserve indices)
        resolved_indices.sort_unstable_by(|a, b| b.cmp(a));
        for idx in resolved_indices {
            pending_deletes.remove(idx);
        }

        Ok(())
    }

    /// Updates an item's path.
    async fn update_item_path(&self, item_id: i64, new_path: &str) -> Result<(), DomainError> {
        let mut item = self
            .item_repo
            .find_by_id(item_id)
            .await?
            .ok_or_else(|| DomainError::ItemNotFound(item_id.to_string()))?;

        let path = crate::domain::value_objects::FilePath::new(new_path)?;
        item.update_path(path);
        self.item_repo.update(&item).await
    }

    /// Updates an item's path and FRN (for cross-volume moves where FRN changes).
    #[cfg(windows)]
    async fn update_item_path_and_frn(
        &self,
        item_id: i64,
        new_path: &str,
        new_frn: u64,
    ) -> Result<(), DomainError> {
        let mut item = self
            .item_repo
            .find_by_id(item_id)
            .await?
            .ok_or_else(|| DomainError::ItemNotFound(item_id.to_string()))?;

        let path = crate::domain::value_objects::FilePath::new(new_path)?;
        item.update_path(path);
        item.update_file_reference_number(new_frn);
        self.item_repo.update(&item).await
    }

    /// Marks an item as deleted (soft delete, preserves tags).
    async fn mark_item_deleted(&self, item_id: i64) -> Result<(), DomainError> {
        let conn = self
            .pool
            .get()
            .await
            .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        conn.interact(move |conn: &mut Connection| {
            conn.execute(
                "UPDATE items SET is_deleted = 1, deleted_at = unixepoch(), updated_at = unixepoch() WHERE id = ?1",
                [item_id],
            )?;
            Ok::<(), rusqlite::Error>(())
        })
        .await
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?
        .map_err(|e| DomainError::DatabaseError(e.to_string()))
    }

    /// Reads a boolean setting with a default value.
    async fn get_setting_bool(&self, key: &str, default: bool) -> bool {
        self.settings_service
            .get(key)
            .await
            .ok()
            .flatten()
            .map(|v| v == "true")
            .unwrap_or(default)
    }

    /// Gets the USN status for all NTFS drives.
    #[cfg(windows)]
    pub async fn get_drive_status(&self) -> Result<Vec<DriveUsnStatusDto>, DomainError> {
        let mut results = Vec::new();

        for letter in 'A'..='Z' {
            match is_ntfs(letter) {
                Ok(true) => {
                    let state = load_usn_state_full(&self.pool, letter).await?;
                    let (last_usn, last_synced_at) = state.unwrap_or((0, 0));
                    results.push(DriveUsnStatusDto {
                        drive: format!("{}:", letter),
                        supported: true,
                        last_usn,
                        last_synced_at,
                    });
                }
                _ => continue,
            }
        }

        Ok(results)
    }

    #[cfg(not(windows))]
    pub async fn get_drive_status(&self) -> Result<Vec<DriveUsnStatusDto>, DomainError> {
        Ok(Vec::new())
    }
}

/// Loads USN state (last_usn, journal_id) for a drive.
async fn load_usn_state(pool: &Pool, drive: char) -> Result<Option<(i64, u64)>, DomainError> {
    let conn = pool
        .get()
        .await
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;
    let d = drive.to_ascii_uppercase().to_string();

    conn.interact(move |conn: &mut Connection| {
        conn.query_row(
            "SELECT last_usn, journal_id FROM usn_state WHERE drive_letter = ?1",
            [&d],
            |row| {
                let last_usn: i64 = row.get(0)?;
                let journal_id: i64 = row.get(1)?;
                Ok((last_usn, journal_id as u64))
            },
        )
        .optional()
    })
    .await
    .map_err(|e| DomainError::DatabaseError(e.to_string()))?
    .map_err(|e| DomainError::DatabaseError(e.to_string()))
}

/// Loads USN state (last_usn, last_synced_at) for drive status display.
async fn load_usn_state_full(pool: &Pool, drive: char) -> Result<Option<(i64, i64)>, DomainError> {
    let conn = pool
        .get()
        .await
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;
    let d = drive.to_ascii_uppercase().to_string();

    conn.interact(move |conn: &mut Connection| {
        conn.query_row(
            "SELECT last_usn, last_synced_at FROM usn_state WHERE drive_letter = ?1",
            [&d],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .optional()
    })
    .await
    .map_err(|e| DomainError::DatabaseError(e.to_string()))?
    .map_err(|e| DomainError::DatabaseError(e.to_string()))
}

/// Saves USN state for a drive.
async fn save_usn_state(
    pool: &Pool,
    drive: char,
    last_usn: i64,
    journal_id: u64,
) -> Result<(), DomainError> {
    let conn = pool
        .get()
        .await
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;
    let d = drive.to_ascii_uppercase().to_string();
    let jid = journal_id as i64;

    conn.interact(move |conn: &mut Connection| {
        conn.execute(
            "INSERT OR REPLACE INTO usn_state (drive_letter, last_usn, journal_id, last_synced_at)
             VALUES (?1, ?2, ?3, unixepoch())",
            (&d, last_usn, jid),
        )?;
        Ok::<(), rusqlite::Error>(())
    })
    .await
    .map_err(|e| DomainError::DatabaseError(e.to_string()))?
    .map_err(|e| DomainError::DatabaseError(e.to_string()))
}
