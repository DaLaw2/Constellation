import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export interface TagGroup {
  id: number
  name: string
  color: string | null
  display_order: number
  created_at: number
  updated_at: number
}

export interface Tag {
  id: number
  group_id: number
  value: string
  created_at: number
  updated_at: number
}

export const useTagsStore = defineStore('tags', () => {
  const tagGroups = ref<TagGroup[]>([])
  const tags = ref<Tag[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)
  const usageCounts = ref<Record<number, number>>({})

  async function loadTagGroups() {
    loading.value = true
    error.value = null
    try {
      tagGroups.value = await invoke<TagGroup[]>('get_tag_groups')
    } catch (e) {
      error.value = e as string
      console.error('Failed to load tag groups:', e)
    } finally {
      loading.value = false
    }
  }

  async function loadTags() {
    loading.value = true
    error.value = null
    try {
      tags.value = await invoke<Tag[]>('get_all_tags')
    } catch (e) {
      error.value = e as string
      console.error('Failed to load tags:', e)
    } finally {
      loading.value = false
    }
  }

  async function loadUsageCounts() {
    try {
      usageCounts.value = await invoke<Record<number, number>>('get_tag_usage_counts')
    } catch (e) {
      console.error('Failed to load usage counts:', e)
    }
  }

  async function searchTags(query: string, groupId?: number): Promise<Tag[]> {
    try {
      return await invoke<Tag[]>('search_tags', { query, groupId })
    } catch (e) {
      console.error('Failed to search tags:', e)
      return []
    }
  }

  async function createTagGroup(name: string, color: string | null = null, displayOrder: number = 0) {
    try {
      const id = await invoke<number>('create_tag_group', {
        name,
        color,
        displayOrder,
      })
      await loadTagGroups()
      return id
    } catch (e) {
      error.value = e as string
      console.error('Failed to create tag group:', e)
      throw e
    }
  }

  async function createTag(groupId: number, value: string) {
    try {
      const id = await invoke<number>('create_tag', {
        groupId,
        value,
      })
      await loadTags()
      return id
    } catch (e) {
      error.value = e as string
      console.error('Failed to create tag:', e)
      throw e
    }
  }

  function getTagsByGroup(groupId: number): Tag[] {
    return tags.value.filter(tag => tag.group_id === groupId)
  }

  async function reorderTagGroups(orderedIds: number[]) {
    try {
      const orders = orderedIds.map((id, index) => ({
        id,
        display_order: index,
      }))

      await invoke('reorder_tag_groups', { orders })
      await loadTagGroups()
    } catch (e) {
      error.value = e as string
      console.error('Failed to reorder tag groups:', e)
      throw e
    }
  }

  return {
    tagGroups,
    tags,
    usageCounts,
    loading,
    error,
    loadTagGroups,
    loadTags,
    loadUsageCounts,
    searchTags,
    createTagGroup,
    createTag,
    getTagsByGroup,
    reorderTagGroups,
  }
})
