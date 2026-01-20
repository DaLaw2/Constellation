<template>
  <div class="tag-cell" @click.stop="startEdit">
    <!-- Display mode: Show tags with overflow indicator -->
    <div v-if="!editing" class="tag-display">
      <span v-if="displayTags.length === 0" class="no-tags">
        Click to add tags
      </span>
      <span
        v-for="tag in displayTags"
        :key="tag.id"
        class="tag-badge"
        :style="{ backgroundColor: getTagColor(tag) }"
      >
        {{ tag.value }}
      </span>
      <span v-if="remainingCount > 0" class="tag-overflow">
        +{{ remainingCount }}
      </span>
    </div>

    <!-- Edit mode: Tag selector grouped by tag groups -->
    <div v-else class="tag-editor" @click.stop>
      <TagSelector
        :selected-tag-ids="selectedTagIds"
        :tag-groups="tagGroups"
        :tags="tags"
        @update:selected-tag-ids="handleTagsChange"
        @create-tag="handleCreateTag"
        @close="handleClose"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted, onUnmounted } from 'vue'
import TagSelector from './TagSelector.vue'
import type { Tag, TagGroup } from '../../stores/tags'

interface Props {
  itemTags: Tag[]
  tagGroups: TagGroup[]
  tags: Tag[]
  maxDisplay?: number
}

const props = withDefaults(defineProps<Props>(), {
  maxDisplay: 3
})

const emit = defineEmits<{
  'update:tags': [tagIds: number[]]
  'create-tag': [groupId: number, value: string]
}>()

const editing = ref(false)
const selectedTagIds = ref<number[]>([])

// Sync selectedTagIds with itemTags
watch(() => props.itemTags, (newTags) => {
  selectedTagIds.value = newTags.map(t => t.id)
}, { immediate: true })

const displayTags = computed(() => {
  return props.itemTags.slice(0, props.maxDisplay)
})

const remainingCount = computed(() => {
  return Math.max(0, props.itemTags.length - props.maxDisplay)
})

function getTagColor(tag: Tag): string {
  const group = props.tagGroups.find(g => g.id === tag.group_id)
  return group?.color || '#9e9e9e'
}

function startEdit() {
  editing.value = true
  selectedTagIds.value = props.itemTags.map(t => t.id)
}

function handleTagsChange(tagIds: number[]) {
  selectedTagIds.value = tagIds
  emit('update:tags', tagIds)
}

function handleCreateTag(groupId: number, value: string) {
  emit('create-tag', groupId, value)
}

function handleClose() {
  editing.value = false
}

// Close on click outside
function handleClickOutside(event: MouseEvent) {
  const target = event.target as HTMLElement
  if (editing.value && !target.closest('.tag-cell')) {
    editing.value = false
  }
}

onMounted(() => {
  document.addEventListener('click', handleClickOutside)
})

onUnmounted(() => {
  document.removeEventListener('click', handleClickOutside)
})
</script>

<style scoped>
.tag-cell {
  min-width: 100px;
  min-height: 28px;
  cursor: pointer;
  padding: 2px 4px;
  border-radius: 4px;
  transition: background-color 0.15s ease;
}

.tag-cell:hover {
  background: rgba(0, 0, 0, 0.04);
}

.tag-display {
  display: flex;
  align-items: center;
  gap: 4px;
  flex-wrap: wrap;
}

.no-tags {
  font-size: 12px;
  color: var(--text-secondary);
  font-style: italic;
}

.tag-badge {
  display: inline-flex;
  align-items: center;
  padding: 2px 8px;
  border-radius: 12px;
  font-size: 11px;
  font-weight: 500;
  color: white;
  max-width: 100px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.tag-overflow {
  font-size: 11px;
  color: var(--text-secondary);
  padding: 2px 6px;
  background: var(--surface);
  border-radius: 10px;
}

.tag-editor {
  position: relative;
}
</style>

