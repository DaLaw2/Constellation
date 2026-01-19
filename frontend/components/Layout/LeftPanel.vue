<template>
  <div class="left-panel">
    <div class="panel-tabs">
      <button
        :class="['tab-btn', { active: currentMode === 'file-browser' }]"
        @click="setMode('file-browser')"
      >
        🗂️ File Browser
      </button>
      <button
        :class="['tab-btn', { active: currentMode === 'tag-management' }]"
        @click="setMode('tag-management')"
      >
        🏷️ Tag Management
      </button>
    </div>

    <div class="panel-content">
      <div v-if="currentMode === 'file-browser'" class="file-browser-panel">
        <DirectoryTree />
      </div>

      <div v-else class="tag-management-panel">
        <TagPanel />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useAppStore } from '../../stores/app'
import type { ViewMode } from '../../stores/app'
import TagPanel from '../TagManagement/TagPanel.vue'
import DirectoryTree from '../FileExplorer/DirectoryTree.vue'

const appStore = useAppStore()

const currentMode = computed(() => appStore.leftPanelMode)

function setMode(mode: ViewMode) {
  appStore.setLeftPanelMode(mode)
}
</script>

<style scoped>
.left-panel {
  width: var(--sidebar-width);
  height: 100%;
  display: flex;
  flex-direction: column;
  border-right: 1px solid var(--border-color);
  background: var(--surface);
}

.panel-tabs {
  display: flex;
  border-bottom: 1px solid var(--border-color);
}

.tab-btn {
  flex: 1;
  padding: 12px 8px;
  border: none;
  background: transparent;
  font-size: 13px;
  cursor: pointer;
  transition: var(--transition-fast);
  border-bottom: 2px solid transparent;
  color: var(--text-secondary);
}

.tab-btn:hover {
  background: rgba(0, 0, 0, 0.04);
  color: var(--text-primary);
}

.tab-btn.active {
  background: var(--background);
  border-bottom-color: var(--primary-color);
  color: var(--primary-color);
  font-weight: 500;
}

.panel-content {
  flex: 1;
  overflow: hidden;
}

.file-browser-panel,
.tag-management-panel {
  height: 100%;
  overflow: auto;
}
</style>
