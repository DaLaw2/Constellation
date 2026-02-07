<template>
  <div class="preview-settings">
    <div class="setting-group">
      <h3 class="group-title">Thumbnails</h3>

      <!-- Thumbnail Size -->
      <div class="setting-row">
        <div class="setting-info">
          <label class="setting-label">Thumbnail Size</label>
          <span class="setting-description">Size of generated thumbnails in pixels</span>
        </div>
        <select
          class="setting-select"
          :value="settingsStore.settings.thumbnail_size"
          @change="handleThumbnailSizeChange"
        >
          <option :value="128">128px</option>
          <option :value="256">256px (Default)</option>
          <option :value="512">512px</option>
        </select>
      </div>

      <!-- Force Shell Cache -->
      <div class="setting-row">
        <div class="setting-info">
          <label class="setting-label">Use Windows Shell Cache Only</label>
          <span class="setting-description">
            Skip disk cache and rely solely on Windows Shell thumbnail cache.
            Reduces disk usage but may be slower on repeat views.
          </span>
        </div>
        <label class="toggle">
          <input
            type="checkbox"
            :checked="settingsStore.settings.thumbnail_force_shell_cache"
            @change="handleForceShellChange"
          />
          <span class="toggle-slider"></span>
        </label>
      </div>

      <!-- Cache Max Size -->
      <div class="setting-row">
        <div class="setting-info">
          <label class="setting-label">Cache Size Limit</label>
          <span class="setting-description">Maximum disk space for thumbnail cache</span>
        </div>
        <select
          class="setting-select"
          :value="settingsStore.settings.thumbnail_cache_max_mb"
          @change="handleCacheMaxChange"
        >
          <option :value="100">100 MB</option>
          <option :value="250">250 MB</option>
          <option :value="500">500 MB (Default)</option>
          <option :value="1000">1 GB</option>
          <option :value="2000">2 GB</option>
        </select>
      </div>
    </div>

    <!-- Cache Management -->
    <div class="setting-group">
      <h3 class="group-title">Cache Management</h3>

      <div class="cache-info" v-if="cacheStats">
        <div class="cache-stat">
          <span class="stat-label">Cache Size</span>
          <span class="stat-value">{{ formatCacheSize(cacheStats.total_size_bytes) }}</span>
        </div>
        <div class="cache-stat">
          <span class="stat-label">Cached Files</span>
          <span class="stat-value">{{ cacheStats.file_count.toLocaleString() }}</span>
        </div>
      </div>

      <button
        class="clear-cache-btn"
        @click="handleClearCache"
        :disabled="clearing"
      >
        {{ clearing ? 'Clearing...' : 'Clear Thumbnail Cache' }}
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useSettingsStore, type CacheStats } from '@/stores/settings'

const settingsStore = useSettingsStore()
const cacheStats = ref<CacheStats | null>(null)
const clearing = ref(false)

onMounted(async () => {
  await loadCacheStats()
})

async function loadCacheStats() {
  try {
    cacheStats.value = await settingsStore.getCacheStats()
  } catch (e) {
    console.error('Failed to load cache stats:', e)
  }
}

function handleThumbnailSizeChange(event: Event) {
  const value = (event.target as HTMLSelectElement).value
  settingsStore.updateSetting('thumbnail_size', value)
}

function handleForceShellChange(event: Event) {
  const checked = (event.target as HTMLInputElement).checked
  settingsStore.updateSetting('thumbnail_force_shell_cache', String(checked))
}

function handleCacheMaxChange(event: Event) {
  const value = (event.target as HTMLSelectElement).value
  settingsStore.updateSetting('thumbnail_cache_max_mb', value)
}

async function handleClearCache() {
  clearing.value = true
  try {
    await settingsStore.clearThumbnailCache()
    await loadCacheStats()
  } catch (e) {
    console.error('Failed to clear cache:', e)
  } finally {
    clearing.value = false
  }
}

function formatCacheSize(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`
  if (bytes < 1024 * 1024 * 1024) return `${(bytes / (1024 * 1024)).toFixed(1)} MB`
  return `${(bytes / (1024 * 1024 * 1024)).toFixed(2)} GB`
}
</script>

<style scoped>
.preview-settings {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.setting-group {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.group-title {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0 0 4px 0;
}

.setting-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 16px;
  padding: 8px 0;
}

.setting-info {
  flex: 1;
  min-width: 0;
}

.setting-label {
  display: block;
  font-size: 13px;
  font-weight: 500;
  color: var(--text-primary);
  margin-bottom: 2px;
}

.setting-description {
  display: block;
  font-size: 12px;
  color: var(--text-secondary);
  line-height: 1.4;
}

.setting-select {
  padding: 6px 28px 6px 10px;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  background: var(--surface);
  color: var(--text-primary);
  font-size: 13px;
  cursor: pointer;
  appearance: none;
  background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='12' height='12' viewBox='0 0 24 24' fill='none' stroke='%23999' stroke-width='2'%3E%3Cpolyline points='6 9 12 15 18 9'/%3E%3C/svg%3E");
  background-repeat: no-repeat;
  background-position: right 8px center;
  min-width: 140px;
}

.setting-select:focus {
  outline: none;
  border-color: var(--primary-color);
}

/* Toggle switch */
.toggle {
  position: relative;
  display: inline-block;
  width: 40px;
  height: 22px;
  flex-shrink: 0;
}

.toggle input {
  opacity: 0;
  width: 0;
  height: 0;
}

.toggle-slider {
  position: absolute;
  cursor: pointer;
  inset: 0;
  background-color: var(--border-color);
  border-radius: 22px;
  transition: var(--transition-fast);
}

.toggle-slider::before {
  content: '';
  position: absolute;
  height: 16px;
  width: 16px;
  left: 3px;
  bottom: 3px;
  background-color: white;
  border-radius: 50%;
  transition: var(--transition-fast);
}

.toggle input:checked + .toggle-slider {
  background-color: var(--primary-color);
}

.toggle input:checked + .toggle-slider::before {
  transform: translateX(18px);
}

/* Cache info */
.cache-info {
  display: flex;
  gap: 24px;
  padding: 12px 16px;
  background: var(--surface);
  border-radius: 8px;
  border: 1px solid var(--border-color);
}

.cache-stat {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.stat-label {
  font-size: 11px;
  color: var(--text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.stat-value {
  font-size: 15px;
  font-weight: 600;
  color: var(--text-primary);
}

.clear-cache-btn {
  align-self: flex-start;
  padding: 8px 16px;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  background: var(--surface);
  color: var(--text-primary);
  font-size: 13px;
  cursor: pointer;
  transition: var(--transition-fast);
}

.clear-cache-btn:hover:not(:disabled) {
  border-color: #ef4444;
  color: #ef4444;
}

.clear-cache-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}
</style>
