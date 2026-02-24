//! COM Worker Pool
//!
//! Manages multiple COM STA worker threads for parallel thumbnail generation.
//! Uses round-robin dispatch with an atomic counter (no locks needed).

use super::com_worker::ComWorker;
use super::generator::ThumbnailError;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

/// Pool of COM worker threads for parallel thumbnail generation.
///
/// Distributes requests across N dedicated STA threads using
/// round-robin selection with an atomic counter.
pub struct ComWorkerPool {
    workers: Vec<ComWorker>,
    next_worker: AtomicUsize,
}

impl ComWorkerPool {
    /// Spawn N COM worker threads.
    ///
    /// Each worker has its own OS thread with separate COM STA
    /// initialization and a dedicated request channel.
    /// `worker_count` is clamped to 1..=16.
    pub fn spawn(worker_count: usize) -> Result<Self, ThumbnailError> {
        let count = worker_count.clamp(1, 16);
        let mut workers = Vec::with_capacity(count);

        for i in 0..count {
            workers.push(ComWorker::spawn_named(i)?);
        }

        Ok(Self {
            workers,
            next_worker: AtomicUsize::new(0),
        })
    }

    /// Generate a thumbnail, routing to the next worker via round-robin.
    pub async fn generate(&self, path: PathBuf, size: u32) -> Result<Vec<u8>, ThumbnailError> {
        let index = self.next_worker.fetch_add(1, Ordering::Relaxed) % self.workers.len();
        self.workers[index].generate(path, size).await
    }

    /// Number of workers in the pool.
    #[allow(dead_code)]
    pub fn worker_count(&self) -> usize {
        self.workers.len()
    }
}
