# Thumbnail Worker Pool

## Problem

Thumbnail generation in Tauri uses the Windows Shell API (`IShellItemImageFactory::GetImage()`), which requires COM STA (Single-Threaded Apartment) initialization. This forces all thumbnail generation onto a single dedicated OS thread, creating a bottleneck when loading many thumbnails simultaneously.

Additionally, WebView2 (Chromium-based) limits concurrent connections per host to ~6, and the backend semaphore was hardcoded to 4 — further compounding the bottleneck.

## Solution: Multi-Worker COM Pool

### Architecture

```
Frontend <img> requests  →  WebView (6 concurrent per host)
                                ↓
thumb:// protocol handler  →  Semaphore (configurable, default: workers * 2)
                                ↓
ComWorkerPool              →  Round-robin dispatch (AtomicUsize, lock-free)
  ├── com-thumb-worker-0   →  Dedicated STA thread + channel(64)
  ├── com-thumb-worker-1   →  Dedicated STA thread + channel(64)
  ├── com-thumb-worker-2   →  Dedicated STA thread + channel(64)
  └── com-thumb-worker-N   →  Dedicated STA thread + channel(64)
```

### Round-Robin Dispatch (No Locks)

Uses `AtomicUsize` with `Relaxed` ordering for round-robin worker selection. This avoids `Mutex`/`RwLock` per the project constitution:

```rust
pub struct ComWorkerPool {
    workers: Vec<ComWorker>,
    next_worker: AtomicUsize,
}

impl ComWorkerPool {
    pub async fn generate(&self, path: PathBuf, size: u32) -> Result<Vec<u8>, ThumbnailError> {
        let index = self.next_worker.fetch_add(1, Ordering::Relaxed) % self.workers.len();
        self.workers[index].generate(path, size).await
    }
}
```

### Each Worker

Each `ComWorker` is a dedicated OS thread with:
- Independent COM STA initialization (`CoInitializeEx(COINIT_APARTMENTTHREADED)`)
- Bounded `tokio::sync::mpsc` channel (capacity 64) for request queuing
- `oneshot` channels for returning results to async callers

### Configurable Settings

| Setting | Key | Default | Range |
|---------|-----|---------|-------|
| Worker Threads | `thumbnail_worker_count` | `0` (auto) | 1-16 |
| Concurrency Limit | `thumbnail_semaphore_count` | `0` (auto) | 1-32 |

**Auto defaults:**
- Workers: `max(2, CPU cores / 2)`
- Semaphore: `workers * 2`

Settings are read once at app startup. Changes require restart.

## Key Files

| File | Purpose |
|------|---------|
| `src-tauri/src/infrastructure/thumbnail/com_worker_pool.rs` | Worker pool with round-robin dispatch |
| `src-tauri/src/infrastructure/thumbnail/com_worker.rs` | Individual COM STA worker thread |
| `src-tauri/src/application/services/thumbnail_service.rs` | Orchestrates pool + cache + semaphore |
| `src-tauri/src/domain/entities/settings.rs` | Default values for worker/semaphore settings |
| `src/components/Settings/AdvancedSettings.vue` | UI for configuring worker/semaphore counts |

## References

- [COM Threading and Apartment Models](https://learn.microsoft.com/en-us/windows/win32/com/processes--threads--and-apartments)
- [IShellItemImageFactory](https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nn-shobjidl_core-ishellitemimagefactory)
- [Tokio Semaphore](https://docs.rs/tokio/latest/tokio/sync/struct.Semaphore.html)
