/**
 * Formatting utility functions.
 */

/**
 * Formats bytes to human-readable string.
 * @param bytes - The number of bytes
 * @param decimals - Number of decimal places (default: 1)
 * @returns Formatted string like "1.5 MB"
 */
export function formatBytes(bytes: number | null, decimals = 1): string {
  if (bytes === null || bytes === 0) return '0 B'

  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))

  return parseFloat((bytes / Math.pow(k, i)).toFixed(decimals)) + ' ' + sizes[i]
}

/**
 * Formats a timestamp to locale date string.
 * @param timestamp - Unix timestamp in seconds
 * @returns Formatted date string
 */
export function formatDate(timestamp: number | null): string {
  if (!timestamp) return ''
  return new Date(timestamp * 1000).toLocaleDateString()
}

/**
 * Formats a timestamp to locale date and time string.
 * @param timestamp - Unix timestamp in seconds
 * @returns Formatted date and time string
 */
export function formatDateTime(timestamp: number | null): string {
  if (!timestamp) return ''
  return new Date(timestamp * 1000).toLocaleString()
}
