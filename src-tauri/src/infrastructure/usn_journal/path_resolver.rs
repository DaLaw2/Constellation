//! Path Resolver
//!
//! Resolves a File Reference Number (FRN) to its current full path using OpenFileById.

use crate::domain::errors::DomainError;
use windows::Win32::Foundation::{CloseHandle, HANDLE};
use windows::Win32::Storage::FileSystem::{
    GetFinalPathNameByHandleW, OpenFileById, FILE_FLAG_BACKUP_SEMANTICS, FILE_ID_DESCRIPTOR,
    FILE_ID_TYPE, FILE_SHARE_DELETE, FILE_SHARE_READ, FILE_SHARE_WRITE, VOLUME_NAME_DOS,
};

/// Resolves a File Reference Number to its current full path.
///
/// Returns `Ok(None)` if the file no longer exists (FRN invalid / MFT entry recycled).
pub fn resolve_path_by_frn(volume_handle: HANDLE, frn: u64) -> Result<Option<String>, DomainError> {
    // Build FILE_ID_DESCRIPTOR for OpenFileById
    let mut descriptor: FILE_ID_DESCRIPTOR = unsafe { std::mem::zeroed() };
    descriptor.dwSize = std::mem::size_of::<FILE_ID_DESCRIPTOR>() as u32;
    descriptor.Type = FILE_ID_TYPE(0); // FileIdType
    descriptor.Anonymous.FileId = frn as i64;

    // Open file by ID — dwDesiredAccess = 0 (no data access needed)
    let file_handle = unsafe {
        OpenFileById(
            volume_handle,
            &descriptor,
            0, // No access needed, just want the handle for path resolution
            FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE,
            None,
            FILE_FLAG_BACKUP_SEMANTICS, // Required for directories
        )
    };

    let file_handle = match file_handle {
        Ok(h) => h,
        Err(_) => return Ok(None), // File doesn't exist anymore
    };

    // Get the final path name
    let result = get_path_from_handle(file_handle);

    // Always close the handle
    unsafe {
        let _ = CloseHandle(file_handle);
    }

    result
}

/// Gets the full path from an open file handle.
fn get_path_from_handle(handle: HANDLE) -> Result<Option<String>, DomainError> {
    let mut buf = [0u16; 1024];

    let len = unsafe { GetFinalPathNameByHandleW(handle, &mut buf, VOLUME_NAME_DOS) };

    if len == 0 || len as usize >= buf.len() {
        return Ok(None);
    }

    let path = String::from_utf16_lossy(&buf[..len as usize]);

    // Remove the \\?\ prefix that GetFinalPathNameByHandle adds
    let clean = path.strip_prefix("\\\\?\\").unwrap_or(&path);

    Ok(Some(clean.to_string()))
}
