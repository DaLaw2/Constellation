<template>
  <div class="file-list">
    <!-- Header with current path and navigation -->
    <div v-if="currentPath" class="file-list-header">
      <button class="nav-btn" @click="navigateUp" :disabled="!canNavigateUp">
        ⬆️ Up
      </button>
      <div class="current-path">{{ currentPath }}</div>
      <button class="refresh-btn" @click="refresh">
        🔄
      </button>
    </div>

    <!-- Loading state -->
    <div v-if="loading" class="loading-state">
      <div class="loading-spinner"></div>
      <div>Loading...</div>
    </div>

    <!-- Empty state -->
    <div v-else-if="!currentPath" class="empty-state">
      <div class="empty-state-icon">📂</div>
      <div class="empty-state-title">No Directory Selected</div>
      <div class="empty-state-description">
        Select a drive or directory from the tree on the left
      </div>
    </div>

    <!-- Empty directory -->
    <div v-else-if="files.length === 0" class="empty-state">
      <div class="empty-state-icon">📭</div>
      <div class="empty-state-title">Empty Directory</div>
      <div class="empty-state-description">
        This directory contains no visible files or folders
      </div>
    </div>

    <!-- Virtual scrolling file list -->
    <RecycleScroller
      v-else
      class="file-scroller"
      :items="files"
      :item-size="60"
      key-field="path"
      v-slot="{ item }"
    >
      <FileItem
        :entry="item"
        :selected="selectedPath === item.path"
        @click="handleFileClick"
        @double-click="handleFileDoubleClick"
        @context-menu="handleFileContextMenu"
      />
    </RecycleScroller>

    <!-- Error state -->
    <div v-if="error" class="error-toast">
      {{ error }}
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'
import { RecycleScroller } from 'vue-virtual-scroller'
import { useFileExplorerStore } from '../../stores/fileExplorer'
import { useFileContextMenu } from '../../composables/useFileContextMenu'
import FileItem from './FileItem.vue'
import type { FileEntry } from '../../stores/fileExplorer'
import 'vue-virtual-scroller/dist/vue-virtual-scroller.css'

const fileExplorerStore = useFileExplorerStore()
const { showFileContextMenu } = useFileContextMenu()

const currentPath = computed(() => fileExplorerStore.currentPath)
const files = computed(() => fileExplorerStore.currentFiles)
const loading = computed(() => fileExplorerStore.loading)
const error = computed(() => fileExplorerStore.error)

// Local selection state (writable ref, not computed)
// Selection is a UI concern separate from navigation
const selectedPath = ref<string | null>(null)

const canNavigateUp = computed(() => {
  if (!currentPath.value) return false
  const pathParts = currentPath.value.split('\\').filter(Boolean)
  return pathParts.length > 1
})

function navigateUp() {
  fileExplorerStore.navigateUp()
  selectedPath.value = null // Clear selection on navigation
}

function refresh() {
  if (currentPath.value) {
    fileExplorerStore.readDirectory(currentPath.value)
    selectedPath.value = null // Clear selection on refresh
  }
}

function handleFileClick(entry: FileEntry) {
  // Single click - update local selection state
  selectedPath.value = entry.path
}

function handleFileDoubleClick(entry: FileEntry) {
  if (entry.is_directory) {
    // Navigate into directory
    fileExplorerStore.navigateTo(entry.path)
    selectedPath.value = null // Clear selection on navigation
  } else {
    // Open file with default application
    fileExplorerStore.openFileExternal(entry.path)
  }
}

function handleFileContextMenu(entry: FileEntry, event: MouseEvent) {
  // Select file before showing context menu
  selectedPath.value = entry.path

  showFileContextMenu({
    entry,
    x: event.x,
    y: event.y,
  })
}
</script>

<style scoped>
.file-list {
  height: 100%;
  display: flex;
  flex-direction: column;
  background: var(--background);
  overflow: hidden;
}

.file-list-header {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px;
  border-bottom: 1px solid var(--border-color);
  background: var(--surface);
}

.nav-btn,
.refresh-btn {
  padding: 6px 12px;
  border: 1px solid var(--border-color);
  background: var(--background);
  border-radius: 4px;
  cursor: pointer;
  font-size: 13px;
  transition: var(--transition-fast);
}

.nav-btn:hover:not(:disabled),
.refresh-btn:hover {
  background: rgba(0, 0, 0, 0.04);
}

.nav-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.refresh-btn {
  padding: 6px 10px;
}

.current-path {
  flex: 1;
  font-size: 13px;
  color: var(--text-secondary);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  padding: 0 8px;
}

.file-scroller {
  flex: 1;
  overflow-y: auto;
}

.loading-state {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 12px;
  color: var(--text-secondary);
  font-size: 13px;
}

.loading-spinner {
  width: 32px;
  height: 32px;
  border: 3px solid var(--border-color);
  border-top-color: var(--primary-color);
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

.empty-state {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 12px;
  padding: 48px 24px;
  color: var(--text-secondary);
}

.empty-state-icon {
  font-size: 64px;
  opacity: 0.5;
}

.empty-state-title {
  font-size: 18px;
  font-weight: 500;
  color: var(--text-primary);
}

.empty-state-description {
  font-size: 14px;
  text-align: center;
  max-width: 300px;
}

.error-toast {
  position: absolute;
  bottom: 16px;
  left: 50%;
  transform: translateX(-50%);
  padding: 12px 24px;
  background: #ef4444;
  color: white;
  border-radius: 6px;
  font-size: 13px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
  z-index: 100;
}
</style>
