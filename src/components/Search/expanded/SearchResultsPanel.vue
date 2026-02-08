<template>
  <div class="search-results-panel">
    <!-- Results header -->
    <div class="results-header">
      <span class="results-count">
        {{ filteredResults.length }} {{ filteredResults.length === 1 ? 'item' : 'items' }} found
      </span>
      <div class="header-controls">
        <div class="sort-controls">
          <label class="sort-label">Sort by:</label>
          <select v-model="sortBy" class="sort-select">
            <option value="name">Name</option>
            <option value="date">Date</option>
            <option value="size">Size</option>
          </select>
          <button class="sort-order-btn" @click="toggleSortOrder" :title="sortOrder === 'asc' ? 'Ascending' : 'Descending'">
            {{ sortOrder === 'asc' ? '↑' : '↓' }}
          </button>
        </div>
        <div class="view-toggle">
          <button
            :class="['view-btn', { active: displayMode === 'detail' }]"
            @click="displayMode = 'detail'"
            title="Detail View"
          >
            <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <line x1="8" y1="6" x2="21" y2="6"></line>
              <line x1="8" y1="12" x2="21" y2="12"></line>
              <line x1="8" y1="18" x2="21" y2="18"></line>
              <line x1="3" y1="6" x2="3.01" y2="6"></line>
              <line x1="3" y1="12" x2="3.01" y2="12"></line>
              <line x1="3" y1="18" x2="3.01" y2="18"></line>
            </svg>
          </button>
          <button
            :class="['view-btn', { active: displayMode === 'grid' }]"
            @click="displayMode = 'grid'"
            title="Large Icons"
          >
            <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <rect x="3" y="3" width="7" height="7"></rect>
              <rect x="14" y="3" width="7" height="7"></rect>
              <rect x="14" y="14" width="7" height="7"></rect>
              <rect x="3" y="14" width="7" height="7"></rect>
            </svg>
          </button>
        </div>
      </div>
    </div>

    <!-- Loading state -->
    <div v-if="loading" class="state-message">Searching...</div>

    <!-- Error state -->
    <div v-else-if="error" class="state-message error">Error: {{ error }}</div>

    <!-- Empty state -->
    <div v-else-if="filteredResults.length === 0 && hasSearched" class="state-message">
      No results found
    </div>

    <!-- Results list (Detail View) -->
    <div v-else-if="displayMode === 'detail'" class="results-list">
      <div
        v-for="item in sortedResults"
        :key="item.id"
        class="result-item"
        @click="openItem(item)"
      >
        <span class="result-icon">{{ item.is_directory ? '📁' : '📄' }}</span>
        <div class="result-info">
          <span class="result-name">{{ getFileName(item.path) }}</span>
          <span class="result-path">{{ item.path }}</span>
        </div>
        <div class="result-meta">
          <span v-if="item.size !== null && item.size !== undefined" class="result-size">
            {{ formatBytes(item.size) }}
          </span>
          <span v-if="item.modified_time" class="result-date">
            {{ formatDate(item.modified_time) }}
          </span>
        </div>
      </div>
    </div>

    <!-- Results grid (Large Icons View) -->
    <div
      v-else
      class="results-grid"
      :style="gridStyle"
      @wheel.ctrl.prevent="handleZoom"
    >
      <GridFileCard
        v-for="item in sortedResults"
        :key="item.id"
        :file="toFileEntry(item)"
        :zoom-level="zoomLevel"
        :tags="[]"
        :show-tags="false"
        @click="handleCardClick"
        @open="handleCardOpen"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useFileExplorerStore } from '@/stores/fileExplorer'
import { useAppStore } from '@/stores/app'
import { getFileName, getParentPath } from '@/utils/path'
import { formatBytes, formatDate } from '@/utils/format'
import GridFileCard from '@/components/FileExplorer/GridFileCard.vue'
import type { Item, FileEntry } from '@/types'

interface Props {
  results: Item[]
  loading: boolean
  error: string | null
  hasSearched: boolean
  clientFilter: ((items: Item[]) => Item[]) | null
}

const props = defineProps<Props>()

const fileExplorerStore = useFileExplorerStore()
const appStore = useAppStore()

const sortBy = ref<'name' | 'date' | 'size'>('name')
const sortOrder = ref<'asc' | 'desc'>('asc')
const displayMode = ref<'detail' | 'grid'>('detail')

const filteredResults = computed(() => {
  if (props.clientFilter) {
    return props.clientFilter(props.results)
  }
  return props.results
})

