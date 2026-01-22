<template>
  <div class="tag-cell">
    <!-- Tag display (always visible, dimmed during editing) -->
    <div class="tag-cell-display" :class="{ editing: editing }">
      <!-- Left: Tags display area (non-clickable) -->
      <div
        ref="tagsContainerRef"
        class="tags-display-area"
      >
        <span v-if="sortedTags.length === 0" class="no-tags">
          No tags
        </span>
        <span
          v-for="tag in displayTags"
          :key="tag.id"
          class="tag-badge-wrapper"
        >
          <span
            class="tag-badge"
            :style="{ backgroundColor: getTagColor(tag) }"
            :title="tag.value"
          >
            <span class="tag-text">{{ tag.value }}</span>
            <button
              v-if="!editing"
              class="tag-delete-btn"
              @click.stop="handleDeleteTag(tag.id)"
              title="Remove tag"
            >
              <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <line x1="18" y1="6" x2="6" y2="18"></line>
                <line x1="6" y1="6" x2="18" y2="18"></line>
              </svg>
            </button>
          </span>
        </span>
        <span v-if="remainingCount > 0" class="tag-overflow">
          +{{ remainingCount }}
        </span>
      </div>

      <!-- Right: Add Tag button (always visible, dimmed during editing) -->
      <button class="add-tag-btn" @click.stop="startEdit" title="Add tags">
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <line x1="12" y1="5" x2="12" y2="19"></line>
          <line x1="5" y1="12" x2="19" y2="12"></line>
        </svg>
      </button>
    </div>

    <!-- Edit mode: Tag selector (overlay) -->
    <div v-if="editing" class="tag-editor" @click.stop>
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
import { getTagTextWidth } from '@/utils'
import { TAG_DISPLAY } from '@/constants'
import type { Tag, TagGroup } from '@/types'

interface Props {
  itemTags: Tag[]
  tagGroups: TagGroup[]
  tags: Tag[]
  maxDisplay?: number
}

const props = withDefaults(defineProps<Props>(), {
  maxDisplay: 999
})

const emit = defineEmits<{
  'update:tags': [tagIds: number[]]
  'create-tag': [groupId: number, value: string]
}>()

const editing = ref(false)
const selectedTagIds = ref<number[]>([])
const tagsContainerRef = ref<HTMLElement | null>(null)
const visibleCount = ref<number>(TAG_DISPLAY.DEFAULT_VISIBLE_COUNT)
const containerWidth = ref(0)
let resizeObserver: ResizeObserver | null = null

// Sync selectedTagIds with itemTags
watch(() => props.itemTags, (newTags) => {
  selectedTagIds.value = newTags.map(t => t.id)
}, { immediate: true })

// Sort tags by group order, then alphabetically within group
const sortedTags = computed(() => {
  const tags = [...props.itemTags]

  return tags.sort((a, b) => {
    const groupA = props.tagGroups.find(g => g.id === a.group_id)
    const groupB = props.tagGroups.find(g => g.id === b.group_id)

    const orderA = groupA?.display_order ?? 999
    const orderB = groupB?.display_order ?? 999

    if (orderA !== orderB) {
      return orderA - orderB
    }

    return a.value.toLowerCase().localeCompare(b.value.toLowerCase())
  })
})

const displayTags = computed(() => {
  return sortedTags.value.slice(0, visibleCount.value)
})

const remainingCount = computed(() => {
  return Math.max(0, sortedTags.value.length - visibleCount.value)
})

