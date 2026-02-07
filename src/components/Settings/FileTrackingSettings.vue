<template>
  <div class="file-tracking-settings">
    <!-- Auto refresh on startup -->
    <div class="setting-row">
      <div class="setting-info">
        <label class="setting-label">Auto refresh on startup</label>
        <p class="setting-desc">Automatically scan USN journal when the app starts</p>
      </div>
      <label class="toggle-switch">
        <input
          type="checkbox"
          :checked="settings.usn_auto_refresh"
          @change="toggle('usn_auto_refresh', ($event.target as HTMLInputElement).checked)"
        />
        <span class="toggle-slider"></span>
      </label>
    </div>

    <!-- Refresh interval -->
    <div class="setting-row">
      <div class="setting-info">
        <label class="setting-label">Refresh interval</label>
        <p class="setting-desc">Periodically scan for file changes</p>
      </div>
      <select
        class="setting-select"
        :value="settings.usn_refresh_interval"
        @change="updateInterval(($event.target as HTMLSelectElement).value)"
      >
        <option value="0">Disabled</option>
        <option value="5">5 minutes</option>
        <option value="15">15 minutes</option>
        <option value="30">30 minutes</option>
        <option value="60">1 hour</option>
      </select>
    </div>

    <!-- Refresh on missing -->
    <div class="setting-row">
      <div class="setting-info">
        <label class="setting-label">Auto refresh on missing files</label>
        <p class="setting-desc">Trigger scan when a tracked file path is no longer valid</p>
      </div>
      <label class="toggle-switch">
        <input
          type="checkbox"
          :checked="settings.usn_refresh_on_missing"
          @change="toggle('usn_refresh_on_missing', ($event.target as HTMLInputElement).checked)"
        />
        <span class="toggle-slider"></span>
      </label>
    </div>

    <!-- Cross volume match -->
    <div class="setting-row">
      <div class="setting-info">
        <label class="setting-label">Cross-volume matching</label>
        <p class="setting-desc">Attempt to match files moved between drives</p>
      </div>
      <label class="toggle-switch">
        <input
          type="checkbox"
          :checked="settings.usn_cross_volume_match"
          @change="toggle('usn_cross_volume_match', ($event.target as HTMLInputElement).checked)"
        />
        <span class="toggle-slider"></span>
      </label>
    </div>

    <!-- Manual refresh button -->
    <div class="setting-row action-row">
      <div class="setting-info">
        <label class="setting-label">Manual refresh</label>
        <p class="setting-desc">Scan all volumes for file changes now</p>
      </div>
      <button class="btn-refresh" disabled title="USN tracking not yet implemented">
        Refresh Now
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useSettingsStore } from '@/stores/settings'

const settingsStore = useSettingsStore()
const settings = computed(() => settingsStore.settings)

function toggle(key: string, checked: boolean) {
  settingsStore.updateSetting(key, String(checked))
}

function updateInterval(value: string) {
  settingsStore.updateSetting('usn_refresh_interval', value)
}
</script>

<style scoped>
.file-tracking-settings {
  display: flex;
  flex-direction: column;
  gap: 0;
}

.setting-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px 0;
  border-bottom: 1px solid var(--border-color);
}

.setting-row:last-child {
  border-bottom: none;
}

.setting-info {
  flex: 1;
  min-width: 0;
}

.setting-label {
  display: block;
  font-size: 14px;
  font-weight: 500;
  color: var(--text-primary);
  margin-bottom: 2px;
}

.setting-desc {
  margin: 0;
  font-size: 12px;
  color: var(--text-secondary);
}

/* Toggle Switch */
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

/* Select */
.setting-select {
  padding: 6px 12px;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  font-size: 13px;
  background: var(--background);
  color: var(--text-primary);
  cursor: pointer;
  flex-shrink: 0;
}

.setting-select:focus {
  outline: none;
  border-color: var(--primary-color);
}

/* Refresh Button */
.btn-refresh {
  padding: 8px 16px;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background: var(--background);
  font-size: 13px;
  font-weight: 500;
  color: var(--text-primary);
  cursor: pointer;
  flex-shrink: 0;
  transition: var(--transition-fast);
}

.btn-refresh:hover:not(:disabled) {
  border-color: var(--primary-color);
  color: var(--primary-color);
}

.btn-refresh:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}
</style>
