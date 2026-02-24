<template>
  <div class="advanced-settings">
    <!-- Restart hint -->
    <div class="restart-hint">
      Changes to worker and concurrency settings require an app restart to take effect.
    </div>

    <div class="setting-group">
      <h3 class="group-title">Thumbnail Generation</h3>

      <!-- Worker Threads -->
      <div class="setting-row">
        <div class="setting-info">
          <label class="setting-label">Worker Threads</label>
          <span class="setting-description">
            Number of dedicated threads for thumbnail generation.
            Auto selects based on CPU cores.
          </span>
        </div>
        <select
          class="setting-select"
          :value="settingsStore.settings.thumbnail_worker_count"
          @change="handleWorkerChange"
        >
          <option :value="0">Auto</option>
          <option v-for="n in workerOptions" :key="n" :value="n">{{ n }}</option>
        </select>
      </div>

      <!-- Concurrency Limit -->
      <div class="setting-row">
        <div class="setting-info">
          <label class="setting-label">Concurrency Limit</label>
          <span class="setting-description">
            Maximum concurrent thumbnail requests. Higher values use more memory.
            Auto selects based on worker count.
          </span>
        </div>
        <select
          class="setting-select"
          :value="settingsStore.settings.thumbnail_semaphore_count"
          @change="handleSemaphoreChange"
        >
          <option :value="0">Auto</option>
          <option v-for="n in semaphoreOptions" :key="n" :value="n">{{ n }}</option>
        </select>
      </div>
    </div>

    <div class="setting-group">
      <h3 class="group-title">Cache Behavior</h3>

      <!-- Force Shell Cache -->
      <div class="setting-row">
        <div class="setting-info">
          <label class="setting-label">Use Windows Shell Cache Only</label>
          <span class="setting-description">
            Skip disk cache and rely solely on Windows Shell thumbnail cache.
            Reduces disk usage but may be slower on repeat views.
          </span>
        </div>
        <label class="toggle-switch">
          <input
            type="checkbox"
            :checked="settingsStore.settings.thumbnail_force_shell_cache"
            @change="handleForceShellChange"
          />
          <span class="toggle-slider"></span>
        </label>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useSettingsStore } from '@/stores/settings'

const settingsStore = useSettingsStore()

const workerOptions = [1, 2, 3, 4, 6, 8, 12, 16]
const semaphoreOptions = [2, 4, 6, 8, 12, 16, 24, 32]

function handleWorkerChange(event: Event) {
  const value = (event.target as HTMLSelectElement).value
  settingsStore.updateSetting('thumbnail_worker_count', value)
}

function handleSemaphoreChange(event: Event) {
  const value = (event.target as HTMLSelectElement).value
  settingsStore.updateSetting('thumbnail_semaphore_count', value)
}

function handleForceShellChange(event: Event) {
  const checked = (event.target as HTMLInputElement).checked
  settingsStore.updateSetting('thumbnail_force_shell_cache', String(checked))
}
</script>

<style scoped>
.advanced-settings {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.restart-hint {
  padding: 10px 14px;
  background: rgba(234, 179, 8, 0.1);
  border: 1px solid rgba(234, 179, 8, 0.3);
  border-radius: 8px;
  font-size: 12px;
  color: var(--text-secondary);
  line-height: 1.4;
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

/* Toggle switch - matches FileTrackingSettings */
.toggle-switch {
  position: relative;
  display: inline-block;
  width: 44px;
  height: 24px;
  flex-shrink: 0;
  cursor: pointer;
}

.toggle-switch input {
  opacity: 0;
  width: 0;
  height: 0;
}

.toggle-slider {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: var(--border-color);
  border-radius: 12px;
  transition: background 0.2s;
}

.toggle-slider::before {
  content: '';
  position: absolute;
  width: 18px;
  height: 18px;
  left: 3px;
  bottom: 3px;
  background: white;
  border-radius: 50%;
  transition: transform 0.2s;
}

.toggle-switch input:checked + .toggle-slider {
  background: var(--primary-color);
}

.toggle-switch input:checked + .toggle-slider::before {
  transform: translateX(20px);
}
</style>
