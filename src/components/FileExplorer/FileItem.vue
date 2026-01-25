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
      <div class="file-name" :title="entry.name">
        <template v-if="nameSegments.length">
          <span 
            v-for="(seg, idx) in nameSegments" 
            :key="idx" 
            :class="{ 'search-highlight': seg.highlight }"
          >{{ seg.text }}</span>
        </template>
        <template v-else>
          {{ entry.name }}
        </template>
      </div>
      <div class="file-meta">
        <span v-if="!entry.is_directory && entry.size !== null" class="file-size">
          {{ formatFileSize(entry.size) }}
        </span>
        <span v-if="entry.modified_time" class="file-date">
          {{ formatRelativeDate(entry.modified_time) }}
        </span>
      </div>
    </div>
    <div class="file-tags-container" :style="{ width: tagAreaWidth + 'px' }">
      <div
        class="resize-handle"
        @mousedown="startResize"
      ></div>
      <div class="file-tags" @click.stop>
        <TagCell
          :item-tags="itemTags"
          :tag-groups="tagGroups"
          :tags="allTags"
          @update:tags="handleTagsUpdate"
          @create-tag="handleCreateTag"
        />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref, watch, onMounted } from 'vue'
import { useTagsStore } from '@/stores/tags'
import { useItemsStore } from '@/stores/items'
import { getHighlightRanges } from '@/utils'
import TagCell from '../TagManagement/TagCell.vue'
import type { FileEntry, Tag } from '@/types'

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

const tagsStore = useTagsStore()
const itemsStore = useItemsStore()

const isSelected = computed(() => props.selected)
const tagGroups = computed(() => tagsStore.tagGroups)
const allTags = computed(() => tagsStore.tags)

const nameSegments = computed(() => {
  if (!props.highlightQuery) return []
  return getHighlightRanges(props.entry.name, props.highlightQuery)
})

// Track tags for this specific file item
const itemTags = ref<Tag[]>([])
const itemId = ref<number | null>(null)

// Load item and its tags when entry changes
watch(() => props.entry.path, async (newPath) => {
  try {
    const item = await itemsStore.getItemByPath(newPath)
    if (item) {
      itemId.value = item.id
      const tags = await itemsStore.getTagsForItem(item.id)
      itemTags.value = tags
    } else {
      itemId.value = null
      itemTags.value = []
    }
  } catch (e) {
    console.error('Failed to load item tags:', e)
    itemTags.value = []
  }
}, { immediate: true })

async function handleTagsUpdate(tagIds: number[]) {
  try {
    if (itemId.value === null) {
      const newId = await itemsStore.createItem(
        props.entry.path,
        props.entry.is_directory,
        props.entry.size,
        props.entry.modified_time
      )
      itemId.value = newId
    }

    await itemsStore.updateItemTags(itemId.value, tagIds)
    const tags = await itemsStore.getTagsForItem(itemId.value)
    itemTags.value = tags
  } catch (e) {
    console.error('Failed to update tags:', e)
  }
}

async function handleCreateTag(groupId: number, value: string) {
  try {
    const newTagId = await tagsStore.createTag(groupId, value)
    if (itemId.value !== null) {
      const currentIds = itemTags.value.map(t => t.id)
      await itemsStore.updateItemTags(itemId.value, [...currentIds, newTagId])
      const tags = await itemsStore.getTagsForItem(itemId.value)
      itemTags.value = tags
    }
  } catch (e) {
    console.error('Failed to create tag:', e)
  }
}

function startResize(e: MouseEvent) {
  emit('resizeStart', e)
}

onMounted(() => {
  if (tagsStore.tagGroups.length === 0) {
    tagsStore.loadTagGroups()
  }
  if (tagsStore.tags.length === 0) {
    tagsStore.loadTags()
  }
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

function getFileIcon(entry: FileEntry): string {
  if (entry.is_directory) {
    return '📁'
  }

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

function formatFileSize(bytes: number): string {
  if (bytes === 0) return '0 B'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return Math.round(bytes / Math.pow(k, i) * 100) / 100 + ' ' + sizes[i]
}

function formatRelativeDate(timestamp: number): string {
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
  flex: 1 1 60%;
  min-width: 100px;
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

.file-tags-container {
  position: relative;
  display: flex;
  flex-shrink: 1;
  min-width: 150px;
}

.resize-handle {
  position: absolute;
  left: 0;
  top: 0;
  bottom: 0;
  width: 4px;
  cursor: col-resize;
  z-index: 5;
  transition: background-color 0.2s ease;
}

.resize-handle:hover {
  background-color: var(--primary-color);
}

.resize-handle::before {
  content: '';
  position: absolute;
  left: -4px;
  right: -4px;
  top: 0;
  bottom: 0;
}

.file-tags {
  flex: 1;
  min-width: 0;
  position: relative;
  padding-left: 8px;
}

.search-highlight {
  background-color: rgba(255, 193, 7, 0.3);
  border-radius: 2px;
  font-weight: 500;
}
</style>
