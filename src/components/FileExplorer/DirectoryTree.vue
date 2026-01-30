<template>
  <div class="directory-tree">
    <div v-if="loading && !drives.length" class="loading-state">
      <div class="loading-spinner"></div>
      <div>Loading drives...</div>
    </div>

    <div v-else class="tree-content">
      <!-- Drive list -->
      <div v-for="drive in drives" :key="drive.letter" class="drive-item">
        <div
          :class="['tree-node', 'drive-node', { selected: isSelected(drive.letter + ':\\') }]"
          @click="handleDriveClick(drive)"
        >
          <span class="node-icon">
            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <line x1="22" y1="12" x2="2" y2="12"></line>
              <path d="M5.45 5.11L2 12v6a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-6l-3.45-6.89A2 2 0 0 0 16.76 4H7.24a2 2 0 0 0-1.79 1.11z"></path>
              <line x1="6" y1="16" x2="6.01" y2="16"></line>
              <line x1="10" y1="16" x2="10.01" y2="16"></line>
            </svg>
          </span>
          <span class="node-label">{{ drive.letter }}: {{ drive.label || 'Local Disk' }}</span>
          <span v-if="drive.available_space" class="drive-space">
            {{ formatBytes(drive.available_space) }} free
          </span>
        </div>

        <!-- Expanded drive directories -->
        <div v-if="expandedDrives.has(drive.letter)" class="tree-children">
          <DirectoryNode
            v-for="entry in getDriveContents(drive.letter)"
            :key="entry.path"
            :entry="entry"
            :level="1"
            :selected-path="fileExplorerStore.currentPath"
            @select="handleNodeSelect"
          />
        </div>
      </div>
    </div>

    <div v-if="error" class="error-state">
      <div class="error-icon">⚠️</div>
      <div class="error-message">{{ error }}</div>
      <button class="retry-btn" @click="loadDrives">Retry</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { useFileExplorerStore } from '@/stores/fileExplorer'
import { formatBytes } from '@/utils'
import DirectoryNode from './DirectoryNode.vue'
import type { DriveInfo, FileEntry } from '@/types'

const fileExplorerStore = useFileExplorerStore()

const drives = computed(() => fileExplorerStore.drives)
const loading = computed(() => fileExplorerStore.loading)
const error = computed(() => fileExplorerStore.error)

const expandedDrives = ref(new Set<string>())
const driveContents = ref(new Map<string, FileEntry[]>())

onMounted(async () => {
  await loadDrives()
})

async function loadDrives() {
  try {
    await fileExplorerStore.getDrives()
  } catch (e) {
    console.error('Failed to load drives:', e)
  }
}

async function handleDriveClick(drive: DriveInfo) {
  const drivePath = drive.letter + ':\\'

  // Toggle expansion
  if (expandedDrives.value.has(drive.letter)) {
    expandedDrives.value.delete(drive.letter)
  } else {
    expandedDrives.value.add(drive.letter)

    // Lazy load drive contents if not already loaded
    if (!driveContents.value.has(drive.letter)) {
      try {
        const entries = await fileExplorerStore.readDirectory(drivePath)
        // Only store directories for tree view
        driveContents.value.set(
          drive.letter,
          entries.filter(e => e.is_directory)
        )
      } catch (e) {
        console.error(`Failed to read drive ${drive.letter}:`, e)
      }
    }
  }

  // Navigate to drive root
  fileExplorerStore.navigateTo(drivePath)
}

function getDriveContents(driveLetter: string): FileEntry[] {
  return driveContents.value.get(driveLetter) || []
}

function handleNodeSelect(path: string) {
  fileExplorerStore.navigateTo(path)
}

function isSelected(path: string): boolean {
  return fileExplorerStore.currentPath === path
}
</script>

<style scoped>
.directory-tree {
  height: 100%;
  overflow: auto;
  padding: 8px 0;
  background: var(--surface);
}

.loading-state,
.error-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 12px;
  padding: 24px;
  color: var(--text-secondary);
  font-size: 13px;
}

.loading-spinner {
  width: 24px;
  height: 24px;
  border: 3px solid var(--border-color);
  border-top-color: var(--primary-color);
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

.error-icon {
  font-size: 32px;
}

.error-message {
  color: var(--text-primary);
  text-align: center;
}

.retry-btn {
  padding: 8px 16px;
  background: var(--primary-color);
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 13px;
  transition: var(--transition-fast);
}

.retry-btn:hover {
  opacity: 0.9;
}

.tree-content {
  padding: 0 4px;
}

.drive-item {
  margin-bottom: 4px;
}

.tree-node {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 6px 8px;
  cursor: pointer;
  border-radius: 4px;
  transition: var(--transition-fast);
  font-size: 13px;
  user-select: none;
}

.tree-node:hover {
  background: rgba(0, 0, 0, 0.04);
}

.tree-node.selected {
  background: var(--primary-color);
  color: white;
}

.drive-node {
  font-weight: 500;
}

.node-icon {
  flex-shrink: 0;
  font-size: 16px;
}

.node-label {
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.drive-space {
  font-size: 11px;
  color: var(--text-secondary);
  flex-shrink: 0;
}

.tree-node.selected .drive-space {
  color: rgba(255, 255, 255, 0.8);
}

.tree-children {
  padding-left: 16px;
}
</style>
