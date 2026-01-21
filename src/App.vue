<script setup lang="ts">
import MainLayout from './components/Layout/MainLayout.vue'
import { onMounted, onUnmounted } from 'vue'

function handleGlobalContextMenu(e: MouseEvent) {
  if (!(e.target as HTMLElement).closest('.allow-context-menu')) {
    e.preventDefault()
  }
}

onMounted(() => {
  document.addEventListener('contextmenu', handleGlobalContextMenu)
})

onUnmounted(() => {
  document.removeEventListener('contextmenu', handleGlobalContextMenu)
})
</script>

<template>
  <MainLayout />
</template>