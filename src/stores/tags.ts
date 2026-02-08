import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { Tag, TagGroup } from '@/types'

export const useTagsStore = defineStore('tags', () => {
  const tagGroups = ref<TagGroup[]>([])
  const tags = ref<Tag[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)
  const usageCounts = ref<Record<number, number>>({})
  // Version counter: increments when item-tag associations change (for cache invalidation)
  const itemTagsVersion = ref(0)

  async function loadTagGroups(silent = false) {
    if (!silent) {
      loading.value = true
      error.value = null
    }
    try {
      tagGroups.value = await invoke<TagGroup[]>('get_tag_groups')
    } catch (e) {
      error.value = e as string
      console.error('Failed to load tag groups:', e)
    } finally {
      if (!silent) {
        loading.value = false
      }
    }
  }

  async function loadTags(silent = false) {
    if (!silent) {
      loading.value = true
      error.value = null
    }
    try {
      tags.value = await invoke<Tag[]>('get_all_tags')
    } catch (e) {
      error.value = e as string
      console.error('Failed to load tags:', e)
    } finally {
      if (!silent) {
        loading.value = false
      }
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
      await loadTagGroups(true)
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
      await loadTags(true)
      await loadUsageCounts()
      return id
    } catch (e) {
      error.value = e as string
      console.error('Failed to create tag:', e)
      throw e
    }
  }

  async function updateTagGroup(id: number, name?: string, color?: string) {
    try {
      await invoke('update_tag_group', {
        id,
        name: name || null,
        color: color || null,
      })
      await loadTagGroups(true)
    } catch (e) {
      error.value = e as string
      console.error('Failed to update tag group:', e)
      throw e
    }
  }

  async function updateTag(id: number, value?: string, groupId?: number) {
    try {
      await invoke('update_tag', {
        id,
        value: value || null,
        groupId: groupId || null,
      })
      await loadTags(true)
      await loadUsageCounts()
    } catch (e) {
      error.value = e as string
      console.error('Failed to update tag:', e)
      throw e
    }
  }

  async function mergeTags(sourceId: number, targetId: number) {
    try {
      await invoke('merge_tags', { sourceId, targetId })
      await loadTags(true)
      await loadUsageCounts()
      itemTagsVersion.value++
    } catch (e) {
      error.value = e as string
      console.error('Failed to merge tags:', e)
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
      await loadTagGroups(true)
    } catch (e) {
      error.value = e as string
      console.error('Failed to reorder tag groups:', e)
      throw e
    }
  }

  async function deleteTagGroup(id: number) {
    try {
      await invoke('delete_tag_group', { id })
      await loadTagGroups(true)
      await loadTags(true)
      await loadUsageCounts()
      itemTagsVersion.value++
    } catch (e) {
      error.value = e as string
      console.error('Failed to delete tag group:', e)
      throw e
    }
  }

  async function deleteTag(id: number) {
    try {
      await invoke('delete_tag', { id })
      await loadTags(true)
      await loadUsageCounts()
      itemTagsVersion.value++
    } catch (e) {
      error.value = e as string
      console.error('Failed to delete tag:', e)
      throw e
    }
  }

  return {
    tagGroups,
    tags,
    usageCounts,
    loading,
    error,
    itemTagsVersion,
    loadTagGroups,
    loadTags,
    loadUsageCounts,
    searchTags,
    createTagGroup,
    updateTagGroup,
    createTag,
    updateTag,
    mergeTags,
    getTagsByGroup,
    reorderTagGroups,
    deleteTagGroup,
    deleteTag,
  }
})
