# Tauri 2.x Architecture Patterns

## State Management

```rust
// AppState with dependency injection
struct AppState {
    db: Arc<Mutex<Database>>,
    config: Arc<RwLock<AppConfig>>,
}

#[tauri::command]
async fn get_files(
    state: tauri::State<'_, AppState>,
    query: String,
) -> Result<Vec<FileDto>, String> {
    let db = state.db.lock().await;
    db.search_files(&query).await.map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .manage(AppState { /* ... */ })
        .invoke_handler(tauri::generate_handler![get_files])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

## IPC Communication

### Commands (Frontend → Backend)

```typescript
// Frontend
import { invoke } from '@tauri-apps/api/core'

const files = await invoke<FileItem[]>('search_files', { query: 'test' })
```

### Events (Backend → Frontend)

```rust
// Backend
use tauri::Emitter;

app.emit("file-indexed", &indexed_file)?;
```

```typescript
// Frontend
import { listen } from '@tauri-apps/api/event'

const unlisten = await listen<FileItem>('file-indexed', (event) => {
    console.log('File indexed:', event.payload)
})
```

## Background Mode

### Prevent Exit on Window Close

```rust
use tauri::RunEvent;

tauri::Builder::default()
    .build(tauri::generate_context!())?
    .run(|_app_handle, event| {
        if let RunEvent::ExitRequested { api, .. } = event {
            api.prevent_exit();
        }
    });
```

### Hide Window Instead of Close

```rust
main_window.on_window_event(move |event| {
    if let tauri::WindowEvent::CloseRequested { api, .. } = event {
        api.prevent_close();
        window.hide().unwrap();
    }
});
```

## Useful Plugins

| Plugin | Purpose |
|--------|---------|
| `tauri-plugin-single-instance` | Prevent multiple app instances |
| `tauri-plugin-autostart` | Run at system startup |
| `tauri-plugin-fs` | File system operations with watch |

## References

- [Tauri 2.x Documentation](https://v2.tauri.app/)
- [Calling Rust from Frontend](https://v2.tauri.app/develop/calling-rust)
