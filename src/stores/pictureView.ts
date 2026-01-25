import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { useFileExplorerStore } from './fileExplorer'

export const usePictureViewStore = defineStore('pictureView', () => {
    const fileExplorerStore = useFileExplorerStore()

    // State
    const gridColumns = ref<number>(4)
    const selectedImageIndex = ref<number>(-1)

    // Supported image extensions
    const imageExtensions = ['.jpg', '.jpeg', '.png', '.gif', '.webp', '.bmp', '.svg']

    // Computed
    const currentImages = computed(() => {
        return fileExplorerStore.currentFiles.filter(file =>
            !file.is_directory &&
            imageExtensions.some(ext => file.name.toLowerCase().endsWith(ext))
        )
    })

    const selectedImage = computed(() => {
        if (selectedImageIndex.value >= 0 && selectedImageIndex.value < currentImages.value.length) {
            return currentImages.value[selectedImageIndex.value]
        }
        return null
    })

    const showLightbox = computed(() => selectedImageIndex.value >= 0)

    const hasImages = computed(() => currentImages.value.length > 0)

    // Actions
    function setGridColumns(columns: number) {
        if (columns >= 2 && columns <= 8) {
            gridColumns.value = columns
        }
    }

    function openLightbox(index: number) {
        if (index >= 0 && index < currentImages.value.length) {
            selectedImageIndex.value = index
        }
    }

    function closeLightbox() {
        selectedImageIndex.value = -1
    }

    function navigateNext() {
        if (selectedImageIndex.value < currentImages.value.length - 1) {
            selectedImageIndex.value++
        }
    }

    function navigatePrev() {
        if (selectedImageIndex.value > 0) {
            selectedImageIndex.value--
        }
    }

    function openImageByPath(path: string) {
        const index = currentImages.value.findIndex(img => img.path === path)
        if (index >= 0) {
            openLightbox(index)
        }
    }

    return {
        // State
        gridColumns,
        selectedImageIndex,

        // Computed
        currentImages,
        selectedImage,
        showLightbox,
        hasImages,

        // Actions
        setGridColumns,
        openLightbox,
        closeLightbox,
        navigateNext,
        navigatePrev,
        openImageByPath,
    }
})
