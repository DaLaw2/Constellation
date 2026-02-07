<template>
  <div class="picture-card" @click="handleClick">
    <img
      v-if="!imageError"
      :src="thumbUrl"
      :alt="entry.name"
      loading="lazy"
      class="card-image"
      @error="handleImageError"
    />
    <div v-else class="card-image card-fallback">
      {{ getFileIcon(entry.name) }}
    </div>
    <div class="card-overlay">
      <div class="card-name">{{ entry.name }}</div>
      <div v-if="itemTags.length > 0" class="card-tags">
        <span 
          v-for="tag in itemTags.slice(0, 3)" 
          :key="tag.id" 
          class="tag-badge"
          :style="{ backgroundColor: getTagGroupColor(tag.group_id) }"
        >
          {{ tag.value }}
        </span>
        <span v-if="itemTags.length > 3" class="tag-more">
          +{{ itemTags.length - 3 }}
        </span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { getThumbnailUrl, getFileIcon } from '@/utils'
import { useItemsStore } from '@/stores/items'
import { useTagsStore } from '@/stores/tags'
import { useSettingsStore } from '@/stores/settings'
import type { FileEntry, Tag } from '@/types'

interface Props {
  entry: FileEntry
}

const props = defineProps<Props>()
const emit = defineEmits<{
  click: [entry: FileEntry]
}>()

const itemsStore = useItemsStore()
const tagsStore = useTagsStore()
const settingsStore = useSettingsStore()

const itemTags = ref<Tag[]>([])
const imageError = ref(false)
const thumbUrl = computed(() => getThumbnailUrl(props.entry.path, settingsStore.settings.thumbnail_size))

onMounted(async () => {
  // Load tags for this image
  const item = await itemsStore.getItemByPath(props.entry.path)
  if (item) {
    itemTags.value = await itemsStore.getTagsForItem(item.id)
  }
})

function getTagGroupColor(groupId: number): string {
  const group = tagsStore.tagGroups.find(g => g.id === groupId)
  return group?.color || '#9e9e9e'
}

function handleClick() {
  emit('click', props.entry)
}

function handleImageError() {
  imageError.value = true
}
</script>

<style scoped>
.picture-card {
  position: relative;
  aspect-ratio: 1 / 1;
  overflow: hidden;
  border-radius: 8px;
  cursor: pointer;
  background: var(--surface);
  transition: transform 0.2s ease, box-shadow 0.2s ease;
}

.picture-card:hover {
  transform: translateY(-4px);
  box-shadow: 0 8px 16px rgba(0, 0, 0, 0.15);
}

.card-image {
  width: 100%;
  height: 100%;
  object-fit: cover;
  display: block;
}

.card-fallback {
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 48px;
  background: var(--surface);
}

.card-overlay {
  position: absolute;
  bottom: 0;
  left: 0;
  right: 0;
  background: linear-gradient(to top, rgba(0, 0, 0, 0.8), transparent);
  padding: 12px;
  opacity: 0;
  transition: opacity 0.2s ease;
}

.picture-card:hover .card-overlay {
  opacity: 1;
}

.card-name {
  font-size: 12px;
  font-weight: 500;
  color: white;
  margin-bottom: 4px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.card-tags {
  display: flex;
  gap: 4px;
  flex-wrap: wrap;
}

.tag-badge {
  font-size: 10px;
  padding: 2px 6px;
  border-radius: 4px;
  color: white;
  font-weight: 500;
}

.tag-more {
  font-size: 10px;
  padding: 2px 6px;
  border-radius: 4px;
  background: rgba(255, 255, 255, 0.2);
  color: white;
  font-weight: 500;
}
</style>
