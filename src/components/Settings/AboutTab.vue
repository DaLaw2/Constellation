<template>
  <div class="about-tab">
    <div class="app-icon">
      <img src="/tauri.svg" alt="Constellation" class="icon-image" />
    </div>
    <h2 class="app-name">Constellation</h2>
    <p class="app-version">Version {{ version }}</p>
    <p class="app-author">Author: DaLaw2</p>
    <p class="app-tech">Built with Tauri + Vue 3</p>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { getVersion } from '@tauri-apps/api/app'

const version = ref('...')

onMounted(async () => {
  try {
    version.value = await getVersion()
  } catch {
    version.value = '1.0.0'
  }
})
</script>

<style scoped>
.about-tab {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 32px 24px;
  text-align: center;
}

.app-icon {
  width: 72px;
  height: 72px;
  margin-bottom: 16px;
}

.icon-image {
  width: 100%;
  height: 100%;
  object-fit: contain;
}

.app-name {
  margin: 0 0 8px 0;
  font-size: 22px;
  font-weight: 700;
  color: var(--text-primary);
}

.app-version {
  margin: 0 0 4px 0;
  font-size: 14px;
  color: var(--text-secondary);
}

.app-author {
  margin: 0 0 16px 0;
  font-size: 14px;
  color: var(--text-secondary);
}

.app-tech {
  margin: 0;
  font-size: 12px;
  color: var(--text-secondary);
  opacity: 0.7;
}
</style>
