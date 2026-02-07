# Windows API Integrations

## Thumbnail Extraction (IShellItemImageFactory)

### Dependencies

```toml
[target.'cfg(windows)'.dependencies]
windows = { version = "0.58", features = [
    "Win32_UI_Shell",
    "Win32_UI_Shell_Common",
    "Win32_Graphics_Gdi",
    "Win32_Foundation",
    "Win32_System_Com",
]}
```

### Implementation

```rust
use windows::{
    core::*,
    Win32::UI::Shell::*,
    Win32::Graphics::Gdi::*,
    Win32::Foundation::*,
    Win32::System::Com::*,
};

pub fn get_thumbnail(path: &str, width: i32, height: i32) -> Result<HBITMAP> {
    unsafe {
        // Initialize COM (once per thread)
        CoInitializeEx(None, COINIT_APARTMENTTHREADED)?;

        // Create shell item from path
        let path_wide: Vec<u16> = path.encode_utf16().chain(std::iter::once(0)).collect();
        let shell_item: IShellItem = SHCreateItemFromParsingName(
            PCWSTR(path_wide.as_ptr()),
            None,
        )?;

        // Get image factory interface
        let factory: IShellItemImageFactory = shell_item.cast()?;

        // Get thumbnail
        let size = SIZE { cx: width, cy: height };
        let hbitmap = factory.GetImage(size, SIIGBF_THUMBNAILONLY)?;

        Ok(hbitmap)
    }
}
```

### Important Flags (SIIGBF)

| Flag | Description |
|------|-------------|
| `SIIGBF_THUMBNAILONLY` | Only thumbnail, fail if unavailable |
| `SIIGBF_ICONONLY` | Only icon, never thumbnail |
| `SIIGBF_MEMORYONLY` | Only cached in memory (UI thread safe) |
| `SIIGBF_INCACHEONLY` | Only cached items (Windows 8+) |

**Threading**: Never call `GetImage` without `SIIGBF_MEMORYONLY` on UI thread. Use background thread.

---

## NTFS USN Journal Monitoring

### Overview

USN (Update Sequence Number) Journal 是 NTFS 提供的磁碟區級變更日誌，記錄所有檔案/目錄的建立、修改、重新命名、移動、刪除等操作。

- 僅支援 **NTFS** 和 **ReFS**，不支援 FAT32 / exFAT
- 需要**管理員權限**才能讀取
- Windows Vista 以後預設啟用

### USN Record 結構

#### USN_RECORD_V2 (NTFS, 64-bit FRN)

| 欄位 | 型別 | 說明 |
|------|------|------|
| `RecordLength` | `DWORD` | 記錄總長度 |
| `MajorVersion` | `WORD` | = 2 |
| `MinorVersion` | `WORD` | = 0 |
| `FileReferenceNumber` | `DWORDLONG` | 檔案的 MFT 參考編號 (FRN)，64-bit |
| `ParentFileReferenceNumber` | `DWORDLONG` | 所在目錄的 FRN，64-bit |
| `Usn` | `USN` | 此記錄的 USN 位置 |
| `TimeStamp` | `LARGE_INTEGER` | 記錄時間 |
| `Reason` | `DWORD` | 變更原因 flags (bitwise OR) |
| `SourceInfo` | `DWORD` | 來源資訊 |
| `SecurityId` | `DWORD` | 安全識別碼 |
| `FileAttributes` | `DWORD` | 檔案屬性 |
| `FileNameLength` | `WORD` | 檔名長度 (bytes) |
| `FileNameOffset` | `WORD` | 檔名偏移 |
| `FileName` | `WCHAR[]` | 檔名（僅檔名，不含路徑） |

#### USN_RECORD_V3 (ReFS, 128-bit FRN)

與 V2 相同結構，但 `FileReferenceNumber` 和 `ParentFileReferenceNumber` 為 128-bit。

### FRN (File Reference Number) 特性

- NTFS 的 FRN 基於 MFT index，**不受重新命名/移動/磁碟重組影響**
- FRN = MFT index (低 48 bits) + sequence number (高 16 bits)
- 檔案刪除後 MFT entry 可被重用，此時 sequence number 會遞增
- 同一磁碟區內的重新命名/移動不會改變 FRN
- **跨磁碟區移動 = 刪除 + 建立，會產生新 FRN**

