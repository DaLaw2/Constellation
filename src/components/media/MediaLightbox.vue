<template>
  <Teleport to="body" :disabled="!isMounted">
    <Transition name="lightbox-fade">
      <div
        v-if="lightboxStore.isOpen && isMounted"
        class="media-lightbox"
        @click="handleBackdropClick"
      >
        <!-- Close button -->
        <button
          class="lightbox-btn lightbox-close"
          @click="handleClose"
          title="Close (ESC)"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="24"
            height="24"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
          >
            <line x1="18" y1="6" x2="6" y2="18" />
            <line x1="6" y1="6" x2="18" y2="18" />
          </svg>
        </button>

        <!-- Previous button -->
        <button
          v-if="lightboxStore.canNavigatePrev"
          class="lightbox-btn lightbox-nav lightbox-nav-prev"
          @click.stop="lightboxStore.prev"
          title="Previous (←)"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="32"
            height="32"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
          >
            <polyline points="15 18 9 12 15 6" />
          </svg>
        </button>

        <!-- Next button -->
        <button
          v-if="lightboxStore.canNavigateNext"
          class="lightbox-btn lightbox-nav lightbox-nav-next"
          @click.stop="lightboxStore.next"
          title="Next (→)"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="32"
            height="32"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
          >
            <polyline points="9 18 15 12 9 6" />
          </svg>
        </button>

        <!-- Image counter -->
        <div class="lightbox-counter">
          {{ lightboxStore.currentPosition.current }} / {{ lightboxStore.currentPosition.total }}
        </div>

        <!-- Main content -->
        <div class="lightbox-content" @click.stop>
          <img
            v-if="lightboxStore.currentImage"
            :src="`file://${lightboxStore.currentImage.path}`"
            :alt="lightboxStore.currentImage.name"
            class="lightbox-image"
            @load="handleImageLoad"
            @error="handleImageError"
          />

          <!-- Loading state -->
          <div v-if="isLoading" class="lightbox-loading">
            <div class="spinner"></div>
          </div>

          <!-- Error state -->
          <div v-if="hasError" class="lightbox-error">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="48"
              height="48"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
            >
              <circle cx="12" cy="12" r="10" />
              <line x1="12" y1="8" x2="12" y2="12" />
              <line x1="12" y1="16" x2="12.01" y2="16" />
            </svg>
            <p>Failed to load image</p>
          </div>

          <!-- Image info -->
          <div v-if="lightboxStore.currentImage && !isLoading" class="lightbox-info">
            <div class="info-name">{{ lightboxStore.currentImage.name }}</div>

            <div class="info-meta">
              <span v-if="lightboxStore.currentImage.size">
                {{ formatBytes(lightboxStore.currentImage.size) }}
              </span>
              <span v-if="lightboxStore.currentImage.modified_time">
                {{ formatDateTime(lightboxStore.currentImage.modified_time) }}
              </span>
            </div>

            <!-- Tags -->
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
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
import { ref, watch, onMounted, onUnmounted } from 'vue'
import { useLightboxStore } from '@/stores/lightbox'
import { useItemsStore } from '@/stores/items'
import { useTagsStore } from '@/stores/tags'
import { formatBytes, formatDateTime } from '@/utils'
import type { Tag } from '@/types'

const lightboxStore = useLightboxStore()
const itemsStore = useItemsStore()
const tagsStore = useTagsStore()

const isMounted = ref(false)
const itemTags = ref<Tag[]>([])
const isLoading = ref(false)
const hasError = ref(false)

// Load tags when current image changes
watch(
  () => lightboxStore.currentImage,
  async (newImage) => {
    itemTags.value = []

    if (newImage) {
      isLoading.value = true
      hasError.value = false

      try {
        const item = await itemsStore.getItemByPath(newImage.path)
        if (item) {
          itemTags.value = await itemsStore.getTagsForItem(item.id)
        }
      } catch (error) {
        console.error('Failed to load tags:', error)
      } finally {
        isLoading.value = false
      }
    }
  }
)

