//! USN Journal Reader
//!
//! Reads USN records from the journal using the unprivileged FSCTL.

use crate::domain::errors::DomainError;
use windows::Win32::Foundation::HANDLE;
use windows::Win32::System::IO::DeviceIoControl;

/// FSCTL code for unprivileged USN Journal reading.
/// CTL_CODE(FILE_DEVICE_FILE_SYSTEM, 234, METHOD_NEITHER, FILE_ANY_ACCESS)
/// This is an undocumented API that requires only FILE_TRAVERSE access.
const FSCTL_READ_UNPRIVILEGED_USN_JOURNAL: u32 = 0x0009_03AB;

/// Reason flags we care about.
const USN_REASON_FILE_CREATE: u32 = 0x0000_0100;
const USN_REASON_FILE_DELETE: u32 = 0x0000_0200;
const USN_REASON_RENAME_OLD_NAME: u32 = 0x0000_1000;
const USN_REASON_RENAME_NEW_NAME: u32 = 0x0000_2000;
const USN_REASON_CLOSE: u32 = 0x8000_0000;

/// READ_USN_JOURNAL_DATA_V0 structure (64 bytes).
#[repr(C)]
#[derive(Clone, Copy)]
struct ReadUsnJournalDataV0 {
    start_usn: i64,
    reason_mask: u32,
    return_only_on_close: u32,
    timeout: u64,
    bytes_to_wait_for: u64,
    usn_journal_id: u64,
}

/// A parsed USN record from the journal.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct RawUsnRecord {
    pub usn: i64,
    pub file_reference_number: u64,
    pub parent_file_reference_number: u64,
    pub reason: u32,
    pub file_name: String,
}

/// Reads all USN records from `start_usn` to the journal's current position.
///
/// Returns `(final_usn, records)` where `final_usn` is the USN to save for the next read.
/// Only records with rename or delete reasons are returned.
pub fn read_journal_records(
    handle: HANDLE,
    journal_id: u64,
    start_usn: i64,
) -> Result<(i64, Vec<RawUsnRecord>), DomainError> {
    let mut all_records = Vec::new();
    let mut current_usn = start_usn;

    loop {
        let (next_usn, batch) = read_journal_batch(handle, journal_id, current_usn)?;

        if batch.is_empty() || next_usn == current_usn {
            return Ok((next_usn, all_records));
        }

        all_records.extend(batch);
        current_usn = next_usn;
    }
}

