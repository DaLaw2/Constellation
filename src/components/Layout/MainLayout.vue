<template>
  <div class="main-layout" :style="{ '--sidebar-width': sidebarWidth + 'px' }">
    <TopBar />
    <div class="content-area">
      <LeftPanel />
      <div class="resizer" @mousedown="startResize"></div>
      <div class="right-panel">
        <FileList />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onUnmounted } from 'vue'
import TopBar from './TopBar.vue'
import LeftPanel from './LeftPanel.vue'
import FileList from '../FileExplorer/FileList.vue'

const sidebarWidth = ref(250)
const isResizing = ref(false)

function startResize() {
  isResizing.value = true
  document.addEventListener('mousemove', resize)
  document.addEventListener('mouseup', stopResize)
  document.body.style.cursor = 'col-resize'
  document.body.style.userSelect = 'none'
}

function resize(event: MouseEvent) {
  if (isResizing.value) {
    // Limit width between 150px and 600px
    const newWidth = Math.min(Math.max(event.clientX, 150), 600)
    sidebarWidth.value = newWidth
  }
}

function stopResize() {
  isResizing.value = false
  document.removeEventListener('mousemove', resize)
  document.removeEventListener('mouseup', stopResize)
  document.body.style.cursor = ''
  document.body.style.userSelect = ''
}

onUnmounted(() => {
  stopResize()
})
</script>

<style scoped>
.main-layout {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  --sidebar-width: 250px; /* Default fallback */
}

.content-area {
  flex: 1;
  display: flex;
  overflow: hidden;
}

.resizer {
  width: 4px;
  background: transparent;
  cursor: col-resize;
  transition: background 0.2s;
  z-index: 10;
  margin-left: -2px; /* Overlap slightly */
  margin-right: -2px;
  position: relative;
}

.resizer:hover,
.resizer:active {
  background: var(--primary-color, #3b82f6);
}

.right-panel {
  flex: 1;
  overflow: hidden;
  min-width: 0; /* Prevent flex overflow */
}
</style>