function getTagGroupColor(groupId: number): string {
  const group = tagsStore.tagGroups.find((g) => g.id === groupId)
  return group?.color || '#9e9e9e'
}

function handleClose() {
  lightboxStore.close()
}

function handleBackdropClick() {
  lightboxStore.close()
}

function handleImageLoad() {
  isLoading.value = false
  hasError.value = false
}

function handleImageError() {
  isLoading.value = false
  hasError.value = true
}

// Keyboard navigation
function handleKeydown(event: KeyboardEvent) {
  if (!lightboxStore.isOpen) return

  switch (event.key) {
    case 'Escape':
      lightboxStore.close()
      break
    case 'ArrowLeft':
      lightboxStore.prev()
      break
    case 'ArrowRight':
      lightboxStore.next()
      break
  }
}

onMounted(() => {
  isMounted.value = true
  document.addEventListener('keydown', handleKeydown)
})

onUnmounted(() => {
  document.removeEventListener('keydown', handleKeydown)
})
</script>

<style scoped>
.media-lightbox {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.95);
  z-index: 9999;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: zoom-out;
}

.lightbox-btn {
  position: absolute;
  background: rgba(255, 255, 255, 0.1);
  border: none;
  color: white;
  cursor: pointer;
  transition: var(--transition-fast);
  backdrop-filter: blur(8px);
  display: flex;
  align-items: center;
  justify-content: center;
}

.lightbox-btn:hover {
  background: rgba(255, 255, 255, 0.2);
}

.lightbox-close {
  top: 20px;
  right: 20px;
  width: 48px;
  height: 48px;
  border-radius: 50%;
}

.lightbox-nav {
  width: 64px;
  height: 64px;
  border-radius: 50%;
  top: 50%;
  transform: translateY(-50%);
}

.lightbox-nav-prev {
  left: 40px;
}

.lightbox-nav-next {
  right: 40px;
}

.lightbox-counter {
  position: absolute;
  top: 20px;
  left: 50%;
  transform: translateX(-50%);
  background: rgba(0, 0, 0, 0.6);
  color: white;
  padding: 8px 16px;
  border-radius: 20px;
  font-size: 14px;
  font-weight: 500;
  backdrop-filter: blur(8px);
  user-select: none;
}

.lightbox-content {
  position: relative;
  max-width: 90vw;
  max-height: 90vh;
  display: flex;
  flex-direction: column;
  align-items: center;
  cursor: default;
}

.lightbox-image {
  max-width: 100%;
  max-height: calc(90vh - 120px);
  object-fit: contain;
  border-radius: 4px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.5);
}

.lightbox-loading,
.lightbox-error {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 16px;
  color: white;
  min-height: 200px;
}

.spinner {
  width: 48px;
  height: 48px;
  border: 4px solid rgba(255, 255, 255, 0.2);
  border-top-color: white;
  border-radius: 50%;
  animation: spin 0.8s linear infinite;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

.lightbox-error svg {
  color: #ff5252;
}

.lightbox-error p {
  margin: 0;
  font-size: 16px;
}

.lightbox-info {
  margin-top: 20px;
  padding: 16px 24px;
  background: rgba(0, 0, 0, 0.8);
  border-radius: 8px;
  max-width: 600px;
  backdrop-filter: blur(12px);
}

.info-name {
  font-size: 16px;
  font-weight: 500;
  color: white;
  margin-bottom: 8px;
  word-break: break-all;
}

.info-meta {
  display: flex;
  gap: 16px;
  font-size: 13px;
  color: rgba(255, 255, 255, 0.7);
  margin-bottom: 12px;
}

.info-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
}

.tag-badge {
  display: inline-block;
  padding: 4px 10px;
  border-radius: 12px;
  font-size: 12px;
  font-weight: 500;
  color: white;
  background: #9e9e9e;
}

/* Transitions */
.lightbox-fade-enter-active,
.lightbox-fade-leave-active {
  transition: opacity 0.3s ease;
}

.lightbox-fade-enter-from,
.lightbox-fade-leave-to {
  opacity: 0;
}
</style>
