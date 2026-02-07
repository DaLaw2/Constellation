import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { useFileExplorerStore } from './fileExplorer'
import { isMediaFile } from '@/utils'

/**
 * Picture View store - manages grid layout for image viewing.
 * Lightbox functionality has been moved to stores/lightbox.ts
 */
export const usePictureViewStore = defineStore('pictureView', () => {
    const fileExplorerStore = useFileExplorerStore()

    // State
    const gridColumns = ref<number>(4)

    // Computed
    const currentImages = computed(() => {
        return fileExplorerStore.currentFiles.filter(file =>
            !file.is_directory && isMediaFile(file.name)
        )
    })

    const hasImages = computed(() => currentImages.value.length > 0)

    // Actions
    function setGridColumns(columns: number) {
        if (columns >= 2 && columns <= 8) {
            gridColumns.value = columns
        }
    }

    return {
        // State
        gridColumns,

        // Computed
        currentImages,
        hasImages,

        // Actions
        setGridColumns,
    }
})
