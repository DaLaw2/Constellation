import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useAppStore } from './app'

export interface DriveInfo {
  letter: string
  label: string | null
  drive_type: string
  total_space: number | null
  available_space: number | null
}

export interface FileEntry {
  name: string
  path: string
  is_directory: boolean
  size: number | null
  modified_time: number | null
  is_hidden: boolean
}

export interface FileMetadata {
  path: string
  size: number | null
  modified_time: number | null
  created_time: number | null
  is_directory: boolean
  is_readonly: boolean
}

export const useFileExplorerStore = defineStore('fileExplorer', () => {
  const currentPath = ref<string>('')
  const currentFiles = ref<FileEntry[]>([])
  const drives = ref<DriveInfo[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)

  // Link to app store for UI syncing
  const appStore = useAppStore()

  async function getDrives() {
    try {
      loading.value = true
      error.value = null
      drives.value = await invoke<DriveInfo[]>('get_drives')
      return drives.value
    } catch (e) {
      error.value = e as string
      console.error('Failed to get drives:', e)
      throw e
    } finally {
      loading.value = false
    }
  }

  async function readDirectory(path: string) {
    try {
      loading.value = true
      error.value = null
      currentFiles.value = await invoke<FileEntry[]>('read_directory', { path })
      currentPath.value = path
      // Sync with app store
      appStore.setCurrentPath(path)
      return currentFiles.value
    } catch (e) {
      error.value = e as string
      console.error('Failed to read directory:', e)
      throw e
    } finally {
      loading.value = false
    }
  }

  async function getFileMetadata(path: string) {
    try {
      const metadata = await invoke<FileMetadata>('get_file_metadata', { path })
      return metadata
    } catch (e) {
      error.value = e as string
      console.error('Failed to get file metadata:', e)
      throw e
    }
  }

  async function openFileExternal(path: string) {
    try {
      await invoke('open_file_external', { path })
    } catch (e) {
      error.value = e as string
      console.error('Failed to open file:', e)
      throw e
    }
  }

  async function revealInExplorer(path: string) {
    try {
      await invoke('reveal_in_explorer', { path })
    } catch (e) {
      error.value = e as string
      console.error('Failed to reveal in explorer:', e)
      throw e
    }
  }

  function navigateUp() {
    if (!currentPath.value) return

    const pathParts = currentPath.value.split('\\').filter(Boolean)
    if (pathParts.length <= 1) {
      // At drive root, clear current path
      currentPath.value = ''
      currentFiles.value = []
      // Sync with app store
      appStore.setCurrentPath('')
    } else {
      // Go up one directory
      pathParts.pop()
      const parentPath = pathParts.join('\\') + '\\'
      readDirectory(parentPath)
    }
  }

  function navigateTo(path: string) {
    readDirectory(path)
  }

  return {
    currentPath,
    currentFiles,
    drives,
    loading,
    error,
    getDrives,
    readDirectory,
    getFileMetadata,
    openFileExternal,
    revealInExplorer,
    navigateUp,
    navigateTo,
  }
})
