<template>
  <div
    :class="['file-item', { selected: isSelected, directory: entry.is_directory }]"
    @click="handleClick"
    @dblclick="handleDoubleClick"
    @contextmenu.prevent="handleContextMenu"
  >
    <div class="file-icon">
      {{ getFileIcon(entry) }}
    </div>
    <div class="file-info">
      <div class="file-name">{{ entry.name }}</div>
      <div class="file-meta">
        <span v-if="!entry.is_directory && entry.size !== null" class="file-size">
          {{ formatBytes(entry.size) }}
        </span>
        <span v-if="entry.modified_time" class="file-date">
          {{ formatDate(entry.modified_time) }}
        </span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import type { FileEntry } from '../../stores/fileExplorer'

interface Props {
  entry: FileEntry
  selected?: boolean
}

const props = defineProps<Props>()
const emit = defineEmits<{
  click: [entry: FileEntry]
  doubleClick: [entry: FileEntry]
  contextMenu: [entry: FileEntry, event: MouseEvent]
}>()

const isSelected = computed(() => props.selected)

function handleClick() {
  emit('click', props.entry)
}

function handleDoubleClick() {
  emit('doubleClick', props.entry)
}

function handleContextMenu(event: MouseEvent) {
  emit('contextMenu', props.entry, event)
}

function getFileIcon(entry: FileEntry): string {
  if (entry.is_directory) {
    return '📁'
  }

  // Determine icon based on file extension
  const ext = entry.name.split('.').pop()?.toLowerCase()

  switch (ext) {
    case 'txt':
    case 'md':
    case 'doc':
    case 'docx':
      return '📄'
    case 'pdf':
      return '📕'
    case 'jpg':
    case 'jpeg':
    case 'png':
    case 'gif':
    case 'bmp':
    case 'svg':
    case 'webp':
      return '🖼️'
    case 'mp3':
    case 'wav':
    case 'flac':
    case 'ogg':
      return '🎵'
    case 'mp4':
    case 'avi':
    case 'mkv':
    case 'mov':
    case 'wmv':
      return '🎬'
    case 'zip':
    case 'rar':
    case '7z':
    case 'tar':
    case 'gz':
      return '📦'
    case 'js':
    case 'ts':
    case 'jsx':
    case 'tsx':
    case 'py':
    case 'java':
    case 'cpp':
    case 'c':
    case 'cs':
    case 'go':
    case 'rs':
    case 'php':
      return '💻'
    case 'html':
    case 'css':
    case 'scss':
    case 'sass':
      return '🌐'
    case 'json':
    case 'xml':
    case 'yaml':
    case 'yml':
      return '📋'
    case 'exe':
    case 'msi':
    case 'app':
      return '⚙️'
    default:
      return '📄'
  }
}

function formatBytes(bytes: number): string {
  if (bytes === 0) return '0 B'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return Math.round(bytes / Math.pow(k, i) * 100) / 100 + ' ' + sizes[i]
}

function formatDate(timestamp: number): string {
  const date = new Date(timestamp * 1000)
  const now = new Date()
  const diff = now.getTime() - date.getTime()
  const days = Math.floor(diff / (1000 * 60 * 60 * 24))

  if (days === 0) {
    return 'Today ' + date.toLocaleTimeString('en-US', { hour: '2-digit', minute: '2-digit' })
  } else if (days === 1) {
    return 'Yesterday'
  } else if (days < 7) {
    return `${days} days ago`
  } else {
    return date.toLocaleDateString('en-US', { month: 'short', day: 'numeric', year: 'numeric' })
  }
}
</script>

<style scoped>
.file-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 10px 12px;
  border-bottom: 1px solid var(--border-color);
  cursor: pointer;
  transition: var(--transition-fast);
  user-select: none;
}

.file-item:hover {
  background: rgba(0, 0, 0, 0.02);
}

.file-item.selected {
  background: rgba(99, 102, 241, 0.1);
  border-left: 3px solid var(--primary-color);
  padding-left: 9px;
}

.file-item.directory {
  font-weight: 500;
}

.file-icon {
  flex-shrink: 0;
  font-size: 24px;
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.file-info {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.file-name {
  font-size: 14px;
  color: var(--text-primary);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.file-meta {
  display: flex;
  align-items: center;
  gap: 12px;
  font-size: 12px;
  color: var(--text-secondary);
}

.file-size,
.file-date {
  white-space: nowrap;
}
</style>
