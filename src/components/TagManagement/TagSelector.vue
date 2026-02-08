<template>
  <Teleport to="body">
    <div class="tag-selector-overlay" @click="emit('close')">
      <div class="tag-selector" @click.stop>
        <div class="selector-header">
          <span class="selector-title">Select Tags</span>
          <button class="btn-close" @click="emit('close')">×</button>
        </div>

        <div class="selector-body">
          <div class="search-bar-container">
            <input 
              v-model="searchQuery" 
              type="text" 
              placeholder="Search tags..." 
              class="tag-search-input"
              ref="searchInputRef"
            />
          </div>

          <div v-for="group in filteredTagGroups" :key="group.id" class="tag-group">
            <div class="group-header">
              <span
                class="group-color"
                :style="{ backgroundColor: group.color || '#9e9e9e' }"
              ></span>
              <span class="group-name">{{ group.name }}</span>
            </div>

            <div class="group-tags">
              <label
                v-for="tag in getFilteredTagsByGroup(group.id)"
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

              <!-- Create new tag inline (only show when not searching or exact match not found) -->
              <div v-if="creatingInGroup === group.id" class="new-tag-input">
                <input
                  :ref="setNewTagInputRef"
                  v-model="newTagValue"
                  type="text"
                  placeholder="New tag name..."
                  :class="{ error: isDuplicate }"
                  @keyup.enter="createTag(group.id)"
                  @keyup.escape="cancelCreate"
                />
                <button class="btn-small" @click="createTag(group.id)">Add</button>
                <button class="btn-small btn-cancel" @click="cancelCreate">×</button>
              </div>
              <button
                v-else-if="!searchQuery"
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
          <div v-else-if="filteredTagGroups.length === 0" class="empty-state">
            No tags match "{{ searchQuery }}"
          </div>
        </div>

        <div class="selector-footer">
          <span class="selected-count">{{ selectedTagIds.length }} selected</span>
          <button class="btn btn-primary" @click="emit('close')">Done</button>
        </div>
      </div>
    </div>
  </Teleport>
</template>

<script setup lang="ts">
import { ref, nextTick, computed, onMounted, type ComponentPublicInstance } from 'vue'
import type { Tag, TagGroup } from '@/types'

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
const searchInputRef = ref<HTMLInputElement | null>(null)
const isDuplicate = ref(false)
const searchQuery = ref('')

// Use a function ref for the new tag input since it's inside v-for
function setNewTagInputRef(el: Element | ComponentPublicInstance | null) {
  if (el instanceof HTMLInputElement) {
    el.focus()
  }
}

// Focus search on mount
onMounted(() => {
  nextTick(() => {
    searchInputRef.value?.focus()
  })
})

function getTagsByGroup(groupId: number): Tag[] {
  return props.tags.filter(tag => tag.group_id === groupId)
}

function getFilteredTagsByGroup(groupId: number): Tag[] {
  const query = searchQuery.value.trim().toLowerCase()
  const tags = getTagsByGroup(groupId)
  
  if (!query) return tags
  
  return tags.filter(tag => tag.value.toLowerCase().includes(query))
}

const filteredTagGroups = computed(() => {
  const query = searchQuery.value.trim().toLowerCase()
  if (!query) return props.tagGroups
  
  return props.tagGroups.filter(group => {
    const groupTags = getTagsByGroup(group.id)
    return groupTags.some(tag => tag.value.toLowerCase().includes(query))
  })
})

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
  isDuplicate.value = false
  // Focus is handled by the function ref setNewTagInputRef
}

function cancelCreate() {
  creatingInGroup.value = null
  newTagValue.value = ''
  isDuplicate.value = false
}

function createTag(groupId: number) {
  const value = newTagValue.value.trim()
  if (!value) return

  // Check for duplicates (case-insensitive)
  const groupTags = getTagsByGroup(groupId)
  const isDup = groupTags.some(t => t.value.toLowerCase() === value.toLowerCase())

  if (isDup) {
    isDuplicate.value = true
    // Input will be re-focused by setNewTagInputRef when component updates
    return
  }

  emit('create-tag', groupId, value)
  newTagValue.value = ''
  isDuplicate.value = false
  // Keep the input open for adding more tags
}

// Clear duplicate error when typing
import { watch } from 'vue'
watch(newTagValue, () => {
    if (isDuplicate.value) {
        isDuplicate.value = false
    }
})
</script>

<style scoped>
.tag-selector {
  position: fixed;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  z-index: 9999;
  min-width: 600px; /* Increased from 500px */
  max-width: 95vw;
  max-height: 85vh;
  background: var(--background);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  box-shadow: 0 4px 24px rgba(0, 0, 0, 0.25);
  display: flex;
  flex-direction: column;
  font-size: 14px;
}

