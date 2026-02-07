<template>
  <div class="picture-grid-container">
    <div v-if="!hasImages" class="empty-state">
      <div class="empty-icon">🖼️</div>
      <div class="empty-title">No Media Found</div>
      <div class="empty-description">
        This directory doesn't contain any image or video files.
      </div>
    </div>

    <div v-else class="picture-grid" :style="gridStyle">
      <PictureCard
        v-for="(image, index) in currentImages"
        :key="image.path"
        :entry="image"
        @click="handleCardClick(index)"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { usePictureViewStore } from '@/stores/pictureView'
import { useLightboxStore } from '@/stores/lightbox'
import PictureCard from './PictureCard.vue'

const pictureViewStore = usePictureViewStore()
const lightboxStore = useLightboxStore()

const currentImages = computed(() => pictureViewStore.currentImages)
const hasImages = computed(() => pictureViewStore.hasImages)

const gridStyle = computed(() => ({
  gridTemplateColumns: `repeat(auto-fill, minmax(${getCardSize()}px, 1fr))`
}))

function getCardSize(): number {
  // Calculate card size based on grid columns
  // Assuming container width ~1200px, with 16px gaps
  const columns = pictureViewStore.gridColumns
  const gap = 16
  const containerWidth = 1200
  return Math.floor((containerWidth - (gap * (columns + 1))) / columns)
}

function handleCardClick(index: number) {
  lightboxStore.open(currentImages.value, index)
}
</script>

<style scoped>
.picture-grid-container {
  height: 100%;
  overflow-y: auto;
  padding: 16px;
}

.picture-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
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
