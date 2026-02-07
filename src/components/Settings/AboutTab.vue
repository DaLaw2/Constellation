<template>
  <div class="about-tab">
    <div class="app-icon">
      <img src="/app-icon.png" alt="Constellation" class="icon-image" />
    </div>
    <h2 class="app-name">Constellation</h2>
    <p class="app-version">Version {{ version }}</p>
    <p class="app-author">Author: DaLaw2</p>
    <p class="app-tech">Built with Tauri + Vue 3</p>

    <div class="about-links">
      <button class="link-btn" @click="openGitHub">
        <svg class="link-icon" viewBox="0 0 16 16" fill="currentColor">
          <path d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.013 8.013 0 0016 8c0-4.42-3.58-8-8-8z" />
        </svg>
        GitHub
      </button>
      <span class="separator">|</span>
      <button class="link-btn" @click="openLicense">
        <svg class="link-icon" viewBox="0 0 16 16" fill="currentColor">
          <path d="M8.75.75V2h.985c.304 0 .603.08.867.231l1.29.736c.038.022.08.033.124.033h2.234a.75.75 0 010 1.5h-.427l2.104 4.461a.75.75 0 01-.296.91 3.18 3.18 0 01-1.63.46 3.18 3.18 0 01-1.63-.46.75.75 0 01-.296-.91l2.104-4.46H13.5a1.75 1.75 0 01-.867-.232L11.343 3.5H8.75v8h2.5a.75.75 0 010 1.5h-6.5a.75.75 0 010-1.5h2.5v-8H5.657L4.367 4.268A1.75 1.75 0 013.5 4.5h-.427l2.104 4.461a.75.75 0 01-.296.91A3.18 3.18 0 013.25 10.33a3.18 3.18 0 01-1.63-.46.75.75 0 01-.296-.91L3.428 4.5H3a.75.75 0 010-1.5h2.234c.044 0 .086-.011.124-.033l1.29-.736A1.75 1.75 0 017.5 2H8.75V.75a.75.75 0 011.5 0zM11.5 6.532L13.2 10h-3.4L11.5 6.532zM4.5 6.532L6.2 10H2.8L4.5 6.532z" />
        </svg>
        GPL-3.0
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { getVersion } from '@tauri-apps/api/app'
import { openUrl } from '@tauri-apps/plugin-opener'

const version = ref('...')

onMounted(async () => {
  try {
    version.value = await getVersion()
  } catch {
    version.value = '1.1.0'
  }
})

function openGitHub() {
  openUrl('https://github.com/DaLaw2/Constellation')
}

function openLicense() {
  openUrl('https://github.com/DaLaw2/Constellation/blob/master/LICENSE')
}
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
  margin: 0 0 20px 0;
  font-size: 12px;
  color: var(--text-secondary);
  opacity: 0.7;
}

.about-links {
  display: flex;
  align-items: center;
  gap: 12px;
}

.link-btn {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  background: none;
  border: none;
  color: var(--accent-primary, #60a5fa);
  font-size: 13px;
  cursor: pointer;
  padding: 4px 8px;
  border-radius: 4px;
  transition: background 0.15s;
}

.link-btn:hover {
  background: var(--bg-hover, rgba(255, 255, 255, 0.06));
}

.link-icon {
  width: 14px;
  height: 14px;
}

.separator {
  color: var(--text-secondary);
  opacity: 0.3;
  font-size: 13px;
}
</style>
