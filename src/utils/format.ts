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

/**
 * Formats a timestamp to relative time string (e.g., "2 hours ago").
 * @param timestamp - Unix timestamp in seconds
 * @returns Relative time string
 */
export function formatRelativeDate(timestamp: number): string {
  const date = new Date(timestamp * 1000)
  const now = new Date()
  const diff = now.getTime() - date.getTime()
  const seconds = Math.floor(diff / 1000)
  const minutes = Math.floor(seconds / 60)
  const hours = Math.floor(minutes / 60)
  const days = Math.floor(hours / 24)

  if (seconds < 60) {
    return 'Just now'
  } else if (minutes < 60) {
    return `${minutes} minute${minutes > 1 ? 's' : ''} ago`
  } else if (hours < 24) {
    return `${hours} hour${hours > 1 ? 's' : ''} ago`
  } else if (days < 7) {
    return `${days} day${days > 1 ? 's' : ''} ago`
  } else {
    return date.toLocaleDateString('en-US', { month: 'short', day: 'numeric', year: 'numeric' })
  }
}

