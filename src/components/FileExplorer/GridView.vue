<template>
  <div
    ref="containerRef"
    class="grid-view"
    @wheel.ctrl.prevent="handleZoom"
    @click.self="handleBackgroundClick"
  >
    <RecycleScroller
      class="grid-scroller"
      :items="rows"
      :item-size="rowHeight"
      key-field="id"
      v-slot="{ item: row }"
      @click.self="handleBackgroundClick"
    >
      <div class="grid-row" :style="rowStyle">
        <GridFileCard
          v-for="file in row.items"
          :key="file.path"
          :file="file"
          :zoom-level="zoomLevel"
          :tags="getTagsForFile(file.path)"
          :selected="selectedPaths.has(file.path)"
          @click="handleFileClick"
          @open="handleOpen"
          @contextmenu="handleContextMenu"
        />
      </div>
    </RecycleScroller>

    <!-- Batch tag action bar -->
    <BatchTagActionBar
      :selected-paths="selectedPaths"
      :selected-count="selectedCount"
      @clear="clearSelection"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { RecycleScroller } from 'vue-virtual-scroller'
import 'vue-virtual-scroller/dist/vue-virtual-scroller.css'
import { useItemsStore } from '@/stores/items'
import { useTagsStore } from '@/stores/tags'
import { useGridVirtualScroll } from '@/composables'
import { LAYOUT } from '@/constants'
import GridFileCard from './GridFileCard.vue'
import BatchTagActionBar from './BatchTagActionBar.vue'
import type { FileEntry, Tag } from '@/types'

interface Props {
  files: FileEntry[]
}

const props = defineProps<Props>()

const containerRef = ref<HTMLElement | null>(null)

// Zoom level: 50-300 (percentage)
const zoomLevel = ref(100)
const MIN_ZOOM = 50
const MAX_ZOOM = 300

// Compute dynamic card size and gap based on zoom
const cardSize = computed(() => Math.floor(LAYOUT.GRID_MIN_CARD_WIDTH * (zoomLevel.value / 100)))
const gap = computed(() => Math.floor(LAYOUT.GRID_GAP * (zoomLevel.value / 100)))

// Virtual scroll row grouping (pass computed refs for zoom reactivity)
const filesRef = computed(() => props.files)
const { rows, columnCount } = useGridVirtualScroll(filesRef, {
  minCardWidth: cardSize,
  gap: gap,
  containerRef,
})

// Row height adapts to zoom level
const rowHeight = computed(() => Math.floor(LAYOUT.GRID_ROW_HEIGHT * (zoomLevel.value / 100)))

const rowStyle = computed(() => ({
  display: 'grid',
  gridTemplateColumns: `repeat(${columnCount.value}, 1fr)`,
  gap: `${gap.value}px`,
  padding: '0 16px',
  height: `${rowHeight.value}px`,
  alignContent: 'start',
}))

// Multi-selection state
const selectedPaths = ref<Set<string>>(new Set())
const lastClickedIndex = ref<number | null>(null)
const selectedCount = computed(() => selectedPaths.value.size)

function toggleSelection(path: string) {
  const newSet = new Set(selectedPaths.value)
  if (newSet.has(path)) {
    newSet.delete(path)
  } else {
    newSet.add(path)
  }
  selectedPaths.value = newSet
}

function selectRange(startIndex: number, endIndex: number, addToExisting: boolean = false) {
  const start = Math.min(startIndex, endIndex)
  const end = Math.max(startIndex, endIndex)
  // If not adding to existing, start fresh
  const newSet = addToExisting ? new Set(selectedPaths.value) : new Set<string>()

  for (let i = start; i <= end; i++) {
    if (props.files[i]) {
      newSet.add(props.files[i].path)
    }
  }
  selectedPaths.value = newSet
}

function clearSelection() {
  selectedPaths.value = new Set()
  lastClickedIndex.value = null
}

function handleFileClick(file: FileEntry, event: MouseEvent) {
  const index = props.files.findIndex(f => f.path === file.path)

  if (event.shiftKey && lastClickedIndex.value !== null) {
    // Shift+Click: Range selection from anchor
    // Ctrl+Shift: Add range to existing selection
    selectRange(lastClickedIndex.value, index, event.ctrlKey || event.metaKey)
    // Don't update lastClickedIndex - keep the anchor point
    return
  }

  if (event.ctrlKey || event.metaKey) {
    // Ctrl+Click: Toggle individual selection
    toggleSelection(file.path)
  } else {
    // Regular click: Single selection (clear others)
    selectedPaths.value = new Set([file.path])
  }

  // Update anchor point for Shift+Click
  lastClickedIndex.value = index
}

function handleBackgroundClick() {
  clearSelection()
}

// Clear selection when files change (navigation)
watch(() => props.files, () => {
  clearSelection()
})

const itemsStore = useItemsStore()
const tagsStore = useTagsStore()

// Cache for tags: path -> tags
const tagsCache = ref<Map<string, Tag[]>>(new Map())

// Request counter to prevent race conditions
let requestId = 0

async function refreshTagsCache() {
  const currentRequestId = ++requestId
  const files = props.files

  if (files.length === 0) {
    tagsCache.value = new Map()
    return
  }

  const paths = files.map(f => f.path)

  // Batch fetch items by paths
  const items = await itemsStore.getItemsByPaths(paths)

  // Check if this request is still the latest
  if (currentRequestId !== requestId) return

  if (items.length === 0) {
    tagsCache.value = new Map()
    return
  }

  // Create path -> item ID map
  const pathToId = new Map(items.map(item => [item.path, item.id]))

  // Batch fetch tags for all items
  const itemIds = items.map(item => item.id)
  const tagsMap = await itemsStore.getTagsForItems(itemIds)

  // Check if this request is still the latest
  if (currentRequestId !== requestId) return

  // Build path -> tags cache
  const newCache = new Map<string, Tag[]>()
  for (const [path, itemId] of pathToId) {
    newCache.set(path, tagsMap[itemId] || [])
  }
  tagsCache.value = newCache
}

// Batch load tags when files change
watch(() => props.files, refreshTagsCache, { immediate: true })

// Refresh cache when item-tag associations change
watch(() => tagsStore.itemTagsVersion, refreshTagsCache)

// Refresh cache when tag metadata (name, color, group) changes
watch(() => tagsStore.tags, refreshTagsCache, { deep: true })

function getTagsForFile(path: string): Tag[] {
  return tagsCache.value.get(path) || []
}

const emit = defineEmits<{
  open: [file: FileEntry]
  contextmenu: [event: MouseEvent, file: FileEntry]
}>()

function handleOpen(file: FileEntry) {
  clearSelection()
  emit('open', file)
}

function handleContextMenu(event: MouseEvent, file: FileEntry) {
  // If right-clicked file is not in selection, select only it
  if (!selectedPaths.value.has(file.path)) {
    selectedPaths.value = new Set([file.path])
  }
  emit('contextmenu', event, file)
}

function handleZoom(event: WheelEvent) {
  const delta = event.deltaY > 0 ? -10 : 10
  const newZoom = Math.max(MIN_ZOOM, Math.min(MAX_ZOOM, zoomLevel.value + delta))
  zoomLevel.value = newZoom
}
</script>

<style scoped>
.grid-view {
  height: 100%;
  overflow: hidden;
  padding-top: 16px;
}

.grid-scroller {
  height: 100%;
}

.grid-row {
  box-sizing: border-box;
  overflow: hidden;
}
</style>
