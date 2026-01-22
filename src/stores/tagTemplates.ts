import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { TagTemplate } from '@/types'

export const useTagTemplatesStore = defineStore('tagTemplates', () => {
  const templates = ref<TagTemplate[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)

  async function loadTemplates() {
    loading.value = true
    error.value = null
    try {
      templates.value = await invoke<TagTemplate[]>('get_tag_templates')
    } catch (e) {
      error.value = e as string
      console.error('Failed to load tag templates:', e)
    } finally {
      loading.value = false
    }
  }

  async function createTemplate(name: string, tagIds: number[]) {
    try {
      const id = await invoke<number>('create_tag_template', { name, tagIds })
      await loadTemplates()
      return id
    } catch (e) {
      error.value = e as string
      console.error('Failed to create tag template:', e)
      throw e
    }
  }

  async function applyTemplate(itemId: number, templateId: number) {
    try {
      await invoke('apply_tag_template', { itemId, templateId })
    } catch (e) {
      error.value = e as string
      console.error('Failed to apply tag template:', e)
      throw e
    }
  }

  async function deleteTemplate(id: number) {
    try {
      await invoke('delete_tag_template', { id })
      await loadTemplates()
    } catch (e) {
      error.value = e as string
      console.error('Failed to delete tag template:', e)
      throw e
    }
  }

  async function updateTemplate(id: number, name?: string, tagIds?: number[]) {
    try {
      await invoke('update_tag_template', { id, name, tagIds })
      await loadTemplates()
    } catch (e) {
      error.value = e as string
      console.error('Failed to update tag template:', e)
      throw e
    }
  }

  return {
    templates,
    loading,
    error,
    loadTemplates,
    createTemplate,
    applyTemplate,
    deleteTemplate,
    updateTemplate,
  }
})
