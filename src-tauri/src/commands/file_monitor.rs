//! File Monitor Commands
//!
//! Tauri commands for on-demand USN Journal file index refresh.

use crate::application::dto::{DriveUsnStatusDto, RefreshResultDto};
use crate::error::{AppError, AppResult};
use crate::state::AppState;
use tauri::State;

/// Refreshes the file index for the specified drives using USN Journal.
#[tauri::command]
pub async fn refresh_file_index(
    drives: Vec<String>,
    state: State<'_, AppState>,
) -> AppResult<RefreshResultDto> {
    let letters: Vec<char> = drives.iter().filter_map(|d| d.chars().next()).collect();
    state
        .usn_refresh_service
        .refresh(&letters)
        .await
        .map_err(|e| AppError::UsnJournal(e.to_string()))
}

/// Checks whether a drive supports USN Journal (is NTFS).
#[tauri::command]
pub async fn check_usn_support(drive: String) -> AppResult<bool> {
    let letter = drive
        .chars()
        .next()
        .ok_or_else(|| AppError::InvalidInput("Empty drive".into()))?;

    #[cfg(windows)]
    {
        crate::infrastructure::usn_journal::is_ntfs(letter)
            .map_err(|e| AppError::UsnJournal(e.to_string()))
    }

    #[cfg(not(windows))]
    {
        let _ = letter;
        Ok(false)
    }
}

/// Gets USN Journal status for all NTFS drives.
#[tauri::command]
pub async fn get_usn_drive_status(state: State<'_, AppState>) -> AppResult<Vec<DriveUsnStatusDto>> {
    state
        .usn_refresh_service
        .get_drive_status()
        .await
        .map_err(|e| AppError::UsnJournal(e.to_string()))
}

/// Enables USN Journal on a drive (requires admin — triggers UAC dialog).
#[tauri::command]
pub async fn enable_usn_journal(drive: String) -> AppResult<()> {
    let letter = drive
        .chars()
        .next()
        .ok_or_else(|| AppError::InvalidInput("Empty drive".into()))?;

    #[cfg(windows)]
    {
        use windows::core::PCWSTR;
        use windows::Win32::UI::Shell::ShellExecuteW;
        use windows::Win32::UI::WindowsAndMessaging::SW_HIDE;

        let verb: Vec<u16> = "runas\0".encode_utf16().collect();
        let file: Vec<u16> = "cmd.exe\0".encode_utf16().collect();
        let args = format!(
            "/c fsutil usn createjournal m=33554432 a=4194304 {}:\0",
            letter
        );
        let args_wide: Vec<u16> = args.encode_utf16().collect();

        let result = unsafe {
            ShellExecuteW(
                None,
                PCWSTR(verb.as_ptr()),
                PCWSTR(file.as_ptr()),
                PCWSTR(args_wide.as_ptr()),
                None,
                SW_HIDE,
            )
        };

        // ShellExecuteW returns > 32 on success
        if result.0 as usize <= 32 {
            return Err(AppError::UsnJournal(
                "Failed to launch elevated process. User may have cancelled UAC.".into(),
            ));
        }
        Ok(())
    }

    #[cfg(not(windows))]
    {
        let _ = letter;
        Err(AppError::UsnJournal(
            "USN Journal is only supported on Windows".into(),
        ))
    }
}