/// Reads a single batch of USN records.
fn read_journal_batch(
    handle: HANDLE,
    journal_id: u64,
    start_usn: i64,
) -> Result<(i64, Vec<RawUsnRecord>), DomainError> {
    let input = ReadUsnJournalDataV0 {
        start_usn,
        reason_mask: USN_REASON_FILE_CREATE
            | USN_REASON_RENAME_OLD_NAME
            | USN_REASON_RENAME_NEW_NAME
            | USN_REASON_FILE_DELETE
            | USN_REASON_CLOSE,
        return_only_on_close: 0,
        timeout: 0,
        bytes_to_wait_for: 0,
        usn_journal_id: journal_id,
    };

    let mut buffer = vec![0u8; 64 * 1024]; // 64KB buffer
    let mut bytes_returned = 0u32;

    let result = unsafe {
        DeviceIoControl(
            handle,
            FSCTL_READ_UNPRIVILEGED_USN_JOURNAL,
            Some(&input as *const _ as *const _),
            std::mem::size_of::<ReadUsnJournalDataV0>() as u32,
            Some(buffer.as_mut_ptr() as *mut _),
            buffer.len() as u32,
            Some(&mut bytes_returned),
            None,
        )
    };

    if let Err(e) = result {
        let code = e.code().0 as u32;
        // HRESULT for ERROR_HANDLE_EOF = 0x80070026
        if code == 0x8007_0026 {
            return Ok((start_usn, Vec::new()));
        }
        // HRESULT for ERROR_JOURNAL_NOT_ACTIVE = 0x8007049B
        if code == 0x8007_049B {
            return Err(DomainError::UsnJournalError(
                "Journal not active".to_string(),
            ));
        }
        return Err(DomainError::UsnJournalError(format!(
            "Failed to read USN Journal: code=0x{:08X}, {}",
            code, e
        )));
    }

    let returned = bytes_returned as usize;
    if returned < 8 {
        return Ok((start_usn, Vec::new()));
    }

    // First 8 bytes: next USN value
    let next_usn = i64::from_le_bytes(buffer[0..8].try_into().unwrap());

    // Parse USN record entries starting at offset 8.
    // Supports both V2 (64-bit FRN) and V3 (128-bit FRN) records.
    //
    // USN_RECORD_V2 layout:                 USN_RECORD_V3 layout:
    //  0..4   RecordLength (u32)             0..4   RecordLength (u32)
    //  4..6   MajorVersion (u16)             4..6   MajorVersion (u16)
    //  6..8   MinorVersion (u16)             6..8   MinorVersion (u16)
    //  8..16  FileReferenceNumber (u64)      8..24  FileReferenceNumber (FILE_ID_128)
    // 16..24  ParentFileRefNum (u64)        24..40  ParentFileRefNum (FILE_ID_128)
    // 24..32  Usn (i64)                     40..48  Usn (i64)
    // 32..40  TimeStamp (i64)               48..56  TimeStamp (i64)
    // 40..44  Reason (u32)                  56..60  Reason (u32)
    // 44..48  SourceInfo (u32)              60..64  SourceInfo (u32)
    // 48..52  SecurityId (u32)              64..68  SecurityId (u32)
    // 52..56  FileAttributes (u32)          68..72  FileAttributes (u32)
    // 56..58  FileNameLength (u16)          72..74  FileNameLength (u16)
    // 58..60  FileNameOffset (u16)          74..76  FileNameOffset (u16)
    // 60..    FileName (UTF-16LE)           76..    FileName (UTF-16LE)

    let mut records = Vec::new();
    let mut offset = 8usize;

    while offset + 6 <= returned {
        let record_length =
            u32::from_le_bytes(buffer[offset..offset + 4].try_into().unwrap()) as usize;
        let major_version =
            u16::from_le_bytes(buffer[offset + 4..offset + 6].try_into().unwrap());

        let is_v3 = major_version >= 3;
        let min_size: usize = if is_v3 { 76 } else { 64 };

        if record_length < min_size || offset + record_length > returned {
            break;
        }

        // Parse fields at version-dependent offsets.
        // For V3, FILE_ID_128 is 16 bytes; we take the lower 8 bytes (NTFS FRN).
        let (frn, parent_frn, usn, reason, file_name_length, file_name_offset) = if is_v3 {
            (
                u64::from_le_bytes(buffer[offset + 8..offset + 16].try_into().unwrap()),
                u64::from_le_bytes(buffer[offset + 24..offset + 32].try_into().unwrap()),
                i64::from_le_bytes(buffer[offset + 40..offset + 48].try_into().unwrap()),
                u32::from_le_bytes(buffer[offset + 56..offset + 60].try_into().unwrap()),
                u16::from_le_bytes(buffer[offset + 72..offset + 74].try_into().unwrap()) as usize,
                u16::from_le_bytes(buffer[offset + 74..offset + 76].try_into().unwrap()) as usize,
            )
        } else {
            (
                u64::from_le_bytes(buffer[offset + 8..offset + 16].try_into().unwrap()),
                u64::from_le_bytes(buffer[offset + 16..offset + 24].try_into().unwrap()),
                i64::from_le_bytes(buffer[offset + 24..offset + 32].try_into().unwrap()),
                u32::from_le_bytes(buffer[offset + 40..offset + 44].try_into().unwrap()),
                u16::from_le_bytes(buffer[offset + 56..offset + 58].try_into().unwrap()) as usize,
                u16::from_le_bytes(buffer[offset + 58..offset + 60].try_into().unwrap()) as usize,
            )
        };

        // Parse file name (UTF-16LE)
        let name_start = offset + file_name_offset;
        let name_end = name_start + file_name_length;

        let file_name = if name_end <= returned && file_name_length >= 2 {
            let u16_slice: Vec<u16> = buffer[name_start..name_end]
                .chunks_exact(2)
                .map(|c| u16::from_le_bytes([c[0], c[1]]))
                .collect();
            String::from_utf16_lossy(&u16_slice)
        } else {
            String::new()
        };

        // Only keep records with reasons we care about
        if reason
            & (USN_REASON_FILE_CREATE | USN_REASON_RENAME_NEW_NAME | USN_REASON_FILE_DELETE)
            != 0
        {
            records.push(RawUsnRecord {
                usn,
                file_reference_number: frn,
                parent_file_reference_number: parent_frn,
                reason,
                file_name,
            });
        }

        offset += record_length;
    }

    Ok((next_usn, records))
}
