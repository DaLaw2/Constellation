<template>
  <div class="left-panel">
    <div class="panel-tabs">
      <button
        v-if="!sidebarExpanded"
        :class="['tab-btn', { active: currentMode === 'file-browser' }]"
        @click="setMode('file-browser')"
      >
        🗂️ Files
      </button>
      <button
        :class="['tab-btn', { active: currentMode === 'tag-management' }]"
        @click="setMode('tag-management')"
      >
        🏷️ Tags
      </button>
      <button
        :class="['tab-btn', { active: currentMode === 'search' }]"
        @click="setMode('search')"
      >
        🔍 Search
      </button>

      <button
        v-if="currentMode !== 'file-browser'"
        class="toggle-expand-btn"
        :title="sidebarExpanded ? 'Collapse sidebar' : 'Expand sidebar'"
        @click="toggleExpand"
      >
        {{ sidebarExpanded ? '«' : '»' }}
      </button>
    </div>

    <div class="panel-content">
      <div v-if="currentMode === 'file-browser'" class="file-browser-panel">
        <DirectoryTree />
      </div>

      <div v-else-if="currentMode === 'tag-management'" class="tag-management-panel">
        <TagPanel />
      </div>

      <div v-else class="search-panel">
        <FilterPanel />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useAppStore } from '@/stores/app'
import type { ViewMode } from '@/types'
import TagPanel from '../TagManagement/TagPanel.vue'
import DirectoryTree from '../FileExplorer/DirectoryTree.vue'
import FilterPanel from '../Search/FilterPanel.vue'

const appStore = useAppStore()

const currentMode = computed(() => appStore.leftPanelMode)
const sidebarExpanded = computed(() => appStore.sidebarExpanded)

function setMode(mode: ViewMode) {
  appStore.setLeftPanelMode(mode)
}

function toggleExpand() {
  appStore.toggleSidebarExpanded()
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

.toggle-expand-btn {
  padding: 0 12px;
  border: none;
  background: transparent;
  font-size: 16px;
  cursor: pointer;
  color: var(--text-secondary);
  border-bottom: 2px solid transparent;
  display: flex;
  align-items: center;
  justify-content: center;
}

.toggle-expand-btn:hover {
  background: rgba(0, 0, 0, 0.04);
  color: var(--primary-color);
}

.panel-content {
  flex: 1;
  overflow: hidden;
}

.file-browser-panel,
.tag-management-panel,
.search-panel {
  height: 100%;
  overflow: auto;
}
</style>
