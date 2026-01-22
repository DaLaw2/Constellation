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

### Dependencies

```toml
[target.'cfg(windows)'.dependencies]
usn-journal-rs = "0.1"
```

### Implementation

```rust
use usn_journal_rs::{UsnJournal, UsnRecord, UsnReason};

async fn monitor_file_changes(volume: &str) -> Result<(), Error> {
    // Requires administrator privileges
    let journal = UsnJournal::open(volume)?;

    // Read from last known USN
    let records = journal.read_from(last_usn)?;

    for record in records {
        match record.reason {
            UsnReason::FileCreate => handle_create(&record),
            UsnReason::FileDelete => handle_delete(&record),
            UsnReason::DataOverwrite | UsnReason::DataExtend => handle_modify(&record),
            UsnReason::RenameNewName => handle_rename(&record),
            _ => {}
        }
    }

    Ok(())
}
```

### Alternative: Cross-Platform File Watching

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
- [usn-journal-rs](https://crates.io/crates/usn-journal-rs)
- [notify crate](https://docs.rs/notify/)
