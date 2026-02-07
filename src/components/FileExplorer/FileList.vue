<template>
  <div class="file-list">
    <!-- Header with current path and navigation -->
    <div v-if="currentPath" class="file-list-header">
      <button class="nav-btn" @click="navigateUp" :disabled="!canNavigateUp">
        ⬆️ Up
      </button>
      <div class="current-path">{{ currentPath }}</div>
      <button class="refresh-btn" @click="refresh" title="Refresh">
        <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <path d="M21.5 2v6h-6"></path>
          <path d="M2.5 22v-6h6"></path>
          <path d="M2 11.5a10 10 0 0 1 18.8-4.3"></path>
          <path d="M22 12.5a10 10 0 0 1-18.8 4.3"></path>
        </svg>
      </button>
    </div>

    <!-- Loading state -->
    <div v-if="loading" class="loading-state">
      <div class="loading-spinner"></div>
      <div>Loading...</div>
    </div>

    <!-- Empty state -->
    <div v-else-if="!currentPath" class="empty-state">
      <div class="empty-state-icon">
        <svg xmlns="http://www.w3.org/2000/svg" width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1" stroke-linecap="round" stroke-linejoin="round">
          <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"></path>
        </svg>
      </div>
      <div class="empty-state-title">No Directory Selected</div>
      <div class="empty-state-description">
        Select a drive or directory from the tree on the left
      </div>
    </div>

    <!-- Empty directory -->
    <div v-else-if="files.length === 0" class="empty-state">
      <div class="empty-state-icon">
         <svg xmlns="http://www.w3.org/2000/svg" width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1" stroke-linecap="round" stroke-linejoin="round">
          <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"></path>
          <line x1="9" y1="14" x2="15" y2="14"></line>
        </svg>
      </div>
      <div class="empty-state-title">Empty Directory</div>
      <div class="empty-state-description">
        This directory contains no visible files or folders
      </div>
    </div>

    <!-- Detail view: Virtual scrolling file list -->
    <RecycleScroller
      v-else-if="displayMode === 'detail'"
      class="file-scroller"
      :items="filteredFiles"
      :item-size="LAYOUT.FILE_ITEM_HEIGHT"
      key-field="path"
      v-slot="{ item }"
    >
      <FileItem
        :entry="item"
        :selected="selectedPath === item.path"
        :tag-area-width="tagAreaWidth"
        :highlight-query="appStore.searchQuery"
        @click="handleFileClick"
        @double-click="handleFileDoubleClick"
        @context-menu="handleFileContextMenu"
        @resize-start="handleResizeStart"
      />
    </RecycleScroller>

    <!-- Grid view: Large icons mode -->
    <GridView
      v-else
      :files="filteredFiles"
      @open="handleFileDoubleClick"
      @contextmenu="(event, file) => handleFileContextMenu(file, event)"
    />

    <!-- Error state -->
    <div v-if="error" class="error-toast">
      {{ error }}
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref, onMounted } from 'vue'
import { RecycleScroller } from 'vue-virtual-scroller'
import { useFileExplorerStore } from '@/stores/fileExplorer'
import { useAppStore } from '@/stores/app'
import { useTagsStore } from '@/stores/tags'
import { useFileContextMenu, useResizable, useLocalStorage } from '@/composables'
import { fuzzyMatch } from '@/utils'
import { LAYOUT, STORAGE_KEYS } from '@/constants'
import FileItem from './FileItem.vue'
import GridView from './GridView.vue'
import type { FileEntry } from '@/types'
import 'vue-virtual-scroller/dist/vue-virtual-scroller.css'

const fileExplorerStore = useFileExplorerStore()
const appStore = useAppStore()
const tagsStore = useTagsStore()
const { showFileContextMenu } = useFileContextMenu()

// Preload tag data on mount for better performance
onMounted(() => {
  if (tagsStore.tagGroups.length === 0) {
    tagsStore.loadTagGroups()
  }
  if (tagsStore.tags.length === 0) {
    tagsStore.loadTags()
  }
})

const currentPath = computed(() => fileExplorerStore.currentPath)
const files = computed(() => fileExplorerStore.currentFiles)
const loading = computed(() => fileExplorerStore.loading)
const error = computed(() => fileExplorerStore.error)
const displayMode = computed(() => appStore.displayMode)

const filteredFiles = computed(() => {
  const query = appStore.searchQuery.trim()
  if (!query) return files.value

  return files.value.filter(file => fuzzyMatch(file.name, query))
})

// Local selection state
const selectedPath = ref<string | null>(null)

// Shared tag area width for all file items with persistence
const savedTagWidth = useLocalStorage<number>(STORAGE_KEYS.TAG_AREA_WIDTH, LAYOUT.DEFAULT_TAG_AREA_WIDTH)

const { width: tagAreaWidth, handleResizeStart } = useResizable(savedTagWidth.value, {
  minWidth: LAYOUT.MIN_TAG_AREA_WIDTH,
  onResizeEnd: (width) => {
    savedTagWidth.value = width
  },
})

const canNavigateUp = computed(() => {
  if (!currentPath.value) return false
  const pathParts = currentPath.value.split('\\').filter(Boolean)
  return pathParts.length > 1
})

function navigateUp() {
  fileExplorerStore.navigateUp()
  selectedPath.value = null
}

function refresh() {
  if (currentPath.value) {
    fileExplorerStore.readDirectory(currentPath.value)
    selectedPath.value = null
  }
}

function handleFileClick(entry: FileEntry) {
  selectedPath.value = entry.path
}

function handleFileDoubleClick(entry: FileEntry) {
  if (entry.is_directory) {
    fileExplorerStore.navigateTo(entry.path)
    selectedPath.value = null
  } else {
    fileExplorerStore.openFileExternal(entry.path)
  }
}

function handleFileContextMenu(entry: FileEntry, event: MouseEvent) {
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
