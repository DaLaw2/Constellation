//! Thumbnail Service
//!
//! Orchestrates thumbnail generation with caching and concurrency control.

use crate::application::services::SettingsService;
use crate::infrastructure::thumbnail::{ComWorker, ThumbnailCache, ThumbnailError};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Semaphore;

/// Statistics about the thumbnail cache.
pub struct CacheStats {
    pub total_size_bytes: u64,
    pub file_count: u64,
    pub max_size_bytes: u64,
}

/// Service for thumbnail generation with disk caching and concurrency control.
pub struct ThumbnailService {
    cache: ThumbnailCache,
    worker: ComWorker,
    semaphore: Arc<Semaphore>,
    settings_service: Arc<SettingsService>,
}

impl ThumbnailService {
    /// Create a new thumbnail service.
    ///
    /// - `app_data_dir`: Base AppData directory (thumbnails stored in `{dir}/thumbnails/`)
    /// - `settings_service`: For reading thumbnail-related settings
    pub fn new(app_data_dir: PathBuf, settings_service: Arc<SettingsService>) -> Self {
        let cache_dir = app_data_dir.join("thumbnails");
        let cache = ThumbnailCache::new(cache_dir);
        let worker = ComWorker::spawn();
        let semaphore = Arc::new(Semaphore::new(4));

        Self {
            cache,
            worker,
            semaphore,
            settings_service,
        }
    }

    /// Get or generate a thumbnail. Returns WebP-encoded bytes.
    pub async fn get_thumbnail(
        &self,
        file_path: &str,
        mtime: i64,
        file_size: u64,
        thumb_size: u32,
    ) -> Result<Vec<u8>, ThumbnailError> {
        let _permit = self
            .semaphore
            .acquire()
            .await
            .map_err(|_| ThumbnailError::ChannelClosed)?;

        let force_shell = self.is_force_shell_cache().await;

        if !force_shell {
            let hash = ThumbnailCache::cache_key(file_path, mtime, file_size, thumb_size);
            if let Some(bytes) = self.cache.get(&hash).map_err(ThumbnailError::Io)? {
                return Ok(bytes);
            }
        }

        let webp = self
            .worker
            .generate(PathBuf::from(file_path), thumb_size)
            .await?;

        if !force_shell {
            let hash = ThumbnailCache::cache_key(file_path, mtime, file_size, thumb_size);
            // Best-effort cache store — don't fail the request if caching fails
            if let Err(e) = self.cache.put(&hash, &webp) {
                eprintln!("Failed to cache thumbnail: {}", e);
            }
        }

        Ok(webp)
    }

    /// Clear all cached thumbnails.
    pub async fn clear_cache(&self) -> Result<CacheStats, ThumbnailError> {
        self.cache.clear().map_err(ThumbnailError::Io)?;
        Ok(CacheStats {
            total_size_bytes: 0,
            file_count: 0,
            max_size_bytes: self.cache_max_bytes().await,
        })
    }

    /// Get cache statistics.
    pub async fn cache_stats(&self) -> Result<CacheStats, ThumbnailError> {
        let total_size_bytes = self.cache.total_size().map_err(ThumbnailError::Io)?;
        let file_count = self.cache.file_count().map_err(ThumbnailError::Io)?;
        Ok(CacheStats {
            total_size_bytes,
            file_count,
            max_size_bytes: self.cache_max_bytes().await,
        })
    }

    /// Run cache eviction (delete oldest entries until under size limit).
    pub async fn evict_cache(&self) -> Result<u64, ThumbnailError> {
        let max = self.cache_max_bytes().await;
        self.cache.evict_to_limit(max).map_err(ThumbnailError::Io)
    }

    /// Check if force shell cache mode is enabled.
    async fn is_force_shell_cache(&self) -> bool {
        match self
            .settings_service
            .get("thumbnail_force_shell_cache")
            .await
        {
            Ok(Some(val)) => val == "true",
            _ => false,
        }
    }

    /// Reads the cache size limit from settings (default 500MB).
    async fn cache_max_bytes(&self) -> u64 {
        let mb = self
            .settings_service
            .get("thumbnail_cache_max_mb")
            .await
            .ok()
            .flatten()
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(500);
        mb * 1024 * 1024
    }
}
