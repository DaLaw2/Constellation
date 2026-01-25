<template>
  <div 
    class="grid-view" 
    :style="gridStyle"
    @wheel.ctrl.prevent="handleZoom"
  >
    <GridFileCard
      v-for="file in files"
      :key="file.path"
      :file="file"
      :zoom-level="zoomLevel"
      @open="handleOpen"
      @contextmenu="handleContextMenu"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import GridFileCard from './GridFileCard.vue'
import type { FileEntry } from '@/types'

interface Props {
  files: FileEntry[]
}

defineProps<Props>()

const emit = defineEmits<{
  open: [file: FileEntry]
  contextmenu: [event: MouseEvent, file: FileEntry]
}>()

// Zoom level: 50-300 (percentage)
const zoomLevel = ref(100)
const MIN_ZOOM = 50
const MAX_ZOOM = 300

const gridStyle = computed(() => {
  const baseSize = 150
  const baseGap = 16
  const cardSize = Math.floor(baseSize * (zoomLevel.value / 100))
  const gap = Math.floor(baseGap * (zoomLevel.value / 100))
  
  return {
    gridTemplateColumns: `repeat(auto-fill, minmax(${cardSize}px, 1fr))`,
    gap: `${gap}px`
  }
})

function handleOpen(file: FileEntry) {
  emit('open', file)
}

function handleContextMenu(event: MouseEvent, file: FileEntry) {
  emit('contextmenu', event, file)
}

function handleZoom(event: WheelEvent) {
  const delta = event.deltaY > 0 ? -10 : 10
  const newZoom = Math.max(MIN_ZOOM, Math.min(MAX_ZOOM, zoomLevel.value + delta))
  zoomLevel.value = newZoom
}
</script>

<style scoped>
.grid-view {
  display: grid;
  grid-auto-rows: min-content;
  padding: 16px;
  height: 100%;
  overflow-y: auto;
}
</style>