### 重要的 Reason Flags

| Flag | 值 | 說明 |
|------|---|------|
| `USN_REASON_RENAME_OLD_NAME` | `0x00001000` | 重新命名/移動 — 舊名稱記錄 |
| `USN_REASON_RENAME_NEW_NAME` | `0x00002000` | 重新命名/移動 — 新名稱記錄 |
| `USN_REASON_FILE_DELETE` | `0x00000200` | 檔案刪除 |
| `USN_REASON_FILE_CREATE` | `0x00000100` | 檔案建立 |
| `USN_REASON_CLOSE` | `0x80000000` | 檔案關閉（summary record） |
| `USN_REASON_HARD_LINK_CHANGE` | `0x00010000` | 硬連結建立/刪除 |
| `USN_REASON_DATA_OVERWRITE` | `0x00000001` | 資料覆寫 |
| `USN_REASON_DATA_EXTEND` | `0x00000002` | 資料擴展 |
| `USN_REASON_DATA_TRUNCATION` | `0x00000004` | 資料截斷 |
| `USN_REASON_BASIC_INFO_CHANGE` | `0x00008000` | 基本資訊（時間戳等）變更 |

### 重新命名/移動的 USN 行為

重新命名或移動會產生**兩筆記錄**：

1. `USN_REASON_RENAME_OLD_NAME`: `ParentFileReferenceNumber` = 舊目錄 FRN, `FileName` = 舊檔名
2. `USN_REASON_RENAME_NEW_NAME`: `ParentFileReferenceNumber` = 新目錄 FRN, `FileName` = 新檔名

兩筆記錄的 `FileReferenceNumber` 相同（同一個檔案）。

### Record 合併機制

- 同一檔案的多次相同操作（如多次寫入）只會產生一個 reason flag
- `USN_REASON_CLOSE` 記錄是 open→close 期間所有變更的 summary（bitwise OR）
- CLOSE 記錄不保留操作順序

### Journal 溢出與過期偵測

NTFS 大約每 5 秒進行 checkpoint，超出 `MaximumSize` 時自動刪除舊記錄。

```rust
// 完整的 stale state 偵測邏輯
// 儲存的狀態: { last_usn, journal_id }

// 1. 查詢目前 journal 狀態
let current = fsctl_query_usn_journal(volume_handle)?;

// 2. Journal 被刪除並重建 → 全量重新掃描
if current.UsnJournalID != stored.journal_id {
    return full_rescan();
}

// 3. 舊記錄被回收，遺漏事件 → 全量重新掃描
if stored.last_usn < current.FirstUsn {
    return full_rescan();
}

// 4. 正常：可以從 stored.last_usn 增量讀取
// 5. ERROR_JOURNAL_NOT_ACTIVE → 該磁碟區無 journal
// 6. ERROR_JOURNAL_DELETE_IN_PROGRESS → 等待重試或 fallback
```

### FSCTL_READ_USN_JOURNAL 分頁讀取

```rust
// Output buffer 結構: [NextUsn: u64][USN_RECORD_V2...][USN_RECORD_V2...]
// 1. 前 8 bytes 是下一次讀取的起始 USN
// 2. 之後是零或多筆變長的 USN_RECORD_V2
// 3. 用 RecordLength 逐筆遍歷
// 4. 下一批: StartUsn = buffer 的前 8 bytes
// 5. 當 dwBytesReturned == sizeof(USN) (只有 NextUsn 沒有記錄) → 讀取完畢
```

### 磁碟區格式偵測

```rust
// 用 GetVolumeInformationW 取得檔案系統名稱
// 回傳 "NTFS"、"ReFS"、"FAT32"、"exFAT" 等
// 非 NTFS/ReFS → 不啟用 USN 追蹤
```

### 硬連結 (Hard Links) 注意事項

- 同一檔案的多個硬連結共享同一個 FRN
- USN Journal 對硬連結變更只回報**一個**檔名
- 要精確追蹤硬連結重新命名，需設定 `ReturnOnlyOnClose = 0`（更多記錄，更大開銷）
- 本專案暫不特別處理硬連結

### 路徑重建

USN record 只有 `FileName`（不含路徑）和 `ParentFileReferenceNumber`。重建完整路徑的方法：

#### 方案 A：即時查詢（OnTheFlyResolver）

