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
import { useAppStore } from '../../stores/app'
import { ref, onMounted, onUnmounted, computed } from 'vue'
import { useFileExplorerStore } from '../../stores/fileExplorer'
import TopBar from './TopBar.vue'
import LeftPanel from './LeftPanel.vue'
import FileList from '../FileExplorer/FileList.vue'

const appStore = useAppStore()
const fileExplorerStore = useFileExplorerStore()
const sidebarWidth = ref(270) // Pixels
const isResizing = ref(false)

const sidebarExpanded = computed(() => appStore.sidebarExpanded)

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
  stopResize()
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
  --sidebar-width: 270px; /* Default fallback */
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

/* Expanded state styles */
/* .main-layout.sidebar-expanded .content-area {
   No changes needed for flex container 
} */

.main-layout.sidebar-expanded .resizer {
  display: none;
}

.main-layout.sidebar-expanded .right-panel {
  display: none;
}

/* When expanded, LeftPanel (child of content-area) will flex-grow if we tell it to, 
   but LeftPanel has fixed width via style. We need to override that.
   However, LeftPanel uses var(--sidebar-width). 
   We can override the variable or the style in LeftPanel.
   Let's use a deep selector or simple CSS cascade if LeftPanel allows.
   Actually LeftPanel has scoped style: width: var(--sidebar-width).
   We can override the variable locally here if we could, but var is set on .main-layout.
   Better approach: force width 100% on the left panel via deep selector or 
   changing the var value dynamically? 
   Changing var is easy.
*/
.main-layout.sidebar-expanded {
  --sidebar-width: 100% !important;
}
</style>
