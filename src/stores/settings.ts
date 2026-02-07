import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export interface AppSettings {
  usn_auto_refresh: boolean
  usn_refresh_on_missing: boolean
  usn_cross_volume_match: boolean
  thumbnail_size: number
  thumbnail_force_shell_cache: boolean
  thumbnail_cache_max_mb: number
}

export interface CacheStats {
  total_size_bytes: number
  file_count: number
  max_size_bytes: number
}

const DEFAULTS: AppSettings = {
  usn_auto_refresh: false,
  usn_refresh_on_missing: true,
  usn_cross_volume_match: true,
  thumbnail_size: 256,
  thumbnail_force_shell_cache: false,
  thumbnail_cache_max_mb: 500,
}

function parseSettings(raw: Record<string, string>): AppSettings {
  return {
    usn_auto_refresh: raw.usn_auto_refresh === 'true',
    usn_refresh_on_missing: raw.usn_refresh_on_missing !== 'false',
    usn_cross_volume_match: raw.usn_cross_volume_match !== 'false',
    thumbnail_size: parseInt(raw.thumbnail_size || '256', 10),
    thumbnail_force_shell_cache: raw.thumbnail_force_shell_cache === 'true',
    thumbnail_cache_max_mb: parseInt(raw.thumbnail_cache_max_mb || '500', 10),
  }
}

export const useSettingsStore = defineStore('settings', () => {
  const raw = ref<Record<string, string>>({})
  const loading = ref(false)

  const settings = computed<AppSettings>(() => {
    if (Object.keys(raw.value).length === 0) return { ...DEFAULTS }
    return parseSettings(raw.value)
  })

  async function loadSettings() {
    loading.value = true
    try {
      raw.value = await invoke<Record<string, string>>('get_all_settings')
    } catch (e) {
      console.error('Failed to load settings:', e)
    } finally {
      loading.value = false
    }
  }

  async function updateSetting(key: string, value: string) {
    try {
      await invoke('update_setting', { key, value })
      raw.value = { ...raw.value, [key]: value }
    } catch (e) {
      console.error('Failed to update setting:', e)
      throw e
    }
  }

  async function resetSetting(key: string) {
    try {
      await invoke('reset_setting', { key })
      const updated = { ...raw.value }
      delete updated[key]
      raw.value = updated
    } catch (e) {
      console.error('Failed to reset setting:', e)
      throw e
    }
  }

  async function getCacheStats(): Promise<CacheStats> {
    return await invoke<CacheStats>('get_cache_stats')
  }

  async function clearThumbnailCache(): Promise<CacheStats> {
    return await invoke<CacheStats>('clear_thumbnail_cache')
  }

  return {
    settings,
    loading,
    loadSettings,
    updateSetting,
    resetSetting,
    getCacheStats,
    clearThumbnailCache,
  }
})