```rust
// 1. 用 FRN 開啟 parent 目錄
let handle = OpenFileById(volume_handle, &file_id_descriptor, ...)?;
// 2. 取得完整路徑
let path = GetFinalPathNameByHandleW(handle, &mut buffer, VOLUME_NAME_DOS);
// 3. 組合: parent_path + "\" + file_name
```

- 優點：簡單直接
- 缺點：每筆記錄需要 I/O；parent 被刪除時無法查詢

#### 方案 B：快取對應表（CachedResolver）

```rust
// 掃描 USN 時建立快取
let cache: HashMap<u64, (String, u64)> = HashMap::new(); // FRN → (name, parent_FRN)
// 用 parent chain 重建路徑
// chain 斷裂時 fallback 到方案 A
```

- 優點：減少 I/O，處理連鎖重新命名
- 缺點：較複雜，記憶體開銷

### Dependencies

```toml
[target.'cfg(windows)'.dependencies]
# 選項 1: 使用現成 crate
usn-journal-rs = "0.1"

# 選項 2: 直接用 windows crate（更多控制）
windows = { version = "0.58", features = [
    "Win32_Foundation",
    "Win32_Storage_FileSystem",
    "Win32_System_Ioctl",
    "Win32_System_IO",
    "Win32_Security",
]}
```

### usn-journal-rs 使用範例

```rust
use usn_journal_rs::{volume::Volume, journal::UsnJournal};

let volume = Volume::from_drive_letter('C').unwrap();
let journal = UsnJournal::new(&volume);

for result in journal.iter().unwrap().take(10) {
    match result {
        Ok(entry) => println!("USN entry: {entry:?}"),
        Err(e) => eprintln!("Error: {e}"),
    }
}
```

### 直接使用 Windows API

```rust
use windows::Win32::{
    Foundation::*,
    Storage::FileSystem::*,
    System::Ioctl::*,
    System::IO::DeviceIoControl,
};

// 開啟磁碟區
let volume_path: Vec<u16> = "\\\\.\\C:".encode_utf16().chain(std::iter::once(0)).collect();
let volume_handle = unsafe {
    CreateFileW(
        PCWSTR(volume_path.as_ptr()),
        FILE_GENERIC_READ.0,
        FILE_SHARE_READ | FILE_SHARE_WRITE,
        None,
        OPEN_EXISTING,
        FILE_FLAGS_AND_ATTRIBUTES(0),
        None,
    )?
};

// 查詢 USN Journal 資訊
let mut journal_data = USN_JOURNAL_DATA_V2::default();
let mut bytes_returned = 0u32;
unsafe {
    DeviceIoControl(
        volume_handle,
        FSCTL_QUERY_USN_JOURNAL,
        None, 0,
        Some(&mut journal_data as *mut _ as *mut _),
        std::mem::size_of::<USN_JOURNAL_DATA_V2>() as u32,
        Some(&mut bytes_returned),
        None,
    )?;
}

// 從指定 USN 位置讀取記錄
let read_data = READ_USN_JOURNAL_DATA_V0 {
    StartUsn: last_usn,
    ReasonMask: USN_REASON_RENAME_OLD_NAME | USN_REASON_RENAME_NEW_NAME
        | USN_REASON_FILE_DELETE | USN_REASON_CLOSE,
    ReturnOnlyOnClose: 0, // 0 = 取得所有記錄（含 rename old/new）
    Timeout: 0,
    BytesToWaitFor: 0,
    UsnJournalID: journal_data.UsnJournalID,
};
```

### OpenFileById 路徑解析 (方案 A)

```rust
use windows::Win32::Storage::FileSystem::*;

// 建立 FILE_ID_DESCRIPTOR
let mut file_id_desc = FILE_ID_DESCRIPTOR {
    dwSize: std::mem::size_of::<FILE_ID_DESCRIPTOR>() as u32,
    Type: FileIdType, // 0 = 64-bit FRN
    ..Default::default()
};
file_id_desc.Anonymous.FileId = parent_frn as i64;

// 用 FRN 開啟檔案
// 注意：FILE_FLAG_BACKUP_SEMANTICS 對目錄是必須的
let handle = unsafe {
    OpenFileById(
        volume_handle,
        &file_id_desc,
        0, // 只需解析路徑不需要讀取權限
        FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE,
        None,
        FILE_FLAG_BACKUP_SEMANTICS, // 必須！否則開啟目錄會失敗
    )?
};

// 取得完整路徑
let mut buffer = [0u16; 512];
let len = unsafe {
    GetFinalPathNameByHandleW(
        handle,
        &mut buffer,
        VOLUME_NAME_DOS, // 回傳 \\?\C:\path\to\dir
    )
};
let path = String::from_utf16_lossy(&buffer[..len as usize]);
// 去掉 \\?\ 前綴
let clean_path = path.strip_prefix("\\\\?\\").unwrap_or(&path);
```

