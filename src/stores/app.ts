import { defineStore } from 'pinia'
import { ref } from 'vue'

export type ViewMode = 'file-browser' | 'tag-management'
export type DisplayMode = 'detail' | 'large-icons'

export const useAppStore = defineStore('app', () => {
  // Left panel mode
  const leftPanelMode = ref<ViewMode>('file-browser')

  // Display mode for file list
  const displayMode = ref<DisplayMode>('detail')

  // Current path
  const currentPath = ref<string>('')

  // Search query
  const searchQuery = ref<string>('')

  // Actions
  function setLeftPanelMode(mode: ViewMode) {
    leftPanelMode.value = mode
  }

  function setDisplayMode(mode: DisplayMode) {
    displayMode.value = mode
  }

  function setCurrentPath(path: string) {
    currentPath.value = path
  }

  function setSearchQuery(query: string) {
    searchQuery.value = query
  }

  return {
    leftPanelMode,
    displayMode,
    currentPath,
    searchQuery,
    setLeftPanelMode,
    setDisplayMode,
    setCurrentPath,
    setSearchQuery,
  }
})
