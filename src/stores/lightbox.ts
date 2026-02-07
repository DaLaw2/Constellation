import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { FileEntry } from '@/types'

/**
 * Unified lightbox store for all image viewing functionality.
 * Replaces scattered lightbox logic in fileExplorerStore and pictureViewStore.
 */
export const useLightboxStore = defineStore('lightbox', () => {
  // State
  const isOpen = ref(false)
  const currentIndex = ref(-1)
  const images = ref<FileEntry[]>([])

  // Computed
  const currentImage = computed<FileEntry | null>(() => {
    if (currentIndex.value >= 0 && currentIndex.value < images.value.length) {
      return images.value[currentIndex.value]
    }
    return null
  })

  const canNavigatePrev = computed(() => currentIndex.value > 0)

  const canNavigateNext = computed(() => currentIndex.value < images.value.length - 1)

  const totalImages = computed(() => images.value.length)

  const currentPosition = computed(() => {
    if (currentIndex.value === -1) return { current: 0, total: 0 }
    return {
      current: currentIndex.value + 1,
      total: images.value.length,
    }
  })

  // Actions

  /**
   * Open lightbox with a specific image
   * @param imageList - Array of FileEntry objects (images only)
   * @param index - Index of the image to display
   */
  function open(imageList: FileEntry[], index: number) {
    if (index < 0 || index >= imageList.length) {
      console.warn('Invalid lightbox index:', index)
      return
    }

    images.value = imageList
    currentIndex.value = index
    isOpen.value = true
  }

  /**
   * Open lightbox by finding image path in list
   * @param imageList - Array of FileEntry objects (images only)
   * @param imagePath - Path of the image to display
   */
  function openByPath(imageList: FileEntry[], imagePath: string) {
    const index = imageList.findIndex((img) => img.path === imagePath)
    if (index >= 0) {
      open(imageList, index)
    } else {
      console.warn('Image not found in list:', imagePath)
    }
  }

  /**
   * Close the lightbox
   */
  function close() {
    isOpen.value = false
    currentIndex.value = -1
    // Keep images in memory for faster reopening
  }

  /**
   * Navigate to previous image
   */
  function prev() {
    if (canNavigatePrev.value) {
      currentIndex.value--
    }
  }

  /**
   * Navigate to next image
   */
  function next() {
    if (canNavigateNext.value) {
      currentIndex.value++
    }
  }

  /**
   * Navigate to specific index
   * @param index - Target index
   */
  function goToIndex(index: number) {
    if (index >= 0 && index < images.value.length) {
      currentIndex.value = index
    }
  }

  /**
   * Clear all lightbox state (useful when changing directories)
   */
  function reset() {
    isOpen.value = false
    currentIndex.value = -1
    images.value = []
  }

  return {
    // State
    isOpen,
    currentIndex,
    images,

    // Computed
    currentImage,
    canNavigatePrev,
    canNavigateNext,
    totalImages,
    currentPosition,

    // Actions
    open,
    openByPath,
    close,
    prev,
    next,
    goToIndex,
    reset,
  }
})
