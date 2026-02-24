<template>
  <div ref="containerRef" class="picture-grid-view">
    <div v-if="imageFiles.length === 0" class="empty-state">
      <div class="empty-icon">🖼️</div>
      <div class="empty-title">No Media Found</div>
      <div class="empty-description">
        This directory doesn't contain any image or video files.
      </div>
    </div>

    <RecycleScroller
      v-else
      class="image-grid-scroller"
      :items="rows"
      :item-size="LAYOUT.PICTURE_ROW_HEIGHT"
      key-field="id"
      v-slot="{ item: row }"
    >
      <div class="image-row" :style="rowStyle">
        <ImageCard
          v-for="(file, localIndex) in row.items"
          :key="file.path"
          :file="file"
          :tags="getTagsForFile(file.path)"
          @click="openLightbox(row.startIndex + localIndex)"
        />
      </div>
    </RecycleScroller>

  </div>
</template>

<script setup lang="ts">
import { computed, ref, watch } from 'vue'
import { RecycleScroller } from 'vue-virtual-scroller'
import 'vue-virtual-scroller/dist/vue-virtual-scroller.css'
import { useFileExplorerStore } from '@/stores/fileExplorer'
import { useLightboxStore } from '@/stores/lightbox'
import { useItemsStore } from '@/stores/items'
import { useTagsStore } from '@/stores/tags'
import { isMediaFile } from '@/utils'
import { useGridVirtualScroll } from '@/composables'
import { LAYOUT } from '@/constants'
import ImageCard from './ImageCard.vue'
import type { Tag } from '@/types'

const fileExplorerStore = useFileExplorerStore()
const lightboxStore = useLightboxStore()
const itemsStore = useItemsStore()
const tagsStore = useTagsStore()

const containerRef = ref<HTMLElement | null>(null)

// Cache for tags: path -> tags
const tagsCache = ref<Map<string, Tag[]>>(new Map())

// Request counter to prevent race conditions
let requestId = 0

const imageFiles = computed(() => {
  return fileExplorerStore.currentFiles.filter(file =>
    !file.is_directory && isMediaFile(file.name)
  )
})

// Virtual scroll row grouping
const { rows, columnCount } = useGridVirtualScroll(imageFiles, {
  minCardWidth: LAYOUT.PICTURE_MIN_CARD_WIDTH,
  gap: LAYOUT.GRID_GAP,
  containerRef,
})

const rowStyle = computed(() => ({
  display: 'grid',
  gridTemplateColumns: `repeat(${columnCount.value}, 1fr)`,
  gap: `${LAYOUT.GRID_GAP}px`,
  height: `${LAYOUT.PICTURE_ROW_HEIGHT}px`,
  alignContent: 'start',
}))

async function refreshTagsCache() {
  const currentRequestId = ++requestId
  const files = imageFiles.value

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

// Batch load items and tags when imageFiles changes
watch(imageFiles, refreshTagsCache, { immediate: true })

// Refresh cache when item-tag associations change (e.g., tag deleted/merged)
watch(() => tagsStore.itemTagsVersion, refreshTagsCache)

function getTagsForFile(path: string): Tag[] {
  return tagsCache.value.get(path) || []
}

function openLightbox(index: number) {
  lightboxStore.open(imageFiles.value, index)
}
</script>

<style scoped>
.picture-grid-view {
  height: 100%;
  overflow: hidden;
  padding-top: 16px;
}

.image-grid-scroller {
  height: 100%;
}

.image-row {
  box-sizing: border-box;
  overflow: hidden;
  padding: 0 16px;
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  color: var(--text-secondary);
}

.empty-icon {
  font-size: 64px;
  margin-bottom: 16px;
  opacity: 0.5;
}

.empty-title {
  font-size: 18px;
  font-weight: 600;
  margin-bottom: 8px;
  color: var(--text-primary);
}

.empty-description {
  font-size: 14px;
  text-align: center;
  max-width: 400px;
}
</style>
