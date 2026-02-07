//! Volume Handle and USN Journal Query
//!
//! RAII wrapper for NTFS volume handles and journal state queries.

use crate::domain::errors::DomainError;
use std::mem::MaybeUninit;
use windows::core::HSTRING;
use windows::Win32::Foundation::{CloseHandle, HANDLE};
use windows::Win32::Storage::FileSystem::{
    CreateFileW, GetVolumeInformationW, FILE_SHARE_READ, FILE_SHARE_WRITE, OPEN_EXISTING,
};
use windows::Win32::System::Ioctl::{FSCTL_QUERY_USN_JOURNAL, USN_JOURNAL_DATA_V0};
use windows::Win32::System::IO::DeviceIoControl;

/// Minimum access right for opening a volume handle without admin privileges.
const FILE_TRAVERSE: u32 = 0x20;

/// RAII wrapper for an NTFS volume handle.
pub struct VolumeHandle {
    handle: HANDLE,
    pub drive_letter: char,
}

// SAFETY: Windows HANDLEs are safe to send and share across threads.
// The underlying kernel object is reference-counted and thread-safe.
unsafe impl Send for VolumeHandle {}
unsafe impl Sync for VolumeHandle {}

impl Drop for VolumeHandle {
    fn drop(&mut self) {
        unsafe {
            let _ = CloseHandle(self.handle);
        }
    }
}

/// USN Journal metadata for a volume.
pub struct UsnJournalInfo {
    pub journal_id: u64,
    pub first_usn: i64,
    pub next_usn: i64,
}

impl VolumeHandle {
    /// Opens a volume handle with FILE_TRAVERSE access (no admin required).
    pub fn open(drive_letter: char) -> Result<Self, DomainError> {
        let drive = drive_letter.to_ascii_uppercase();
        let path = format!("\\\\.\\{}:", drive);
        let path_wide = HSTRING::from(&path);

        let handle = unsafe {
            CreateFileW(
                &path_wide,
                FILE_TRAVERSE,
                FILE_SHARE_READ | FILE_SHARE_WRITE,
                None,
                OPEN_EXISTING,
                Default::default(),
                None,
            )
        }
        .map_err(|e| {
            DomainError::UsnJournalError(format!("Failed to open volume {}: {}", drive, e))
        })?;

        Ok(Self {
            handle,
            drive_letter: drive,
        })
    }

    /// Queries the USN Journal state for this volume.
    pub fn query_journal(&self) -> Result<UsnJournalInfo, DomainError> {
        let mut journal_data = MaybeUninit::<USN_JOURNAL_DATA_V0>::uninit();
        let mut bytes_returned = 0u32;

        let result = unsafe {
            DeviceIoControl(
                self.handle,
                FSCTL_QUERY_USN_JOURNAL,
                None,
                0,
                Some(journal_data.as_mut_ptr() as *mut _),
                std::mem::size_of::<USN_JOURNAL_DATA_V0>() as u32,
                Some(&mut bytes_returned),
                None,
            )
        };

        if let Err(e) = result {
            let code = e.code().0 as u32;
            // HRESULT for ERROR_JOURNAL_NOT_ACTIVE (1179) = 0x8007049B
            // HRESULT for ERROR_JOURNAL_DELETE_IN_PROGRESS (1178) = 0x8007049A
            if code == 0x8007_049B {
                return Err(DomainError::UsnJournalError(format!(
                    "Journal not active on {}:",
                    self.drive_letter
                )));
            }
            if code == 0x8007_049A {
                return Err(DomainError::UsnJournalError(format!(
                    "Journal deletion in progress on {}:",
                    self.drive_letter
                )));
            }
            return Err(DomainError::UsnJournalError(format!(
                "Failed to query USN Journal on {}: {}",
                self.drive_letter, e
            )));
        }

        let data = unsafe { journal_data.assume_init() };

        Ok(UsnJournalInfo {
            journal_id: data.UsnJournalID,
            first_usn: data.FirstUsn,
            next_usn: data.NextUsn,
        })
    }

    /// Returns the raw Win32 HANDLE for use by reader/resolver.
    pub fn raw_handle(&self) -> HANDLE {
        self.handle
    }
}

/// Checks whether the given drive letter hosts an NTFS (or ReFS) file system.
pub fn is_ntfs(drive_letter: char) -> Result<bool, DomainError> {
    let drive = drive_letter.to_ascii_uppercase();
    let root = format!("{}:\\", drive);
    let root_wide = HSTRING::from(&root);

    let mut fs_name_buf = [0u16; 64];
    let ok = unsafe {
        GetVolumeInformationW(&root_wide, None, None, None, None, Some(&mut fs_name_buf))
    };

    if ok.is_err() {
        // Drive may not exist or be inaccessible
        return Ok(false);
    }

    let len = fs_name_buf
        .iter()
        .position(|&c| c == 0)
        .unwrap_or(fs_name_buf.len());
    let fs_name = String::from_utf16_lossy(&fs_name_buf[..len]);

    Ok(fs_name == "NTFS" || fs_name == "ReFS")
}
