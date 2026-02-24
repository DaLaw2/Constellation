//! Thumbnail Infrastructure
//!
//! Provides thumbnail generation via Windows Shell API,
//! disk caching, and a COM worker thread for async operation.

mod cache;
mod com_worker;
mod com_worker_pool;
mod generator;

pub use cache::ThumbnailCache;
pub use com_worker_pool::ComWorkerPool;
pub use generator::ThumbnailError;
