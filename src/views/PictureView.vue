<template>
  <div class="picture-view">
    <div class="picture-toolbar">
      <div class="toolbar-title">
        <span class="title-icon">🖼️</span>
        <h2>Picture View</h2>
      </div>
      <div class="toolbar-actions">
        <div class="grid-size-control">
          <button 
            v-for="size in [3, 4, 5, 6]" 
            :key="size"
            :class="['size-btn', { active: gridColumns === size }]"
            @click="setGridSize(size)"
            :title="`${size} columns`"
          >
            {{ size }}
          </button>
        </div>
        <div class="image-count">
          {{ currentImages.length }} {{ currentImages.length === 1 ? 'image' : 'images' }}
        </div>
      </div>
    </div>

    <PictureGrid />
    <Lightbox />
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { usePictureViewStore } from '@/stores/pictureView'
import PictureGrid from '@/components/PictureView/PictureGrid.vue'
import Lightbox from '@/components/PictureView/Lightbox.vue'

const pictureViewStore = usePictureViewStore()

const gridColumns = computed(() => pictureViewStore.gridColumns)
const currentImages = computed(() => pictureViewStore.currentImages)

function setGridSize(size: number) {
  pictureViewStore.setGridColumns(size)
}
</script>

<style scoped>
.picture-view {
  height: 100%;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.picture-toolbar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px 20px;
  border-bottom: 1px solid var(--border-color);
  background: var(--surface);
  flex-shrink: 0;
}

.toolbar-title {
  display: flex;
  align-items: center;
  gap: 12px;
}

.title-icon {
  font-size: 24px;
}

.toolbar-title h2 {
  font-size: 18px;
  font-weight: 600;
  margin: 0;
  color: var(--text-primary);
}

.toolbar-actions {
  display: flex;
  align-items: center;
  gap: 16px;
}

.grid-size-control {
  display: flex;
  gap: 4px;
  padding: 4px;
  background: var(--background);
  border-radius: 6px;
}

.size-btn {
  padding: 6px 12px;
  border: none;
  background: transparent;
  color: var(--text-secondary);
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  border-radius: 4px;
  transition: all 0.2s ease;
}

.size-btn:hover {
  background: var(--surface);
  color: var(--text-primary);
}

.size-btn.active {
  background: var(--primary-color);
  color: white;
}

.image-count {
  font-size: 13px;
  color: var(--text-secondary);
  padding: 6px 12px;
  background: var(--background);
  border-radius: 6px;
}
</style>
