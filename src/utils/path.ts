/**
 * Path utility functions.
 */

/**
 * Extracts the filename from a path.
 * @param path - The full file path
 * @returns The filename
 */
export function getFileName(path: string): string {
  const parts = path.split('\\')
  return parts[parts.length - 1] || path
}

/**
 * Extracts the parent directory from a path.
 * @param path - The full file path
 * @returns The parent directory path
 */
export function getParentPath(path: string): string {
  const parts = path.split('\\').filter(Boolean)
  if (parts.length <= 1) return ''
  parts.pop()
  return parts.join('\\') + '\\'
}

/**
 * Checks if a path is a root drive path.
 * @param path - The path to check
 * @returns True if the path is a drive root (e.g., "C:\")
 */
export function isDriveRoot(path: string): boolean {
  const parts = path.split('\\').filter(Boolean)
  return parts.length === 1
}
