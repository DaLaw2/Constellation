//! COM Worker Thread
//!
//! Runs a dedicated OS thread with COM STA initialization.
//! Receives thumbnail generation requests via a channel and
//! returns WebP-encoded thumbnails.

use super::generator::{generate_thumbnail, ThumbnailError};
use image::ImageBuffer;
use std::path::PathBuf;
use tokio::sync::{mpsc, oneshot};
use windows::Win32::System::Com::{CoInitializeEx, CoUninitialize, COINIT_APARTMENTTHREADED};

/// A request to generate a thumbnail on the COM worker thread.
struct ThumbnailRequest {
    file_path: PathBuf,
    size: u32,
    response: oneshot::Sender<Result<Vec<u8>, ThumbnailError>>,
}

/// Handle to the COM worker thread.
///
/// Requests are sent via a bounded channel and processed sequentially
/// on the STA thread. Results are returned as WebP-encoded bytes.
pub struct ComWorker {
    sender: mpsc::Sender<ThumbnailRequest>,
}

impl ComWorker {
    /// Spawn the dedicated COM STA thread.
    ///
    /// The thread initializes COM, then loops processing requests
    /// until the channel is closed (when `ComWorker` is dropped).
    pub fn spawn() -> Self {
        let (tx, mut rx) = mpsc::channel::<ThumbnailRequest>(64);

        std::thread::Builder::new()
            .name("com-thumbnail-worker".into())
            .spawn(move || {
                // Initialize COM in Single-Threaded Apartment mode
                unsafe {
                    let hr = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
                    if let Err(e) = hr.ok() {
                        eprintln!("Failed to initialize COM: {}", e);
                        // Drain remaining requests with error
                        while let Some(req) = rx.blocking_recv() {
                            let _ = req.response.send(Err(ThumbnailError::Com(e.clone())));
                        }
                        return;
                    }
                }

                // Process requests until the channel is closed
                while let Some(req) = rx.blocking_recv() {
                    let result = process_request(&req.file_path, req.size);
                    let _ = req.response.send(result);
                }

                unsafe {
                    CoUninitialize();
                }
            })
            .expect("Failed to spawn COM worker thread");

        Self { sender: tx }
    }

    /// Generate a thumbnail asynchronously.
    ///
    /// Sends the request to the COM worker thread and awaits the result.
    /// Returns WebP-encoded bytes.
    pub async fn generate(&self, path: PathBuf, size: u32) -> Result<Vec<u8>, ThumbnailError> {
        let (tx, rx) = oneshot::channel();

        self.sender
            .send(ThumbnailRequest {
                file_path: path,
                size,
                response: tx,
            })
            .await
            .map_err(|_| ThumbnailError::ChannelClosed)?;

        rx.await.map_err(|_| ThumbnailError::ChannelClosed)?
    }
}

/// Process a single thumbnail request: generate RGBA pixels, encode as WebP.
fn process_request(path: &PathBuf, size: u32) -> Result<Vec<u8>, ThumbnailError> {
    let (rgba_data, width, height) = generate_thumbnail(path.as_path(), size)?;

    encode_webp(&rgba_data, width, height)
}

/// Encode RGBA pixel data as WebP.
fn encode_webp(rgba_data: &[u8], width: u32, height: u32) -> Result<Vec<u8>, ThumbnailError> {
    let img: ImageBuffer<image::Rgba<u8>, _> =
        ImageBuffer::from_raw(width, height, rgba_data.to_vec())
            .ok_or_else(|| ThumbnailError::Encoding("Invalid image dimensions".into()))?;

    let mut buf = std::io::Cursor::new(Vec::new());
    img.write_to(&mut buf, image::ImageFormat::WebP)
        .map_err(|e| ThumbnailError::Encoding(e.to_string()))?;

    Ok(buf.into_inner())
}
