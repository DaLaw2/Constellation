import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { Tag } from './tags'

export interface Item {
  id: number
  path: string
  is_directory: boolean
  size: number | null
  modified_time: number | null
  created_at: number
  updated_at: number
  is_deleted: boolean
  deleted_at: number | null
}

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

  async function addTagToItem(itemId: number, tagId: number) {
    try {
      await invoke('add_tag_to_item', {
        itemId,
        tagId,
      })
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

  return {
    items,
    loading,
    error,
    createItem,
    getItem,
    getItemByPath,
    addTagToItem,
    removeTagFromItem,
    getTagsForItem,
  }
})
