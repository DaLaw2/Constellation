//! File Reference Number Retrieval
//!
//! Gets the NTFS File Reference Number (FRN) for a given file path.

use crate::domain::errors::DomainError;
use windows::core::HSTRING;
use windows::Win32::Foundation::CloseHandle;
use windows::Win32::Storage::FileSystem::{
    CreateFileW, GetFileInformationByHandle, BY_HANDLE_FILE_INFORMATION,
    FILE_FLAG_BACKUP_SEMANTICS, FILE_SHARE_DELETE, FILE_SHARE_READ, FILE_SHARE_WRITE,
    OPEN_EXISTING,
};

/// Gets the NTFS File Reference Number for a file path.
///
/// Returns `Ok(None)` if the file does not exist.
pub fn get_file_reference_number(path: &str) -> Result<Option<u64>, DomainError> {
    let path_wide = HSTRING::from(path);

    // Open with minimal access — dwDesiredAccess = 0
    let handle = unsafe {
        CreateFileW(
            &path_wide,
            0, // No access needed
            FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE,
            None,
            OPEN_EXISTING,
            FILE_FLAG_BACKUP_SEMANTICS, // Required for directories
            None,
        )
    };

    let handle = match handle {
        Ok(h) => h,
        Err(_) => return Ok(None), // File doesn't exist
    };

    let mut info = BY_HANDLE_FILE_INFORMATION::default();
    let ok = unsafe { GetFileInformationByHandle(handle, &mut info) };

    unsafe {
        let _ = CloseHandle(handle);
    }

    if ok.is_err() {
        return Ok(None);
    }

    let frn = (info.nFileIndexHigh as u64) << 32 | info.nFileIndexLow as u64;
    Ok(Some(frn))
}
