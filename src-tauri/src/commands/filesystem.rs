use crate::error::{AppError, AppResult};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Component, Path, PathBuf};

use std::os::windows::process::CommandExt;

/// Validate path to prevent path traversal attacks using ./ or ../
/// Allows access to any directory, but blocks relative path manipulation
fn validate_path(path: &str) -> AppResult<PathBuf> {
    let path_buf = PathBuf::from(path);

    // Check for path traversal patterns in components
    for component in path_buf.components() {
        match component {
            Component::ParentDir => {
                return Err(AppError::InvalidInput(
                    "Path traversal not allowed: '..' detected".to_string(),
                ));
            }
            Component::CurDir => {
                return Err(AppError::InvalidInput(
                    "Path traversal not allowed: '.' detected".to_string(),
                ));
            }
            _ => {}
        }
    }

    // Also check raw string for encoded or hidden traversal patterns
    if path.contains("..") || path.contains("./") || path.contains(".\\") {
        return Err(AppError::InvalidInput(
            "Path traversal patterns not allowed".to_string(),
        ));
    }

    Ok(path_buf)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriveInfo {
    pub letter: String,
    pub label: Option<String>,
    pub drive_type: String,
    pub total_space: Option<u64>,
    pub available_space: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEntry {
    pub name: String,
    pub path: String,
    pub is_directory: bool,
    pub size: Option<u64>,
    pub modified_time: Option<i64>,
    pub is_hidden: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetadata {
    pub path: String,
    pub size: Option<u64>,
    pub modified_time: Option<i64>,
    pub created_time: Option<i64>,
    pub is_directory: bool,
    pub is_readonly: bool,
}

/// Get all available drives on Windows
#[tauri::command]
pub async fn get_drives() -> AppResult<Vec<DriveInfo>> {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;

    let mut drives = Vec::new();

    // Get logical drives bitmask
    let drives_mask = unsafe { winapi::um::fileapi::GetLogicalDrives() };

    for i in 0..26 {
        if (drives_mask & (1 << i)) != 0 {
            let letter = (b'A' + i) as char;
            let drive_path = format!("{}:\\", letter);

            // Get drive type
            let wide_path: Vec<u16> = OsStr::new(&drive_path)
                .encode_wide()
                .chain(std::iter::once(0))
                .collect();

            let drive_type = unsafe { winapi::um::fileapi::GetDriveTypeW(wide_path.as_ptr()) };

            let drive_type_str = match drive_type {
                winapi::um::winbase::DRIVE_FIXED => "fixed",
                winapi::um::winbase::DRIVE_REMOVABLE => "removable",
                winapi::um::winbase::DRIVE_REMOTE => "network",
                winapi::um::winbase::DRIVE_CDROM => "cdrom",
                winapi::um::winbase::DRIVE_RAMDISK => "ramdisk",
                _ => "unknown",
            };

            // Only include fixed and removable drives
            if drive_type_str == "fixed" || drive_type_str == "removable" {
                // Try to get drive label and space info
                let label = get_drive_label(&drive_path);
                let (total_space, available_space) = get_drive_space(&drive_path);

                drives.push(DriveInfo {
                    letter: letter.to_string(),
                    label,
                    drive_type: drive_type_str.to_string(),
                    total_space,
                    available_space,
                });
            }
        }
    }

    Ok(drives)
}

fn get_drive_label(drive_path: &str) -> Option<String> {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;

    let wide_path: Vec<u16> = OsStr::new(drive_path)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();

    let mut volume_name_buffer = vec![0u16; 256];

    let result = unsafe {
        winapi::um::fileapi::GetVolumeInformationW(
            wide_path.as_ptr(),
            volume_name_buffer.as_mut_ptr(),
            volume_name_buffer.len() as u32,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            0,
        )
    };

    if result != 0 {
        let len = volume_name_buffer.iter().position(|&c| c == 0).unwrap_or(0);
        if len > 0 {
            return String::from_utf16(&volume_name_buffer[..len]).ok();
        }
    }

    None
}

fn get_drive_space(drive_path: &str) -> (Option<u64>, Option<u64>) {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;

    let wide_path: Vec<u16> = OsStr::new(drive_path)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();

    let mut available_bytes = 0u64;
    let mut total_bytes = 0u64;
    let mut free_bytes = 0u64;

    let result = unsafe {
        winapi::um::fileapi::GetDiskFreeSpaceExW(
            wide_path.as_ptr(),
            &mut available_bytes as *mut _ as *mut _,
            &mut total_bytes as *mut _ as *mut _,
            &mut free_bytes as *mut _ as *mut _,
        )
    };

    if result != 0 {
        (Some(total_bytes), Some(available_bytes))
    } else {
        (None, None)
    }
}

/// Read directory contents
#[tauri::command]
pub async fn read_directory(path: String) -> AppResult<Vec<FileEntry>> {
    // Validate path to prevent traversal attacks
    let path_buf = validate_path(&path)?;

    if !path_buf.exists() {
        return Err(AppError::InvalidInput(format!(
            "Path does not exist: {}",
            path
        )));
    }

    if !path_buf.is_dir() {
        return Err(AppError::InvalidInput(format!(
            "Path is not a directory: {}",
            path
        )));
    }

    let mut entries = Vec::new();

    match fs::read_dir(&path_buf) {
        Ok(dir_entries) => {
            for entry_result in dir_entries {
                match entry_result {
                    Ok(entry) => {
                        let entry_path = entry.path();
                        let metadata = entry.metadata();

                        let file_name = entry.file_name().to_string_lossy().to_string();

                        // Check if hidden (Windows)
                        let is_hidden = is_hidden_file(&entry_path);

                        // Skip hidden files by default
                        if is_hidden {
                            continue;
                        }

                        if let Ok(meta) = metadata {
                            let size = if meta.is_file() {
                                Some(meta.len())
                            } else {
                                None
                            };

                            let modified_time = meta
                                .modified()
                                .ok()
                                .and_then(|time| time.duration_since(std::time::UNIX_EPOCH).ok())
                                .map(|duration| duration.as_secs() as i64);

                            entries.push(FileEntry {
                                name: file_name,
                                path: entry_path.to_string_lossy().to_string(),
                                is_directory: meta.is_dir(),
                                size,
                                modified_time,
                                is_hidden,
                            });
                        }
                    }
                    Err(e) => {
                        eprintln!("Error reading directory entry: {}", e);
                        // Continue with other entries
                    }
                }
            }
        }
        Err(e) => {
            return Err(AppError::InvalidInput(format!(
                "Failed to read directory: {}",
                e
            )));
        }
    }

    // Sort entries: directories first, then files, alphabetically
    entries.sort_by(|a, b| match (a.is_directory, b.is_directory) {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
    });

    Ok(entries)
}

fn is_hidden_file(path: &Path) -> bool {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    use winapi::um::fileapi::{GetFileAttributesW, INVALID_FILE_ATTRIBUTES};
    use winapi::um::winnt::FILE_ATTRIBUTE_HIDDEN;

    let wide_path: Vec<u16> = OsStr::new(path)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();

    let attributes = unsafe { GetFileAttributesW(wide_path.as_ptr()) };

    if attributes == INVALID_FILE_ATTRIBUTES {
        return false;
    }

    (attributes & FILE_ATTRIBUTE_HIDDEN) != 0
}

/// Get detailed file metadata
#[tauri::command]
pub async fn get_file_metadata(path: String) -> AppResult<FileMetadata> {
    // Validate path to prevent traversal attacks
    let path_buf = validate_path(&path)?;

    if !path_buf.exists() {
        return Err(AppError::InvalidInput(format!(
            "Path does not exist: {}",
            path
        )));
    }

    match fs::metadata(&path_buf) {
        Ok(meta) => {
            let size = if meta.is_file() {
                Some(meta.len())
            } else {
                None
            };

            let modified_time = meta
                .modified()
                .ok()
                .and_then(|time| time.duration_since(std::time::UNIX_EPOCH).ok())
                .map(|duration| duration.as_secs() as i64);

            let created_time = meta
                .created()
                .ok()
                .and_then(|time| time.duration_since(std::time::UNIX_EPOCH).ok())
                .map(|duration| duration.as_secs() as i64);

            Ok(FileMetadata {
                path: path.clone(),
                size,
                modified_time,
                created_time,
                is_directory: meta.is_dir(),
                is_readonly: meta.permissions().readonly(),
            })
        }
        Err(e) => Err(AppError::InvalidInput(format!(
            "Failed to get file metadata: {}",
            e
        ))),
    }
}

/// Open file with default application
#[tauri::command]
pub async fn open_file_external(path: String) -> AppResult<()> {
    // Validate path to prevent traversal attacks
    let path_buf = validate_path(&path)?;

    if !path_buf.exists() {
        return Err(AppError::InvalidInput(format!(
            "File does not exist: {}",
            path
        )));
    }

    use std::ffi::OsStr;
    use std::mem;
    use std::os::windows::ffi::OsStrExt;
    use std::ptr;
    use winapi::um::shellapi::{ShellExecuteExW, SHELLEXECUTEINFOW};
    use winapi::um::winuser::SW_SHOWNORMAL;

    // Convert path to wide string
    let wide_path: Vec<u16> = OsStr::new(&path_buf)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();

    let wide_open: Vec<u16> = OsStr::new("open")
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();

    // Initialize SHELLEXECUTEINFO structure
    let mut sei: SHELLEXECUTEINFOW = unsafe { mem::zeroed() };
    sei.cbSize = mem::size_of::<SHELLEXECUTEINFOW>() as u32;
    // Suppress UI for the first "open" attempt to avoid double error dialogs
    const SEE_MASK_FLAG_NO_UI: u32 = 0x00000400;
    sei.fMask = SEE_MASK_FLAG_NO_UI;
    sei.hwnd = ptr::null_mut();
    sei.lpVerb = wide_open.as_ptr();
    sei.lpFile = wide_path.as_ptr();
    sei.lpParameters = ptr::null();
    sei.lpDirectory = ptr::null();
    sei.nShow = SW_SHOWNORMAL;

    // Try to execute with "open" verb first
    let result = unsafe { ShellExecuteExW(&mut sei) };

    if result == 0 {
        // Failed with "open", try "openas" to show Open With dialog
        eprintln!("No file association, showing Open With dialog");

        let wide_openas: Vec<u16> = OsStr::new("openas")
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();

        // Enable UI for the fallback attempt so the "Open With" dialog (or error) can be shown
        sei.fMask = 0;
        sei.lpVerb = wide_openas.as_ptr();
        let result_openas = unsafe { ShellExecuteExW(&mut sei) };

        if result_openas == 0 {
            eprintln!("Failed to show Open With dialog");
        }
    }

    Ok(())
}

/// Reveal file in Windows Explorer
#[tauri::command]
pub async fn reveal_in_explorer(path: String) -> AppResult<()> {
    // Validate path to prevent traversal attacks
    let path_buf = validate_path(&path)?;

    if !path_buf.exists() {
        return Err(AppError::InvalidInput(format!(
            "Path does not exist: {}",
            path
        )));
    }

    // Canonicalize path to get absolute path and prevent command injection
    let canonical_path = path_buf
        .canonicalize()
        .map_err(|e| AppError::InvalidInput(format!("Invalid path: {}", e)))?;

    // Use separate arguments to prevent command injection
    // The /select, argument must include the comma with the path
    let select_arg = format!("/select,{}", canonical_path.display());

    match std::process::Command::new("explorer.exe")
        .raw_arg(&select_arg)
        .spawn()
    {
        Ok(_) => Ok(()),
        Err(e) => Err(AppError::InvalidInput(format!(
            "Failed to open Explorer: {}",
            e
        ))),
    }
}
