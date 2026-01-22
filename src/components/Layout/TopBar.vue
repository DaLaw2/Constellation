<template>
  <div class="topbar">
    <div class="topbar-left">
      <h1 class="app-title">Constellation</h1>
      
      <!-- Expanded Mode Title -->
      <div v-if="sidebarExpanded" class="expanded-title">
        <span v-if="leftPanelMode === 'tag-management'">Tag Management</span>
        <span v-else-if="leftPanelMode === 'search'">Search</span>
      </div>

      <!-- Normal Mode Breadcrumb -->
      <div v-else class="path-breadcrumb">
        <span v-if="!currentPath" class="path-empty">No path selected</span>
        <span v-else class="path-text">{{ currentPath }}</span>
      </div>
    </div>

    <!-- Normal Mode Right Side -->
    <div v-if="!sidebarExpanded" class="topbar-right">
      <div class="search-box">
        <input
          type="search"
          v-model="searchQuery"
          @input="handleSearch"
          placeholder="Search files..."
          class="search-input"
        />
      </div>

      <div class="view-mode-toggle">
        <button
          :class="['btn-icon', { active: displayMode === 'detail' }]"
          @click="setMode('detail')"
          title="Detail view"
        >
          ☰
        </button>
        <button
          :class="['btn-icon', { active: displayMode === 'large-icons' }]"
          @click="setMode('large-icons')"
          title="Large icons"
        >
          ⊞
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useAppStore } from '@/stores/app'
import type { DisplayMode } from '@/types'

const appStore = useAppStore()

const currentPath = computed(() => appStore.currentPath)
const displayMode = computed(() => appStore.displayMode)
const sidebarExpanded = computed(() => appStore.sidebarExpanded)
const leftPanelMode = computed(() => appStore.leftPanelMode)
const searchQuery = computed({
  get: () => appStore.searchQuery,
  set: (value: string) => appStore.setSearchQuery(value),
})

function setMode(mode: DisplayMode) {
  appStore.setDisplayMode(mode)
}

function handleSearch() {
  // Search functionality will be implemented in Phase 1.6
  console.log('Search query:', searchQuery.value)
}
</script>

<style scoped>
.topbar {
  height: var(--topbar-height);
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 1rem;
  border-bottom: 1px solid var(--border-color);
  background: var(--background);
}

.topbar-left {
  display: flex;
  align-items: center;
  gap: 1rem;
  flex: 1;
  min-width: 0;
}

.app-title {
  font-size: 18px;
  font-weight: 600;
  white-space: nowrap;
}

.path-breadcrumb {
  flex: 1;
  min-width: 0;
  padding-left: 1rem;
  border-left: 1px solid var(--border-color);
}

.expanded-title {
  flex: 1;
  padding-left: 1rem;
  border-left: 1px solid var(--border-color);
  font-size: 14px;
  font-weight: 500;
  color: var(--text-primary);
}

.path-text {
  display: block;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  color: var(--text-primary);
  font-size: 13px;
}

.path-empty {
  color: var(--text-secondary);
  font-size: 13px;
  font-style: italic;
}

.topbar-right {
  display: flex;
  align-items: center;
  gap: 1rem;
}

.search-box {
  position: relative;
}

.search-input {
  width: 250px;
  padding: 6px 12px;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  font-size: 13px;
}

.search-input:focus {
  outline: none;
  border-color: var(--primary-color);
}

.view-mode-toggle {
  display: flex;
  gap: 4px;
  padding: 4px;
  background: var(--secondary-color);
  border-radius: 4px;
}

.view-mode-toggle .btn-icon {
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 4px;
  font-size: 16px;
}

.view-mode-toggle .btn-icon.active {
  background: var(--background);
  box-shadow: 0 1px 2px rgba(0, 0, 0, 0.1);
}
</style>
