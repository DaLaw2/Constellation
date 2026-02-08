<template>
  <div
    class="grid-file-card"
    :class="{ selected: selected }"
    :style="{ padding: `${cardPadding}px` }"
    @click="handleClick"
    @dblclick="handleOpen"
    @contextmenu.prevent="handleContextMenu"
  >
    <!-- Thumbnail preview: Media files and directories -->
    <img
      v-if="hasThumbnail && !imageError"
      :src="thumbUrl"
      :alt="file.name"
      loading="lazy"
      class="file-thumbnail"
      :style="{ height: `${thumbnailHeight}px` }"
      @error="handleImageError"
    />

    <!-- Fallback on thumbnail error -->
    <div
      v-else-if="hasThumbnail && imageError"
      class="file-icon"
      :style="{ fontSize: `${iconSize}px`, height: `${iconHeight}px`, maxHeight: `${iconHeight}px` }"
    >
      {{ file.is_directory ? '📁' : fileIcon }}
    </div>
    
    <!-- Other files: Show file type icon -->
    <div 
      v-else 
      class="file-icon"
      :style="{ fontSize: `${iconSize}px`, height: `${iconHeight}px`, maxHeight: `${iconHeight}px` }"
    >
      {{ fileIcon }}
    </div>
    
    <div class="file-name" :style="{ fontSize: `${fontSize}px` }" :title="file.name">{{ file.name }}</div>

    <!-- Tags section -->
    <div
      v-if="showTags"
      class="tags-section"
      :style="{ fontSize: `${Math.max(10, fontSize - 2)}px` }"
      @click.stop
    >
      <TagCell
        :item-tags="itemTags"
        :tag-groups="tagGroups"
        :tags="allTags"
        @update:tags="handleTagsUpdate"
        @create-tag="handleCreateTag"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'
import { getFileIcon, isImageFile, isVideoFile, getThumbnailUrl } from '@/utils'
import { useItemsStore } from '@/stores/items'
import { useTagsStore } from '@/stores/tags'
import { useSettingsStore } from '@/stores/settings'
import TagCell from '../TagManagement/TagCell.vue'
import type { FileEntry, Tag } from '@/types'

interface Props {
  file: FileEntry
  zoomLevel?: number
  tags: Tag[]
  selected?: boolean
  showTags?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  zoomLevel: 100,
  tags: () => [],
  selected: false,
  showTags: true
})

const emit = defineEmits<{
  click: [file: FileEntry, event: MouseEvent]
  open: [file: FileEntry]
  contextmenu: [event: MouseEvent, file: FileEntry]
}>()

const itemsStore = useItemsStore()
const tagsStore = useTagsStore()
const settingsStore = useSettingsStore()

const imageError = ref(false)
const itemId = ref<number | null>(null)

const isImage = computed(() => !props.file.is_directory && isImageFile(props.file.name))
const isVideo = computed(() => !props.file.is_directory && isVideoFile(props.file.name))
const isMedia = computed(() => isImage.value || isVideo.value)
const hasThumbnail = computed(() => isMedia.value || props.file.is_directory)
const fileIcon = computed(() => getFileIcon(props.file.name))
const thumbUrl = computed(() => getThumbnailUrl(props.file.path, settingsStore.settings.thumbnail_size))
const tagGroups = computed(() => tagsStore.tagGroups)
const allTags = computed(() => tagsStore.tags)
const itemTags = computed(() => props.tags)

// Dynamic sizing based on zoom level
const thumbnailHeight = computed(() => Math.floor(120 * (props.zoomLevel / 100)))
const iconSize = computed(() => Math.floor(64 * (props.zoomLevel / 100)))
const iconHeight = computed(() => Math.floor(80 * (props.zoomLevel / 100)))
const cardPadding = computed(() => Math.floor(12 * (props.zoomLevel / 100)))
const fontSize = computed(() => Math.floor(12 * (props.zoomLevel / 100)))

async function handleTagsUpdate(tagIds: number[]) {
  try {
    // If item doesn't exist in DB, create it first
    if (!itemId.value) {
      const item = await itemsStore.getItemByPath(props.file.path)
      if (item) {
        itemId.value = item.id
      } else {
        const id = await itemsStore.createItem(
          props.file.path,
          props.file.is_directory,
          props.file.size,
          props.file.modified_time
        )
        itemId.value = id
      }
    }

    // Update tags (triggers itemTagsVersion increment for cache refresh)
    await itemsStore.updateItemTags(itemId.value!, tagIds)
  } catch (error) {
    console.error('Failed to update tags:', error)
  }
}

async function handleCreateTag(groupId: number, value: string) {
  try {
    await tagsStore.createTag(groupId, value)
    await tagsStore.loadTags()
  } catch (error) {
    console.error('Failed to create tag:', error)
  }
}

function handleClick(event: MouseEvent) {
  emit('click', props.file, event)
}

function handleOpen() {
  emit('open', props.file)
}

function handleContextMenu(event: MouseEvent) {
  emit('contextmenu', event, props.file)
}

function handleImageError() {
  imageError.value = true
}
</script>

<style scoped>
.grid-file-card {
  display: flex;
  flex-direction: column;
  align-items: center;
  border-radius: 8px;
  cursor: pointer;
  transition: transform 0.2s ease, background 0.2s ease;
  user-select: none;
  height: fit-content;
  align-self: start;
}

.grid-file-card:hover {
  background: rgba(0, 0, 0, 0.04);
  transform: translateY(-2px);
}

.grid-file-card.selected {
  background: rgba(25, 118, 210, 0.12);
  outline: 2px solid var(--primary-color);
  outline-offset: -2px;
}

.file-thumbnail {
  width: 100%;
  object-fit: cover;
  border-radius: 4px;
  margin-bottom: 8px;
  background: var(--surface);
}

.file-icon {
  margin-bottom: 8px;
  line-height: 1;
  display: flex;
  align-items: center;
  justify-content: center;
}

.file-name {
  text-align: center;
  overflow: hidden;
  display: -webkit-box;
  -webkit-line-clamp: 4;
  -webkit-box-orient: vertical;
  word-break: break-all;
  width: 100%;
  color: var(--text-primary);
  margin-bottom: 4px;
}

.tags-section {
  width: 100%;
  min-height: 20px;
}

/* Override TagCell font sizes to scale with zoom */
.tags-section :deep(.tag-badge) {
  font-size: inherit !important;
}

.tags-section :deep(.no-tags) {
  font-size: inherit !important;
}

.tags-section :deep(.tag-overflow) {
  font-size: inherit !important;
}
</style>
