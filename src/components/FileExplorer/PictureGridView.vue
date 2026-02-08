<template>
  <div class="picture-grid-view">
    <div v-if="imageFiles.length === 0" class="empty-state">
      <div class="empty-icon">🖼️</div>
      <div class="empty-title">No Media Found</div>
      <div class="empty-description">
        This directory doesn't contain any image or video files.
      </div>
    </div>

    <div v-else class="image-grid">
      <ImageCard
        v-for="(file, index) in imageFiles"
        :key="file.path"
        :file="file"
        :tags="getTagsForFile(file.path)"
        @click="openLightbox(index)"
      />
    </div>

  </div>
</template>

<script setup lang="ts">
import { computed, ref, watch } from 'vue'
import { useFileExplorerStore } from '@/stores/fileExplorer'
import { useLightboxStore } from '@/stores/lightbox'
import { useItemsStore } from '@/stores/items'
import { isMediaFile } from '@/utils'
import ImageCard from './ImageCard.vue'
import type { Tag } from '@/types'

const fileExplorerStore = useFileExplorerStore()
const lightboxStore = useLightboxStore()
const itemsStore = useItemsStore()

// Cache for tags: path -> tags
const tagsCache = ref<Map<string, Tag[]>>(new Map())

const imageFiles = computed(() => {
  return fileExplorerStore.currentFiles.filter(file =>
    !file.is_directory && isMediaFile(file.name)
  )
})

// Batch load items and tags when imageFiles changes
watch(imageFiles, async (files) => {
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
}, { immediate: true })

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
  overflow-y: auto;
  padding: 16px;
}

.image-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
  gap: 16px;
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
