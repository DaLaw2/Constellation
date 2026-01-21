import { defineStore } from 'pinia'
import { ref } from 'vue'

export type ViewMode = 'file-browser' | 'tag-management' | 'search'
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

  // Sidebar expansion state
  const sidebarExpanded = ref<boolean>(false)

  // Actions
  function setLeftPanelMode(mode: ViewMode) {
    leftPanelMode.value = mode
    // Auto-collapse when switching to file browser
    if (mode === 'file-browser') {
      sidebarExpanded.value = false
    }
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

  function setSidebarExpanded(expanded: boolean) {
    sidebarExpanded.value = expanded
  }

  function toggleSidebarExpanded() {
    sidebarExpanded.value = !sidebarExpanded.value
  }

  return {
    leftPanelMode,
    displayMode,
    currentPath,
    searchQuery,
    sidebarExpanded,
    setLeftPanelMode,
    setDisplayMode,
    setCurrentPath,
    setSearchQuery,
    setSidebarExpanded,
    toggleSidebarExpanded,
  }
})
