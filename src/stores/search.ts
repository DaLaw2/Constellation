import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { Item, SearchMode, SearchInputMode } from '@/types'

export const useSearchStore = defineStore('search', () => {
  const results = ref<Item[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)
  const mode = ref<SearchMode>('and')
  const selectedTagIds = ref<number[]>([])
  const filenameQuery = ref('')

  // CQL mode state
  const searchInputMode = ref<SearchInputMode>('simple')
  const cqlQuery = ref('')
  const cqlError = ref<string | null>(null)

  const hasSearchCriteria = computed(() => {
    // Must have at least one tag selected to search
    return selectedTagIds.value.length > 0
  })

  const resultCount = computed(() => results.value.length)

  async function searchByTagsAnd(tagIds: number[]): Promise<Item[]> {
    try {
      return await invoke<Item[]>('search_items_by_tags_and', { tagIds })
    } catch (e) {
      console.error('Failed to search by tags (AND):', e)
      return []
    }
  }

  async function searchByTagsOr(tagIds: number[]): Promise<Item[]> {
    try {
      return await invoke<Item[]>('search_items_by_tags_or', { tagIds })
    } catch (e) {
      console.error('Failed to search by tags (OR):', e)
      return []
    }
  }

  async function searchByFilename(query: string): Promise<Item[]> {
    try {
      return await invoke<Item[]>('search_items_by_filename', { query })
    } catch (e) {
      console.error('Failed to search by filename:', e)
      return []
    }
  }

  async function executeSearch() {
    if (!hasSearchCriteria.value) {
      results.value = []
      return
    }

    loading.value = true
    error.value = null

    try {
      const filenameQueryValue = filenameQuery.value.trim() || null
      results.value = await invoke<Item[]>('search_items', {
        tagIds: selectedTagIds.value,
        mode: mode.value,
        filenameQuery: filenameQueryValue,
      })
    } catch (e) {
      error.value = e as string
      console.error('Failed to execute search:', e)
      results.value = []
    } finally {
      loading.value = false
    }
  }

  function setMode(newMode: SearchMode) {
    mode.value = newMode
  }

  function toggleTag(tagId: number) {
    const index = selectedTagIds.value.indexOf(tagId)
    if (index === -1) {
      selectedTagIds.value.push(tagId)
    } else {
      selectedTagIds.value.splice(index, 1)
    }
  }

  function selectTag(tagId: number) {
    if (!selectedTagIds.value.includes(tagId)) {
      selectedTagIds.value.push(tagId)
    }
  }

  function deselectTag(tagId: number) {
    const index = selectedTagIds.value.indexOf(tagId)
    if (index !== -1) {
      selectedTagIds.value.splice(index, 1)
    }
  }

  function clearSelectedTags() {
    selectedTagIds.value = []
  }

  function setFilenameQuery(query: string) {
    filenameQuery.value = query
  }

  function setSearchInputMode(newMode: SearchInputMode) {
    searchInputMode.value = newMode
    // Clear results and errors when switching modes
    results.value = []
    error.value = null
    cqlError.value = null
  }

  function setCqlQuery(query: string) {
    cqlQuery.value = query
  }

  async function executeCqlSearch() {
    const query = cqlQuery.value.trim()
    if (!query) {
      results.value = []
      return
    }

    loading.value = true
    cqlError.value = null
    error.value = null

    try {
      results.value = await invoke<Item[]>('search_cql', { query })
    } catch (e) {
      const errMsg = typeof e === 'string' ? e : String(e)
      cqlError.value = errMsg
      results.value = []
    } finally {
      loading.value = false
    }
  }

  function clearSearch() {
    results.value = []
    selectedTagIds.value = []
    filenameQuery.value = ''
    cqlQuery.value = ''
    error.value = null
    cqlError.value = null
  }

  return {
    results,
    loading,
    error,
    mode,
    selectedTagIds,
    filenameQuery,
    hasSearchCriteria,
    resultCount,
    searchByTagsAnd,
    searchByTagsOr,
    searchByFilename,
    executeSearch,
    setMode,
    toggleTag,
    selectTag,
    deselectTag,
    clearSelectedTags,
    setFilenameQuery,
    searchInputMode,
    cqlQuery,
    cqlError,
    setSearchInputMode,
    setCqlQuery,
    executeCqlSearch,
    clearSearch,
  }
})
