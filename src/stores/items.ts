import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useTagsStore } from '@/stores/tags'
import type { Item, Tag } from '@/types'

export const useItemsStore = defineStore('items', () => {
  const items = ref<Item[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)

  async function createItem(
    path: string,
    isDirectory: boolean,
    size: number | null = null,
    modifiedTime: number | null = null
  ) {
    try {
      const id = await invoke<number>('create_item', {
        path,
        isDirectory,
        size,
        modifiedTime,
      })
      return id
    } catch (e) {
      error.value = e as string
      console.error('Failed to create item:', e)
      throw e
    }
  }

  async function getItem(id: number) {
    try {
      const item = await invoke<Item>('get_item', { id })
      return item
    } catch (e) {
      error.value = e as string
      console.error('Failed to get item:', e)
      throw e
    }
  }

  async function getItemByPath(path: string) {
    try {
      const item = await invoke<Item | null>('get_item_by_path', { path })
      return item
    } catch (e) {
      error.value = e as string
      console.error('Failed to get item by path:', e)
      throw e
    }
  }

  async function getItemsByPaths(paths: string[]) {
    try {
      const items = await invoke<Item[]>('get_items_by_paths', { paths })
      return items
    } catch (e) {
      error.value = e as string
      console.error('Failed to get items by paths:', e)
      throw e
    }
  }

  async function addTagToItem(itemId: number, tagId: number) {
    try {
      await invoke('add_tag_to_item', {
        itemId,
        tagId,
      })
      const tagsStore = useTagsStore()
      await tagsStore.loadUsageCounts()
      tagsStore.itemTagsVersion++
    } catch (e) {
      error.value = e as string
      console.error('Failed to add tag to item:', e)
      throw e
    }
  }

  async function removeTagFromItem(itemId: number, tagId: number) {
    try {
      await invoke('remove_tag_from_item', {
        itemId,
        tagId,
      })
      const tagsStore = useTagsStore()
      await tagsStore.loadUsageCounts()
      tagsStore.itemTagsVersion++
    } catch (e) {
      error.value = e as string
      console.error('Failed to remove tag from item:', e)
      throw e
    }
  }

  async function getTagsForItem(itemId: number) {
    try {
      const tags = await invoke<Tag[]>('get_tags_for_item', { itemId })
      return tags
    } catch (e) {
      error.value = e as string
      console.error('Failed to get tags for item:', e)
      throw e
    }
  }

  async function getTagsForItems(itemIds: number[]) {
    try {
      const tagsMap = await invoke<Record<number, Tag[]>>('get_tags_for_items', { itemIds })
      return tagsMap
    } catch (e) {
      error.value = e as string
      console.error('Failed to get tags for items:', e)
      throw e
    }
  }

  async function updateItemTags(itemId: number, tagIds: number[]) {
    try {
      await invoke('update_item_tags', { itemId, tagIds })
      const tagsStore = useTagsStore()
      await tagsStore.loadUsageCounts()
      tagsStore.itemTagsVersion++
    } catch (e) {
      error.value = e as string
      console.error('Failed to update item tags:', e)
      throw e
    }
  }

  return {
    items,
    loading,
    error,
    createItem,
    getItem,
    getItemByPath,
    getItemsByPaths,
    addTagToItem,
    removeTagFromItem,
    getTagsForItem,
    getTagsForItems,
    updateItemTags,
  }
})
