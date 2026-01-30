<template>
  <div class="directory-node">
    <div
      :class="['tree-node', { selected: isSelected, expanded: isExpanded }]"
      :style="{ paddingLeft: level * 8 + 'px' }"
      @click="handleClick"
    >
      <span class="expand-icon" @click.stop="toggleExpand">
        {{ isExpanded ? '▼' : '▶' }}
      </span>
      <span class="node-icon">📁</span>
      <span class="node-label">{{ entry.name }}</span>
    </div>

    <!-- Lazy-loaded children -->
    <div v-if="isExpanded && children.length > 0" class="tree-children">
      <DirectoryNode
        v-for="child in children"
        :key="child.path"
        :entry="child"
        :level="level + 1"
        :selected-path="selectedPath"
        @select="$emit('select', $event)"
      />
    </div>

    <div v-if="isExpanded && loading" class="loading-item" :style="{ paddingLeft: (level + 1) * 8 + 'px' }">
      <span class="loading-spinner-small"></span>
      <span>Loading...</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useFileExplorerStore } from '@/stores/fileExplorer'
import type { FileEntry } from '@/types'

interface Props {
  entry: FileEntry
  level: number
  selectedPath: string
}

const props = defineProps<Props>()
const emit = defineEmits<{
  select: [path: string]
}>()

const fileExplorerStore = useFileExplorerStore()

const isExpanded = ref(false)
const children = ref<FileEntry[]>([])
const loading = ref(false)

const isSelected = computed(() => props.selectedPath === props.entry.path)

async function toggleExpand() {
  isExpanded.value = !isExpanded.value

  // Lazy load children if expanding and not already loaded
  if (isExpanded.value && children.value.length === 0) {
    await loadChildren()
  }
}

async function loadChildren() {
  try {
    loading.value = true
    const entries = await fileExplorerStore.readDirectory(props.entry.path)
    // Only show directories in tree view
    children.value = entries.filter(e => e.is_directory)
  } catch (e) {
    console.error(`Failed to load children for ${props.entry.path}:`, e)
  } finally {
    loading.value = false
  }
}

function handleClick() {
  emit('select', props.entry.path)
}
</script>

<style scoped>
.directory-node {
  /* Container for node and its children */
}

.tree-node {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 4px 8px;
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

.expand-icon {
  flex-shrink: 0;
  width: 12px;
  font-size: 10px;
  cursor: pointer;
  color: var(--text-secondary);
}

.tree-node.selected .expand-icon {
  color: rgba(255, 255, 255, 0.8);
}

.tree-node.expanded .expand-icon {
  transform: rotate(0deg);
}

.node-icon {
  flex-shrink: 0;
  font-size: 14px;
}

.node-label {
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.loading-item {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 4px 8px;
  font-size: 12px;
  color: var(--text-secondary);
}

.loading-spinner-small {
  display: inline-block;
  width: 12px;
  height: 12px;
  border: 2px solid var(--border-color);
  border-top-color: var(--primary-color);
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}
</style>