### GetFinalPathNameByHandleW Flags

| Flag | 值 | 說明 |
|------|---|------|
| `VOLUME_NAME_DOS` | `0x0` | 回傳磁碟代號路徑 (`\\?\C:\...`) |
| `VOLUME_NAME_GUID` | `0x1` | 回傳 GUID 路徑 (`\\?\Volume{...}\...`) |
| `VOLUME_NAME_NT` | `0x2` | 回傳 NT 裝置路徑 (`\Device\HarddiskVolume1\...`) |
| `VOLUME_NAME_NONE` | `0x4` | 不含磁碟資訊 |
| `FILE_NAME_NORMALIZED` | `0x0` | 正規化路徑（預設） |
| `FILE_NAME_OPENED` | `0x8` | 開啟時的路徑（未正規化） |

---

## Admin 權限提升

### Tauri build.rs Manifest 方式

在 `build.rs` 中設定 Windows manifest，讓 exe 永遠以管理員啟動：

```rust
// build.rs
fn main() {
    let mut windows = tauri_build::WindowsAttributes::new();
    windows = windows.app_manifest(r#"
        <assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
            <trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
                <security>
                    <requestedPrivileges>
                        <requestedExecutionLevel level="requireAdministrator" uiAccess="false" />
                    </requestedPrivileges>
                </security>
            </trustInfo>
        </assembly>
    "#);
    tauri_build::try_build(
        tauri_build::Attributes::new().windows_attributes(windows)
    ).expect("failed to build");
}
```

### ShellExecuteEx 動態提權方式

參考 `/mnt/d/Code/Rust/elevate` 專案：

```rust
use privilege::user::privileged;

fn main() {
    if privileged() {
        // 已是管理員，正常啟動
        run_app();
    } else {
        // 用 ShellExecuteExW + "runas" 重新啟動
        windows::elevate().unwrap();
    }
}
```

### WebView2 + Admin 已知問題

Windows 11 啟用 Administrator Protection 時，WebView2 可能無法存取正確的 AppData 路徑。

解決方式：透過 `GetTokenInformation` + `SHGetKnownFolderPath` 計算正確路徑，設定 `WEBVIEW2_USER_DATA_FOLDER` 環境變數。

---

## Cross-Platform File Watching (Alternative)

```toml
[dependencies]
notify = "7"
notify-debouncer-full = "0.4"
```

```rust
use notify::{Event, RecursiveMode, Watcher};
use std::sync::mpsc;

let (tx, rx) = mpsc::channel();
let mut watcher = notify::recommended_watcher(tx)?;
watcher.watch(Path::new("./watched"), RecursiveMode::Recursive)?;

for event in rx {
    match event {
        Ok(e) => println!("Event: {:?}", e),
        Err(e) => println!("Error: {:?}", e),
    }
}
```

## References

- [IShellItemImageFactory](https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/UI/Shell/struct.IShellItemImageFactory.html)
- [USN_RECORD_V2](https://learn.microsoft.com/en-us/windows/win32/api/winioctl/ns-winioctl-usn_record_v2)
- [FSCTL_READ_USN_JOURNAL](https://learn.microsoft.com/en-us/windows/win32/api/winioctl/ni-winioctl-fsctl_read_usn_journal)
- [Change Journal Records](https://learn.microsoft.com/en-us/windows/win32/fileio/change-journal-records)
- [OpenFileById](https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/Storage/FileSystem/fn.OpenFileById.html)
- [GetFinalPathNameByHandleW](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getfinalpathnamebyhandlew)
- [usn-journal-rs](https://crates.io/crates/usn-journal-rs)
- [notify crate](https://docs.rs/notify/)
- [Tauri Admin Elevation Discussion](https://github.com/tauri-apps/tauri/discussions/4201)
