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
        @click="openLightbox(index)"
      />
    </div>

  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useFileExplorerStore } from '@/stores/fileExplorer'
import { useLightboxStore } from '@/stores/lightbox'
import { isMediaFile } from '@/utils'
import ImageCard from './ImageCard.vue'

const fileExplorerStore = useFileExplorerStore()
const lightboxStore = useLightboxStore()

const imageFiles = computed(() => {
  return fileExplorerStore.currentFiles.filter(file =>
    !file.is_directory && isMediaFile(file.name)
  )
})

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
