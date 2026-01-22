/**
 * File system types for the Constellation application.
 * These types represent file system entities and operations.
 */

/** System drive information */
export interface DriveInfo {
  letter: string
  label: string | null
  drive_type: string
  total_space: number | null
  available_space: number | null
}

/** File or directory entry in a directory listing */
export interface FileEntry {
  name: string
  path: string
  is_directory: boolean
  size: number | null
  modified_time: number | null
  is_hidden: boolean
}

/** Detailed file metadata */
export interface FileMetadata {
  path: string
  size: number | null
  modified_time: number | null
  created_time: number | null
  is_directory: boolean
  is_readonly: boolean
}
