<template>
  <div
    class="main-layout"
    :class="{ 'sidebar-expanded': sidebarExpanded }"
    :style="{ '--sidebar-width': sidebarWidth + 'px' }"
  >
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
import { computed, onMounted, onUnmounted } from 'vue'
import { useAppStore } from '@/stores/app'
import { useFileExplorerStore } from '@/stores/fileExplorer'
import { useResizablePanel } from '@/composables'
import TopBar from './TopBar.vue'
import LeftPanel from './LeftPanel.vue'
import FileList from '../FileExplorer/FileList.vue'

const appStore = useAppStore()
const fileExplorerStore = useFileExplorerStore()

const sidebarExpanded = computed(() => appStore.sidebarExpanded)

// Use resizable panel composable for sidebar width
const { width: sidebarWidth, startResize } = useResizablePanel({
  minWidth: 150,
  maxWidth: 600,
  initialWidth: 270,
})

// Mouse navigation handlers (Back/Forward)
function handleMouseNavigation(event: MouseEvent) {
  // Button 3 is Back, Button 4 is Forward
  if (event.button === 3) {
    fileExplorerStore.goBack()
  } else if (event.button === 4) {
    fileExplorerStore.goForward()
  }
}

onMounted(() => {
  window.addEventListener('mouseup', handleMouseNavigation)
})

onUnmounted(() => {
  window.removeEventListener('mouseup', handleMouseNavigation)
})
</script>

<style scoped>
.main-layout {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  --sidebar-width: 270px;
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
  margin-left: -2px;
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
  min-width: 0;
}

/* Expanded sidebar state */
.main-layout.sidebar-expanded {
  --sidebar-width: 100% !important;
}

.main-layout.sidebar-expanded .resizer {
  display: none;
}

.main-layout.sidebar-expanded .right-panel {
  display: none;
}
</style>