const sortedResults = computed(() => {
  const items = [...filteredResults.value]
  const order = sortOrder.value === 'asc' ? 1 : -1

  items.sort((a, b) => {
    switch (sortBy.value) {
      case 'name': {
        const nameA = getFileName(a.path).toLowerCase()
        const nameB = getFileName(b.path).toLowerCase()
        return nameA.localeCompare(nameB) * order
      }
      case 'date': {
        const dateA = a.modified_time || 0
        const dateB = b.modified_time || 0
        return (dateA - dateB) * order
      }
      case 'size': {
        const sizeA = a.size || 0
        const sizeB = b.size || 0
        return (sizeA - sizeB) * order
      }
      default:
        return 0
    }
  })

  return items
})

function toggleSortOrder() {
  sortOrder.value = sortOrder.value === 'asc' ? 'desc' : 'asc'
}

function openItem(item: Item) {
  const dirPath = item.is_directory ? item.path : getParentPath(item.path)
  fileExplorerStore.navigateTo(dirPath)
  // Collapse sidebar so user can see the file in File Browser
  appStore.setSidebarExpanded(false)
}

// Grid view support
const zoomLevel = ref(100)
const MIN_ZOOM = 50
const MAX_ZOOM = 300

const gridStyle = computed(() => {
  const baseSize = 150
  const baseGap = 16
  const cardSize = Math.floor(baseSize * (zoomLevel.value / 100))
  const gap = Math.floor(baseGap * (zoomLevel.value / 100))

  return {
    gridTemplateColumns: `repeat(auto-fill, minmax(${cardSize}px, 1fr))`,
    gap: `${gap}px`
  }
})

function toFileEntry(item: Item): FileEntry {
  return {
    name: getFileName(item.path),
    path: item.path,
    is_directory: item.is_directory,
    size: item.size ?? 0,
    modified_time: item.modified_time ?? 0,
    is_hidden: false
  }
}

function handleCardClick(_file: FileEntry) {
  // Single click - no action in search results
}

function handleCardOpen(file: FileEntry) {
  const item = sortedResults.value.find(i => i.path === file.path)
  if (item) {
    openItem(item)
  }
}

function handleZoom(event: WheelEvent) {
  const delta = event.deltaY > 0 ? -10 : 10
  const newZoom = Math.max(MIN_ZOOM, Math.min(MAX_ZOOM, zoomLevel.value + delta))
  zoomLevel.value = newZoom
}
</script>

<style scoped>
.search-results-panel {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.results-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 20px;
  border-bottom: 1px solid var(--border-color);
  background: var(--background);
  flex-shrink: 0;
}

.results-count {
  font-size: 13px;
  font-weight: 500;
  color: var(--text-secondary);
}

.header-controls {
  display: flex;
  align-items: center;
  gap: 16px;
}

.sort-controls {
  display: flex;
  align-items: center;
  gap: 6px;
}

.view-toggle {
  display: flex;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  overflow: hidden;
}

.view-btn {
  padding: 5px 8px;
  border: none;
  background: var(--surface);
  cursor: pointer;
  color: var(--text-secondary);
  display: flex;
  align-items: center;
  justify-content: center;
  transition: var(--transition-fast);
}

.view-btn:first-child {
  border-right: 1px solid var(--border-color);
}

.view-btn:hover:not(.active) {
  background: rgba(0, 0, 0, 0.04);
}

.view-btn.active {
  background: var(--primary-color);
  color: white;
}

.sort-label {
  font-size: 12px;
  color: var(--text-secondary);
}

.sort-select {
  padding: 4px 8px;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  font-size: 12px;
  background: var(--surface);
  cursor: pointer;
}

.sort-select:focus {
  outline: none;
  border-color: var(--primary-color);
}

.sort-order-btn {
  padding: 4px 8px;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background: var(--surface);
  cursor: pointer;
  font-size: 14px;
  line-height: 1;
  transition: var(--transition-fast);
}

.sort-order-btn:hover {
  border-color: var(--primary-color);
  color: var(--primary-color);
}

.state-message {
  padding: 32px;
  text-align: center;
  font-size: 13px;
  color: var(--text-secondary);
}

.state-message.error {
  color: #d32f2f;
}

.results-list {
  flex: 1;
  overflow-y: auto;
  padding: 8px;
}

.result-item {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 10px 12px;
  border-radius: 4px;
  cursor: pointer;
  transition: var(--transition-fast);
}

.result-item:hover {
  background: rgba(0, 0, 0, 0.04);
}

.result-icon {
  flex-shrink: 0;
  font-size: 16px;
}

.result-info {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.result-name {
  font-size: 13px;
  font-weight: 500;
  color: var(--text-primary);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.result-path {
  font-size: 11px;
  color: var(--text-secondary);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.result-meta {
  display: flex;
  flex-direction: column;
  align-items: flex-end;
  gap: 2px;
  flex-shrink: 0;
}

.result-size,
.result-date {
  font-size: 11px;
  color: var(--text-secondary);
  white-space: nowrap;
}

/* Grid View */
.results-grid {
  display: grid;
  grid-auto-rows: min-content;
  padding: 16px;
  flex: 1;
  overflow-y: auto;
}
</style>
