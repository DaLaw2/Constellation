<template>
  <div ref="containerRef" class="picture-grid-container">
    <div v-if="!hasImages" class="empty-state">
      <div class="empty-icon">🖼️</div>
      <div class="empty-title">No Media Found</div>
      <div class="empty-description">
        This directory doesn't contain any image or video files.
      </div>
    </div>

    <RecycleScroller
      v-else
      class="picture-scroller"
      :items="rows"
      :item-size="LAYOUT.PICTURE_ROW_HEIGHT"
      key-field="id"
      v-slot="{ item: row }"
    >
      <div class="picture-row" :style="rowStyle">
        <PictureCard
          v-for="(image, localIndex) in row.items"
          :key="image.path"
          :entry="image"
          @click="handleCardClick(row.startIndex + localIndex)"
        />
      </div>
    </RecycleScroller>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'
import { RecycleScroller } from 'vue-virtual-scroller'
import 'vue-virtual-scroller/dist/vue-virtual-scroller.css'
import { usePictureViewStore } from '@/stores/pictureView'
import { useLightboxStore } from '@/stores/lightbox'
import { useGridVirtualScroll } from '@/composables'
import { LAYOUT } from '@/constants'
import PictureCard from './PictureCard.vue'

const pictureViewStore = usePictureViewStore()
const lightboxStore = useLightboxStore()

const containerRef = ref<HTMLElement | null>(null)

const currentImages = computed(() => pictureViewStore.currentImages)
const hasImages = computed(() => pictureViewStore.hasImages)

// Virtual scroll row grouping
const { rows, columnCount } = useGridVirtualScroll(currentImages, {
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

function handleCardClick(index: number) {
  lightboxStore.open(currentImages.value, index)
}
</script>

<style scoped>
.picture-grid-container {
  height: 100%;
  overflow: hidden;
}

.picture-scroller {
  height: 100%;
  padding: 16px;
}

.picture-row {
  box-sizing: border-box;
  overflow: hidden;
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
