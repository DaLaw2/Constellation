<template>
  <div
    :class="['file-item', { selected: isSelected, directory: entry.is_directory }]"
    @click="handleClick"
    @dblclick="handleDoubleClick"
    @contextmenu.prevent="handleContextMenu"
  >
    <!-- File Icon -->
    <div class="file-icon">
      {{ entry.is_directory ? '📁' : getFileIcon(entry.name) }}
    </div>

    <!-- File Info -->
    <div class="file-info">
      <div class="file-name" :title="entry.name">
        <template v-if="nameSegments.length">
          <span
            v-for="(seg, idx) in nameSegments"
            :key="idx"
            :class="{ 'search-highlight': seg.highlight }"
          >
            {{ seg.text }}
          </span>
        </template>
        <template v-else>
          {{ entry.name }}
        </template>
      </div>
      <div class="file-meta">
        <span v-if="!entry.is_directory && entry.size !== null" class="file-size">
          {{ formatBytes(entry.size) }}
        </span>
        <span v-if="entry.modified_time" class="file-date">
          {{ formatRelativeDate(entry.modified_time) }}
        </span>
      </div>
    </div>

    <!-- Tags Area -->
    <div class="file-tags-container" :style="{ width: tagAreaWidth + 'px' }">
      <div class="resize-handle" @mousedown="startResize"></div>
      <div class="file-tags" @click.stop>
        <FileItemTags :entry="entry" />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { getHighlightRanges, formatBytes, formatRelativeDate, getFileIcon } from '@/utils'
import FileItemTags from './FileItemTags.vue'
import type { FileEntry } from '@/types'

interface Props {
  entry: FileEntry
  selected?: boolean
  tagAreaWidth: number
  highlightQuery?: string
}

const props = defineProps<Props>()

const emit = defineEmits<{
  click: [entry: FileEntry]
  doubleClick: [entry: FileEntry]
  contextMenu: [entry: FileEntry, event: MouseEvent]
  resizeStart: [event: MouseEvent]
}>()

const isSelected = computed(() => props.selected)

const nameSegments = computed(() => {
  if (!props.highlightQuery) return []
  return getHighlightRanges(props.entry.name, props.highlightQuery)
})

function handleClick() {
  emit('click', props.entry)
}

function handleDoubleClick() {
  emit('doubleClick', props.entry)
}

function handleContextMenu(event: MouseEvent) {
  emit('contextMenu', props.entry, event)
}

function startResize(e: MouseEvent) {
  emit('resizeStart', e)
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
  background: rgba(25, 118, 210, 0.08);
}

.file-item.directory {
  font-weight: 500;
}

.file-icon {
  font-size: 20px;
  width: 24px;
  flex-shrink: 0;
  text-align: center;
}

.file-info {
  flex: 0 1 auto;
  min-width: 200px;
  max-width: 400px;
  overflow: hidden;
}

.file-name {
  font-size: 14px;
  color: var(--text-primary);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.search-highlight {
  background: yellow;
  color: black;
  font-weight: 600;
}

.file-meta {
  display: flex;
  gap: 12px;
  margin-top: 2px;
  font-size: 12px;
  color: var(--text-secondary);
}

.file-size,
.file-date {
  white-space: nowrap;
}

.file-tags-container {
  position: relative;
  flex-shrink: 0;
  display: flex;
  align-items: center;
  min-width: 0;
}

.resize-handle {
  position: absolute;
  left: -4px;
  top: 0;
  bottom: 0;
  width: 8px;
  cursor: col-resize;
  z-index: 10;
}

.resize-handle:hover {
  background: rgba(25, 118, 210, 0.1);
}

.file-tags {
  flex: 1;
  min-width: 0;
  overflow: hidden;
}
</style>
