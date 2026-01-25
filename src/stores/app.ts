import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { ViewMode, DisplayMode, SearchHistory } from '@/types'

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

  // Search History
  const searchHistory = ref<SearchHistory[]>([])

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

  async function loadSearchHistory() {
    try {
      // Limit to 10 recent searches
      searchHistory.value = await invoke<SearchHistory[]>('get_recent_search_history', { limit: 10 })
    } catch (e) {
      console.error('Failed to load search history:', e)
    }
  }

  async function deleteSearchHistory(id: number) {
    try {
      await invoke('delete_search_history', { id })
      await loadSearchHistory()
    } catch (e) {
      console.error('Failed to delete search history:', e)
    }
  }

  async function clearSearchHistory() {
    try {
      await invoke('clear_search_history')
      await loadSearchHistory()
    } catch (e) {
      console.error('Failed to clear search history:', e)
    }
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
    searchHistory,
    loadSearchHistory,
    deleteSearchHistory,
    clearSearchHistory,
  }
})
