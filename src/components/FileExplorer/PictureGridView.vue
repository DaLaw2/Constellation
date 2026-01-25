<template>
  <div class="picture-grid-view">
    <div v-if="imageFiles.length === 0" class="empty-state">
      <div class="empty-icon">🖼️</div>
      <div class="empty-title">No Images Found</div>
      <div class="empty-description">
        This directory doesn't contain any image files.
      </div>
    </div>

    <div v-else class="image-grid">
      <ImageCard
        v-for="(file, index) in imageFiles"
        :key="file.path"
        :file="file"
        @click="openLightbox(index)"
      />
    </div>

    <ImageLightbox />
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useFileExplorerStore } from '@/stores/fileExplorer'
import type { FileEntry } from '@/types'
import ImageCard from './ImageCard.vue'
import ImageLightbox from './ImageLightbox.vue'

const fileExplorerStore = useFileExplorerStore()

const imageExtensions = ['.jpg', '.jpeg', '.png', '.gif', '.webp', '.bmp', '.svg']

const imageFiles = computed(() => {
  return fileExplorerStore.currentFiles.filter((file: FileEntry) => 
    !file.is_directory && 
    imageExtensions.some(ext => file.name.toLowerCase().endsWith(ext))
  )
})

function openLightbox(index: number) {
  fileExplorerStore.openLightbox(index)
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
