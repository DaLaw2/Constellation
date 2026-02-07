//! Thumbnail Generator
//!
//! Uses Windows Shell `IShellItemImageFactory::GetImage()` to generate
//! thumbnails for any file type with a registered thumbnail handler.
//! Must be called from a COM STA-initialized thread.

use std::path::Path;
use thiserror::Error;
use windows::core::HSTRING;
use windows::Win32::Foundation::SIZE;
use windows::Win32::Graphics::Gdi::{
    CreateCompatibleDC, DeleteDC, DeleteObject, GetDIBits, GetObjectW, BITMAP, BITMAPINFO,
    BITMAPINFOHEADER, BI_RGB, DIB_RGB_COLORS, RGBQUAD,
};
use windows::Win32::UI::Shell::{
    IShellItemImageFactory, SHCreateItemFromParsingName, SIIGBF_RESIZETOFIT,
};

#[derive(Debug, Error)]
pub enum ThumbnailError {
    #[error("COM error: {0}")]
    Com(#[from] windows::core::Error),

    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("Failed to extract bitmap pixels")]
    BitmapExtraction,

    #[error("Image encoding error: {0}")]
    Encoding(String),

    #[error("Worker channel closed")]
    ChannelClosed,

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// Generate a thumbnail for the given file path at the specified size.
///
/// **Must be called from a COM STA-initialized thread.**
///
/// Returns RGBA pixel data and the actual (width, height) of the thumbnail.
pub fn generate_thumbnail(path: &Path, size: u32) -> Result<(Vec<u8>, u32, u32), ThumbnailError> {
    let path_str = path
        .to_str()
        .ok_or_else(|| ThumbnailError::FileNotFound(path.display().to_string()))?;

    if !path.exists() {
        return Err(ThumbnailError::FileNotFound(path.display().to_string()));
    }

    unsafe { generate_thumbnail_inner(path_str, size) }
}

unsafe fn generate_thumbnail_inner(
    path_str: &str,
    size: u32,
) -> Result<(Vec<u8>, u32, u32), ThumbnailError> {
    let hpath = HSTRING::from(path_str);

    // Create IShellItem from file path, then query IShellItemImageFactory
    let factory: IShellItemImageFactory = SHCreateItemFromParsingName(&hpath, None)?;

    let desired = SIZE {
        cx: size as i32,
        cy: size as i32,
    };

    // GetImage with SIIGBF_RESIZETOFIT (default: shrink to fit, preserve aspect ratio)
    let hbitmap = factory.GetImage(desired, SIIGBF_RESIZETOFIT)?;

    // Get bitmap dimensions via GetObject (reliable for both DDB and DIB sections)
    let mut bm = BITMAP::default();
    let obj_result = GetObjectW(
        hbitmap,
        std::mem::size_of::<BITMAP>() as i32,
        Some(&mut bm as *mut _ as *mut _),
    );
    if obj_result == 0 || bm.bmWidth == 0 || bm.bmHeight == 0 {
        let _ = DeleteObject(hbitmap);
        return Err(ThumbnailError::BitmapExtraction);
    }

    let width = bm.bmWidth as u32;
    let height = bm.bmHeight as u32;

    // Create compatible DC for pixel extraction
    let hdc = CreateCompatibleDC(None);
    if hdc.is_invalid() {
        let _ = DeleteObject(hbitmap);
        return Err(ThumbnailError::BitmapExtraction);
    }

    // Set up BITMAPINFO for top-down 32-bit BGRA extraction
    let mut bmi = BITMAPINFO {
        bmiHeader: BITMAPINFOHEADER {
            biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
            biWidth: width as i32,
            biHeight: -(height as i32), // negative = top-down
            biPlanes: 1,
            biBitCount: 32,
            biCompression: BI_RGB.0 as u32,
            ..Default::default()
        },
        bmiColors: [RGBQUAD::default()],
    };

    let buf_size = (width * height * 4) as usize;
    let mut buffer = vec![0u8; buf_size];

    let result = GetDIBits(
        hdc,
        hbitmap,
        0,
        height,
        Some(buffer.as_mut_ptr() as *mut _),
        &mut bmi,
        DIB_RGB_COLORS,
    );

    let _ = DeleteDC(hdc);
    let _ = DeleteObject(hbitmap);

    if result == 0 {
        return Err(ThumbnailError::BitmapExtraction);
    }

    // Convert BGRA → RGBA
    for chunk in buffer.chunks_exact_mut(4) {
        chunk.swap(0, 2); // swap B and R
    }

    Ok((buffer, width, height))
}
