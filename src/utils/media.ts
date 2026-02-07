/**
 * Media utilities for thumbnail URLs, asset URLs, and file type detection.
 */

import { convertFileSrc } from '@tauri-apps/api/core'

const IMAGE_EXTENSIONS = ['jpg', 'jpeg', 'png', 'gif', 'webp', 'bmp', 'svg', 'ico', 'tiff', 'tif']
const VIDEO_EXTENSIONS = ['mp4', 'avi', 'mkv', 'mov', 'wmv', 'flv', 'webm', 'm4v', 'mpg', 'mpeg']

/**
 * Get the file extension in lowercase.
 */
function getExtension(filename: string): string {
  return filename.split('.').pop()?.toLowerCase() || ''
}

/**
 * Check if file is an image based on extension.
 */
export function isImageFile(filename: string): boolean {
  return IMAGE_EXTENSIONS.includes(getExtension(filename))
}

/**
 * Check if file is a video based on extension.
 */
export function isVideoFile(filename: string): boolean {
  return VIDEO_EXTENSIONS.includes(getExtension(filename))
}

/**
 * Check if file is a media file (image or video).
 */
export function isMediaFile(filename: string): boolean {
  return isImageFile(filename) || isVideoFile(filename)
}

/**
 * Get thumbnail URL for a file path.
 * Uses the custom `thumb://` protocol registered in the Tauri backend.
 *
 * @param filePath - Absolute file path
 * @param size - Thumbnail size in pixels (default 256)
 * @returns URL string for the thumbnail
 */
export function getThumbnailUrl(filePath: string, size: number = 256): string {
  const encoded = encodeURIComponent(filePath)
  return `http://thumb.localhost/${encoded}?size=${size}`
}

/**
 * Get full-resolution asset URL for a file path.
 * Uses Tauri's built-in asset protocol for streaming files to WebView.
 *
 * @param filePath - Absolute file path
 * @returns URL string for the full-resolution asset
 */
export function getAssetUrl(filePath: string): string {
  return convertFileSrc(filePath)
}