.selector-header {
  padding: 16px;
  border-bottom: 1px solid var(--border-color);
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.selector-title {
  font-weight: 600;
  font-size: 16px;
}

.btn-close {
  background: none;
  border: none;
  font-size: 24px;
  cursor: pointer;
  color: var(--text-secondary);
  padding: 0 4px;
}

.selector-body {
  padding: 16px;
  overflow-y: auto;
  flex: 1;
  display: flex;
  flex-direction: column;
}

.search-bar-container {
  padding-bottom: 12px;
  position: sticky;
  top: 0;
  background: var(--background);
  z-index: 5;
}

.tag-search-input {
  width: 100%;
  padding: 8px 12px;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  font-size: 14px;
}

.tag-search-input:focus {
  outline: none;
  border-color: var(--primary-color);
}

.tag-group {
  margin-bottom: 20px;
}

.group-header {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 10px;
}

.group-color {
  width: 14px;
  height: 14px;
  border-radius: 3px;
}

.group-name {
  font-weight: 500;
  color: var(--text-primary);
  font-size: 15px;
}

.group-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  padding-left: 22px;
}

.tag-option {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 6px 10px;
  border-radius: 4px;
  cursor: pointer;
  transition: background-color 0.15s;
}

.tag-option:hover {
  background: var(--surface);
}

.tag-option.selected {
  background: rgba(25, 118, 210, 0.1);
}

.tag-label {
  color: var(--text-primary);
  font-size: 14px;
}

.btn-add-tag {
  padding: 6px 12px;
  border: 1px dashed var(--border-color);
  background: transparent;
  border-radius: 4px;
  font-size: 13px;
  color: var(--text-secondary);
  cursor: pointer;
}

.new-tag-input {
  display: flex;
  align-items: center;
  gap: 8px;
}

.new-tag-input input {
  padding: 6px 10px;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  font-size: 14px;
  width: 150px;
}



.selector-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px;
  border-bottom: 1px solid var(--border-color);
}

.selector-title {
  font-weight: 600;
  font-size: 18px; /* Increased */
}

.btn-close {
  width: 32px;
  height: 32px;
  border: none;
  background: none;
  font-size: 24px; /* Increased */
  cursor: pointer;
  color: var(--text-secondary);
  border-radius: 4px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.btn-close:hover {
  background: rgba(0, 0, 0, 0.05);
}

.selector-body {
  flex: 1;
  overflow-y: auto;
  padding: 16px; /* Increased padding */
}

.tag-group {
  margin-bottom: 24px;
}

.group-header {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 4px 8px;
  margin-bottom: 8px;
}

.group-color {
  width: 14px; /* Increased */
  height: 14px; /* Increased */
  border-radius: 3px;
}

.group-name {
  font-size: 14px; /* Increased */
  font-weight: 600;
  color: var(--text-secondary);
  text-transform: uppercase;
}

.group-tags {
  display: flex;
  flex-direction: column;
  gap: 4px;
  padding-left: 24px;
}

.tag-option {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 8px 12px; /* Increased padding */
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
  transform: scale(1.2); /* Make checkbox larger */
}

.tag-label {
  font-size: 15px; /* Increased */
  color: var(--text-primary);
}

.new-tag-input {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 6px 12px;
}

.new-tag-input input {
  flex: 1;
  padding: 8px 12px;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  font-size: 14px; /* Increased */
  transition: border-color 0.2s;
}

.new-tag-input input.error {
  border-color: #ef5350;
  background-color: #ffebee;
}

.new-tag-input input.error:focus {
  outline: none;
  border-color: #ef5350;
  box-shadow: 0 0 0 2px rgba(239, 83, 80, 0.2);
}

.btn-small {
  padding: 6px 12px;
  border: 1px solid var(--border-color);
  background: var(--background);
  border-radius: 4px;
  font-size: 13px; /* Increased */
  cursor: pointer;
}

.btn-small:hover {
  background: var(--surface);
}

.btn-cancel {
  padding: 6px 10px;
}

.btn-add-tag {
  padding: 8px 12px;
  border: none;
  background: none;
  font-size: 14px; /* Increased */
  color: var(--primary-color);
  cursor: pointer;
  text-align: left;
  border-radius: 4px;
}

.btn-add-tag:hover {
  background: rgba(99, 102, 241, 0.08);
}

.empty-state {
  padding: 32px;
  text-align: center;
  color: var(--text-secondary);
  font-size: 15px; /* Increased */
}

.selector-footer {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px;
  border-top: 1px solid var(--border-color);
}

.selected-count {
  font-size: 14px; /* Increased */
  color: var(--text-secondary);
}

.btn {
  padding: 8px 20px; /* Increased padding */
  border: 1px solid var(--border-color);
  background: var(--background);
  border-radius: 4px;
  font-size: 14px; /* Increased */
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

.tag-selector-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  background: rgba(0, 0, 0, 0.3);
  z-index: 9998;
  display: flex;
  align-items: center;
  justify-content: center;
}
</style>
