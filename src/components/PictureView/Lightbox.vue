<template>
  <Teleport to="body">
    <div v-if="showLightbox" class="lightbox" @click="handleBackdropClick">
      <button class="lightbox-close" @click="close" title="Close (ESC)">
        ×
      </button>

      <button 
        v-if="canNavigatePrev" 
        class="lightbox-nav lightbox-nav-prev" 
        @click.stop="navigatePrev"
        title="Previous (←)"
      >
        ‹
      </button>

      <button 
        v-if="canNavigateNext" 
        class="lightbox-nav lightbox-nav-next" 
        @click.stop="navigateNext"
        title="Next (→)"
      >
        ›
      </button>

      <div class="lightbox-content" @click.stop>
        <img 
          v-if="selectedImage"
          :src="`file://${selectedImage.path}`" 
          :alt="selectedImage.name"
          class="lightbox-image"
        />
        
        <div v-if="selectedImage" class="lightbox-info">
          <div class="info-name">{{ selectedImage.name }}</div>
          <div class="info-meta">
            <span v-if="selectedImage.size">{{ formatBytes(selectedImage.size) }}</span>
            <span v-if="selectedImage.modified_time">
              {{ formatDateTime(selectedImage.modified_time) }}
            </span>
          </div>
          <div v-if="itemTags.length > 0" class="info-tags">
            <span 
              v-for="tag in itemTags" 
              :key="tag.id" 
              class="tag-badge"
              :style="{ backgroundColor: getTagGroupColor(tag.group_id) }"
            >
              {{ tag.value }}
            </span>
          </div>
        </div>
      </div>
    </div>
  </Teleport>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted, onUnmounted } from 'vue'
import { usePictureViewStore } from '@/stores/pictureView'
import { useItemsStore } from '@/stores/items'
import { useTagsStore } from '@/stores/tags'
import { formatBytes, formatDateTime } from '@/utils'
import type { Tag } from '@/types'

const pictureViewStore = usePictureViewStore()
const itemsStore = useItemsStore()
const tagsStore = useTagsStore()

const itemTags = ref<Tag[]>([])

const showLightbox = computed(() => pictureViewStore.showLightbox)
const selectedImage = computed(() => pictureViewStore.selectedImage)
const canNavigatePrev = computed(() => pictureViewStore.selectedImageIndex > 0)
const canNavigateNext = computed(() => 
  pictureViewStore.selectedImageIndex < pictureViewStore.currentImages.length - 1
)

// Load tags when selected image changes
watch(selectedImage, async (newImage) => {
  if (newImage) {
    const item = await itemsStore.getItemByPath(newImage.path)
    if (item) {
      itemTags.value = await itemsStore.getTagsForItem(item.id)
    } else {
      itemTags.value = []
    }
  } else {
    itemTags.value = []
  }
})

function getTagGroupColor(groupId: number): string {
  const group = tagsStore.tagGroups.find(g => g.id === groupId)
  return group?.color || '#9e9e9e'
}

function close() {
  pictureViewStore.closeLightbox()
}

function navigatePrev() {
  pictureViewStore.navigatePrev()
}

function navigateNext() {
  pictureViewStore.navigateNext()
}

function handleBackdropClick() {
  close()
}

function handleKeydown(event: KeyboardEvent) {
  if (!showLightbox.value) return

  switch (event.key) {
    case 'Escape':
      close()
      break
    case 'ArrowLeft':
      if (canNavigatePrev.value) navigatePrev()
      break
    case 'ArrowRight':
      if (canNavigateNext.value) navigateNext()
      break
  }
}

onMounted(() => {
  window.addEventListener('keydown', handleKeydown)
})

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeydown)
})
</script>

<style scoped>
.lightbox {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.95);
  z-index: 10000;
  display: flex;
  align-items: center;
  justify-content: center;
  animation: fadeIn 0.2s ease;
}

@keyframes fadeIn {
  from {
    opacity: 0;
  }
  to {
    opacity: 1;
  }
}

.lightbox-close {
  position: absolute;
  top: 20px;
  right: 20px;
  width: 48px;
  height: 48px;
  border: none;
  background: rgba(255, 255, 255, 0.1);
  color: white;
  font-size: 32px;
  line-height: 1;
  cursor: pointer;
  border-radius: 50%;
  transition: background 0.2s ease;
  z-index: 10001;
}

.lightbox-close:hover {
  background: rgba(255, 255, 255, 0.2);
}

.lightbox-nav {
  position: absolute;
  top: 50%;
  transform: translateY(-50%);
  width: 64px;
  height: 64px;
  border: none;
  background: rgba(255, 255, 255, 0.1);
  color: white;
  font-size: 48px;
  line-height: 1;
  cursor: pointer;
  border-radius: 50%;
  transition: background 0.2s ease;
  z-index: 10001;
}

.lightbox-nav:hover {
  background: rgba(255, 255, 255, 0.2);
}

.lightbox-nav-prev {
  left: 20px;
}

.lightbox-nav-next {
  right: 20px;
}

.lightbox-content {
  max-width: 90vw;
  max-height: 90vh;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 20px;
}

.lightbox-image {
  max-width: 100%;
  max-height: calc(90vh - 120px);
  object-fit: contain;
  border-radius: 8px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.5);
}

.lightbox-info {
  background: rgba(0, 0, 0, 0.8);
  padding: 16px 24px;
  border-radius: 8px;
  color: white;
  text-align: center;
  max-width: 600px;
}

.info-name {
  font-size: 16px;
  font-weight: 600;
  margin-bottom: 8px;
}

.info-meta {
  font-size: 12px;
  color: rgba(255, 255, 255, 0.7);
  margin-bottom: 12px;
  display: flex;
  gap: 16px;
  justify-content: center;
}

.info-tags {
  display: flex;
  gap: 6px;
  flex-wrap: wrap;
  justify-content: center;
}

.tag-badge {
  font-size: 11px;
  padding: 4px 8px;
  border-radius: 4px;
  color: white;
  font-weight: 500;
}
</style>
