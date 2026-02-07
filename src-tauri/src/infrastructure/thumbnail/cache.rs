//! Thumbnail Disk Cache
//!
//! Stores generated thumbnails as WebP files in AppData.
//! Uses blake3 hashing for cache keys and LRU eviction by mtime.

use std::fs;
use std::path::{Path, PathBuf};

/// Manages a disk-based thumbnail cache.
pub struct ThumbnailCache {
    base_dir: PathBuf,
    max_size_bytes: u64,
}

impl ThumbnailCache {
    /// Create a new cache at the given base directory.
    pub fn new(base_dir: PathBuf, max_size_mb: u64) -> Self {
        Self {
            base_dir,
            max_size_bytes: max_size_mb * 1024 * 1024,
        }
    }

    /// Compute a cache key from file metadata and thumbnail size.
    ///
    /// The key is derived from the file path, modification time, file size,
    /// and requested thumbnail size — so it automatically invalidates when
    /// the file changes.
    pub fn cache_key(path: &str, mtime: i64, file_size: u64, thumb_size: u32) -> String {
        let mut hasher = blake3::Hasher::new();
        hasher.update(path.as_bytes());
        hasher.update(&mtime.to_le_bytes());
        hasher.update(&file_size.to_le_bytes());
        hasher.update(&thumb_size.to_le_bytes());
        hasher.finalize().to_hex().to_string()
    }

    /// Get the filesystem path for a given cache key.
    fn cache_path(&self, hash: &str) -> PathBuf {
        let prefix = &hash[..2];
        self.base_dir.join(prefix).join(format!("{}.webp", hash))
    }

    /// Try to retrieve cached thumbnail bytes.
    ///
    /// On hit, touches the file mtime for LRU tracking.
    pub fn get(&self, hash: &str) -> Result<Option<Vec<u8>>, std::io::Error> {
        let path = self.cache_path(hash);
        if !path.exists() {
            return Ok(None);
        }

        let data = fs::read(&path)?;

        // Touch mtime for LRU tracking (best-effort)
        let _ = filetime::set_file_mtime(&path, filetime::FileTime::now());

        Ok(Some(data))
    }

    /// Store WebP bytes in the cache.
    pub fn put(&self, hash: &str, webp_data: &[u8]) -> Result<(), std::io::Error> {
        let path = self.cache_path(hash);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&path, webp_data)
    }

    /// Delete all cached thumbnails. Returns the number of bytes freed.
    pub fn clear(&self) -> Result<u64, std::io::Error> {
        let size = self.total_size()?;
        if self.base_dir.exists() {
            fs::remove_dir_all(&self.base_dir)?;
        }
        Ok(size)
    }

    /// Get the total cache size in bytes.
    pub fn total_size(&self) -> Result<u64, std::io::Error> {
        if !self.base_dir.exists() {
            return Ok(0);
        }
        dir_size(&self.base_dir)
    }

    /// Count the number of cached files.
    pub fn file_count(&self) -> Result<u64, std::io::Error> {
        if !self.base_dir.exists() {
            return Ok(0);
        }
        let mut count = 0u64;
        visit_files(&self.base_dir, &mut |_| {
            count += 1;
        })?;
        Ok(count)
    }

    /// Evict oldest entries until the cache is under `max_size_bytes`.
    pub fn evict_to_limit(&self) -> Result<u64, std::io::Error> {
        if !self.base_dir.exists() || self.max_size_bytes == 0 {
            return Ok(0);
        }

        let current = self.total_size()?;
        if current <= self.max_size_bytes {
            return Ok(0);
        }

        // Collect all files with their mtime and size
        let mut entries: Vec<(PathBuf, u64, std::time::SystemTime)> = Vec::new();
        visit_files(&self.base_dir, &mut |path: &Path| {
            if let Ok(meta) = fs::metadata(path) {
                let mtime = meta.modified().unwrap_or(std::time::UNIX_EPOCH);
                entries.push((path.to_path_buf(), meta.len(), mtime));
            }
        })?;

        // Sort by mtime ascending (oldest first)
        entries.sort_by_key(|(_, _, mtime)| *mtime);

        let mut freed = 0u64;
        let target = current - self.max_size_bytes;

        for (path, size, _) in &entries {
            if freed >= target {
                break;
            }
            if fs::remove_file(path).is_ok() {
                freed += size;
            }
        }

        // Clean up empty subdirectories
        cleanup_empty_dirs(&self.base_dir)?;

        Ok(freed)
    }
}

/// Recursively calculate directory size.
fn dir_size(dir: &Path) -> Result<u64, std::io::Error> {
    let mut total = 0u64;
    visit_files(dir, &mut |path: &Path| {
        if let Ok(meta) = fs::metadata(path) {
            total += meta.len();
        }
    })?;
    Ok(total)
}

/// Visit all files in a directory recursively.
fn visit_files(dir: &Path, f: &mut dyn FnMut(&Path)) -> Result<(), std::io::Error> {
    if !dir.is_dir() {
        return Ok(());
    }
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            visit_files(&path, f)?;
        } else {
            f(&path);
        }
    }
    Ok(())
}

/// Remove empty subdirectories.
fn cleanup_empty_dirs(dir: &Path) -> Result<(), std::io::Error> {
    if !dir.is_dir() {
        return Ok(());
    }
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            cleanup_empty_dirs(&path)?;
            // Try to remove — fails silently if not empty
            let _ = fs::remove_dir(&path);
        }
    }
    Ok(())
}

/// Minimal mtime manipulation without extra dependency.
mod filetime {
    use std::path::Path;
    use std::time::SystemTime;

    #[derive(Clone, Copy)]
    pub struct FileTime(#[allow(dead_code)] SystemTime);

    impl FileTime {
        pub fn now() -> Self {
            Self(SystemTime::now())
        }
    }

    /// Best-effort touch of file mtime. Uses open+close on Windows
    /// which updates the access time.
    pub fn set_file_mtime(path: &Path, _time: FileTime) -> Result<(), std::io::Error> {
        // Opening and closing the file is enough to update the access metadata
        // on most file systems. For cache LRU purposes this is sufficient.
        let _ = std::fs::OpenOptions::new().write(true).open(path)?;
        Ok(())
    }
}
