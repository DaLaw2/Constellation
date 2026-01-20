<template>
  <div class="tag-selector">
    <div class="selector-header">
      <span class="selector-title">Select Tags</span>
      <button class="btn-close" @click="emit('close')">×</button>
    </div>

    <div class="selector-body">
      <div v-for="group in tagGroups" :key="group.id" class="tag-group">
        <div class="group-header">
          <span
            class="group-color"
            :style="{ backgroundColor: group.color || '#9e9e9e' }"
          ></span>
          <span class="group-name">{{ group.name }}</span>
        </div>

        <div class="group-tags">
          <label
            v-for="tag in getTagsByGroup(group.id)"
            :key="tag.id"
            class="tag-option"
            :class="{ selected: isSelected(tag.id) }"
          >
            <input
              type="checkbox"
              :checked="isSelected(tag.id)"
              @change="toggleTag(tag.id)"
            />
            <span class="tag-label">{{ tag.value }}</span>
          </label>

          <!-- Create new tag inline -->
          <div v-if="creatingInGroup === group.id" class="new-tag-input">
            <input
              ref="newTagInputRef"
              v-model="newTagValue"
              type="text"
              placeholder="New tag name..."
              @keyup.enter="createTag(group.id)"
              @keyup.escape="cancelCreate"
            />
            <button class="btn-small" @click="createTag(group.id)">Add</button>
            <button class="btn-small btn-cancel" @click="cancelCreate">×</button>
          </div>
          <button
            v-else
            class="btn-add-tag"
            @click="startCreate(group.id)"
          >
            + Add Tag
          </button>
        </div>
      </div>

      <div v-if="tagGroups.length === 0" class="empty-state">
        No tag groups yet. Create groups in the Tag Panel first.
      </div>
    </div>

    <div class="selector-footer">
      <span class="selected-count">{{ selectedTagIds.length }} selected</span>
      <button class="btn btn-primary" @click="emit('close')">Done</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, nextTick } from 'vue'
import type { Tag, TagGroup } from '../../stores/tags'

interface Props {
  selectedTagIds: number[]
  tagGroups: TagGroup[]
  tags: Tag[]
}

const props = defineProps<Props>()

const emit = defineEmits<{
  'update:selected-tag-ids': [tagIds: number[]]
  'create-tag': [groupId: number, value: string]
  'close': []
}>()

const creatingInGroup = ref<number | null>(null)
const newTagValue = ref('')
const newTagInputRef = ref<HTMLInputElement | null>(null)

function getTagsByGroup(groupId: number): Tag[] {
  return props.tags.filter(tag => tag.group_id === groupId)
}

function isSelected(tagId: number): boolean {
  return props.selectedTagIds.includes(tagId)
}

function toggleTag(tagId: number) {
  const newIds = [...props.selectedTagIds]
  const index = newIds.indexOf(tagId)

  if (index === -1) {
    newIds.push(tagId)
  } else {
    newIds.splice(index, 1)
  }

  emit('update:selected-tag-ids', newIds)
}

function startCreate(groupId: number) {
  creatingInGroup.value = groupId
  newTagValue.value = ''
  nextTick(() => {
    newTagInputRef.value?.focus()
  })
}

function cancelCreate() {
  creatingInGroup.value = null
  newTagValue.value = ''
}

function createTag(groupId: number) {
  const value = newTagValue.value.trim()
  if (value) {
    emit('create-tag', groupId, value)
    newTagValue.value = ''
    // Keep the input open for adding more tags
  }
}
</script>

<style scoped>
.tag-selector {
  position: absolute;
  top: 100%;
  left: 0;
  z-index: 100;
  min-width: 280px;
  max-width: 400px;
  max-height: 400px;
  background: var(--background);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.15);
  display: flex;
  flex-direction: column;
}

.selector-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px;
  border-bottom: 1px solid var(--border-color);
}

.selector-title {
  font-weight: 600;
  font-size: 14px;
}

.btn-close {
  width: 24px;
  height: 24px;
  border: none;
  background: none;
  font-size: 18px;
  cursor: pointer;
  color: var(--text-secondary);
  border-radius: 4px;
}

.btn-close:hover {
  background: rgba(0, 0, 0, 0.05);
}

.selector-body {
  flex: 1;
  overflow-y: auto;
  padding: 8px;
}

.tag-group {
  margin-bottom: 12px;
}

.group-header {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 4px 8px;
  margin-bottom: 4px;
}

.group-color {
  width: 10px;
  height: 10px;
  border-radius: 2px;
}

.group-name {
  font-size: 12px;
  font-weight: 600;
  color: var(--text-secondary);
  text-transform: uppercase;
}

.group-tags {
  display: flex;
  flex-direction: column;
  gap: 2px;
  padding-left: 18px;
}

.tag-option {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 6px 8px;
  border-radius: 4px;
  cursor: pointer;
  transition: background 0.15s ease;
}

.tag-option:hover {
  background: rgba(0, 0, 0, 0.04);
}

.tag-option.selected {
  background: rgba(99, 102, 241, 0.1);
}

.tag-option input[type="checkbox"] {
  cursor: pointer;
}

.tag-label {
  font-size: 13px;
}

.new-tag-input {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 4px 8px;
}

.new-tag-input input {
  flex: 1;
  padding: 4px 8px;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  font-size: 12px;
}

.btn-small {
  padding: 4px 8px;
  border: 1px solid var(--border-color);
  background: var(--background);
  border-radius: 4px;
  font-size: 12px;
  cursor: pointer;
}

.btn-small:hover {
  background: var(--surface);
}

.btn-cancel {
  padding: 4px 6px;
}

.btn-add-tag {
  padding: 6px 8px;
  border: none;
  background: none;
  font-size: 12px;
  color: var(--primary-color);
  cursor: pointer;
  text-align: left;
  border-radius: 4px;
}

.btn-add-tag:hover {
  background: rgba(99, 102, 241, 0.08);
}

.empty-state {
  padding: 24px;
  text-align: center;
  color: var(--text-secondary);
  font-size: 13px;
}

.selector-footer {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px;
  border-top: 1px solid var(--border-color);
}

.selected-count {
  font-size: 12px;
  color: var(--text-secondary);
}

.btn {
  padding: 6px 16px;
  border: 1px solid var(--border-color);
  background: var(--background);
  border-radius: 4px;
  font-size: 13px;
  cursor: pointer;
}

.btn-primary {
  background: var(--primary-color);
  border-color: var(--primary-color);
  color: white;
}

.btn-primary:hover {
  opacity: 0.9;
}
</style>
