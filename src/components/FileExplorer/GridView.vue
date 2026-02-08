<template>
  <div
    class="grid-view"
    :style="gridStyle"
    @wheel.ctrl.prevent="handleZoom"
  >
    <GridFileCard
      v-for="file in files"
      :key="file.path"
      :file="file"
      :zoom-level="zoomLevel"
      :tags="getTagsForFile(file.path)"
      @open="handleOpen"
      @contextmenu="handleContextMenu"
      @tags-updated="refreshTagsCache"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { useItemsStore } from '@/stores/items'
import { useTagsStore } from '@/stores/tags'
import GridFileCard from './GridFileCard.vue'
import type { FileEntry, Tag } from '@/types'

interface Props {
  files: FileEntry[]
}

const props = defineProps<Props>()

const itemsStore = useItemsStore()
const tagsStore = useTagsStore()

// Cache for tags: path -> tags
const tagsCache = ref<Map<string, Tag[]>>(new Map())

async function refreshTagsCache() {
  const files = props.files
  if (files.length === 0) {
    tagsCache.value = new Map()
    return
  }

  const paths = files.map(f => f.path)

  // Batch fetch items by paths
  const items = await itemsStore.getItemsByPaths(paths)

  if (items.length === 0) {
    tagsCache.value = new Map()
    return
  }

  // Create path -> item ID map
  const pathToId = new Map(items.map(item => [item.path, item.id]))

  // Batch fetch tags for all items
  const itemIds = items.map(item => item.id)
  const tagsMap = await itemsStore.getTagsForItems(itemIds)

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

function getTagsForFile(path: string): Tag[] {
  return tagsCache.value.get(path) || []
}

const emit = defineEmits<{
  open: [file: FileEntry]
  contextmenu: [event: MouseEvent, file: FileEntry]
}>()

// Zoom level: 50-300 (percentage)
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

function handleOpen(file: FileEntry) {
  emit('open', file)
}

function handleContextMenu(event: MouseEvent, file: FileEntry) {
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
  display: grid;
  grid-auto-rows: min-content;
  padding: 16px;
  height: 100%;
  overflow-y: auto;
}
</style>