function calculateVisibleTags() {
  if (!tagsContainerRef.value || sortedTags.value.length === 0) return

  const availableWidth = tagsContainerRef.value.clientWidth

  let currentWidth = 0
  let count = 0

  for (let i = 0; i < sortedTags.value.length; i++) {
    const tag = sortedTags.value[i]
    const tagWidth = getTagTextWidth(tag.value, TAG_DISPLAY.MAX_TAG_WIDTH)
    const isLast = i === sortedTags.value.length - 1

    let spaceForTag = availableWidth - currentWidth
    if (!isLast) {
      spaceForTag -= (TAG_DISPLAY.TAG_GAP + TAG_DISPLAY.OVERFLOW_BADGE_WIDTH)
    }

    if (spaceForTag >= tagWidth) {
      currentWidth += tagWidth + TAG_DISPLAY.TAG_GAP
      count++
    } else if (spaceForTag >= TAG_DISPLAY.MIN_TAG_WIDTH) {
      count++
      break
    } else {
      break
    }
  }

  visibleCount.value = Math.max(0, count)
}

// Recalculate when tags change or width changes
watch([sortedTags, containerWidth], () => {
  calculateVisibleTags()
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

function handleDeleteTag(tagId: number) {
  const updatedIds = selectedTagIds.value.filter(id => id !== tagId)
  emit('update:tags', updatedIds)
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

  if (tagsContainerRef.value) {
    resizeObserver = new ResizeObserver(entries => {
      for (const entry of entries) {
        containerWidth.value = entry.contentRect.width
      }
    })
    resizeObserver.observe(tagsContainerRef.value)
  }
})

onUnmounted(() => {
  document.removeEventListener('click', handleClickOutside)
  if (resizeObserver) {
    resizeObserver.disconnect()
  }
})
</script>

<style scoped>
.tag-cell {
  width: 100%;
  min-height: 32px;
  display: flex;
  align-items: center;
  position: relative;
}

.tag-cell-display {
  display: flex;
  align-items: center;
  gap: 8px;
  width: 100%;
  min-height: 32px;
  transition: opacity 0.2s ease;
}

.tag-cell-display.editing {
  opacity: 0.3;
  pointer-events: none;
}

.tags-display-area {
  flex: 1;
  min-width: 0;
  display: flex;
  align-items: center;
  gap: 4px;
  min-height: 32px;
  padding: 4px 8px;
  border-radius: 4px;
  overflow: hidden;
}

.no-tags {
  font-size: 12px;
  color: var(--text-secondary);
  font-style: italic;
}

.tag-badge-wrapper {
  display: inline-flex;
  position: relative;
  min-width: 0;
  max-width: 100%;
}

.tag-badge {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  padding: 3px 10px;
  padding-right: 8px;
  border-radius: 12px;
  font-size: 11px;
  font-weight: 500;
  color: white;
  max-width: 120px;
  min-width: 30px;
  transition: padding-right 0.2s ease;
}

.tag-text {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  flex: 1;
  min-width: 0;
}

.tag-badge-wrapper:hover .tag-badge {
  padding-right: 24px;
}

.tag-delete-btn {
  position: absolute;
  right: 4px;
  top: 50%;
  transform: translateY(-50%);
  width: 16px;
  height: 16px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(0, 0, 0, 0.2);
  border: none;
  border-radius: 50%;
  cursor: pointer;
  opacity: 0;
  transition: opacity 0.2s ease, background 0.15s ease;
  color: white;
  padding: 0;
}

.tag-badge-wrapper:hover .tag-delete-btn {
  opacity: 1;
}

.tag-delete-btn:hover {
  background: rgba(0, 0, 0, 0.4);
}

.tag-delete-btn svg {
  flex-shrink: 0;
}

.tag-overflow {
  font-size: 11px;
  color: var(--text-secondary);
  padding: 3px 8px;
  background: var(--surface);
  border-radius: 10px;
  font-weight: 500;
}

.add-tag-btn {
  flex-shrink: 0;
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  border: 1px solid var(--border-color);
  background: var(--background);
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.15s ease;
  color: var(--text-secondary);
}

.add-tag-btn:hover {
  background: var(--primary-color);
  border-color: var(--primary-color);
  color: white;
  transform: scale(1.05);
}

.add-tag-btn:active {
  transform: scale(0.95);
}

.add-tag-btn svg {
  flex-shrink: 0;
}

.tag-editor {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  z-index: 10;
}
</style>
