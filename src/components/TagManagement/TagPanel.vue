<template>
  <div class="tag-panel">
    <div class="panel-header">
      <h3>Tag Groups</h3>
      <button class="btn btn-primary" @click="showCreateGroupDialog = true">
        + New Group
      </button>
    </div>

    <div v-if="loading" class="loading-state">
      Loading...
    </div>

    <div v-else-if="error" class="error-state">
      Error: {{ error }}
    </div>

    <div v-else-if="tagGroups.length === 0" class="empty-state">
      <div class="empty-state-icon">🏷️</div>
      <div class="empty-state-title">No Tag Groups</div>
      <div class="empty-state-description">
        Create your first tag group to start organizing files
      </div>
    </div>

    <div v-else class="tag-groups-list">
      <div 
        v-for="(group, index) in tagGroups" 
        :key="group.id" 
        class="tag-group-item"
        :class="{ 'dragging': draggingIndex === index, 'drag-over': dragOverIndex === index }"
        draggable="true"
        @dragstart="handleDragStart(index, $event)"
        @dragover.prevent="handleDragOver(index, $event)"
        @drop="handleDrop(index, $event)"
        @dragend="handleDragEnd"
      >
        <div class="tag-group-header">
          <div class="group-info">
            <span class="drag-handle" title="Drag to reorder">⋮⋮</span>
            <span
              class="group-color-badge"
              :style="{ backgroundColor: group.color || '#9e9e9e' }"
            ></span>
            <span class="group-name">{{ group.name }}</span>
          </div>
          <button class="btn-icon" @click="toggleGroup(group.id)">
            {{ expandedGroups.has(group.id) ? '▼' : '▶' }}
          </button>
        </div>

        <div v-if="expandedGroups.has(group.id)" class="tag-list">
          <div v-for="tag in getTagsByGroup(group.id)" :key="tag.id" class="tag-item">
            <span class="tag-value">{{ tag.value }}</span>
            <span
              class="tag-count-badge"
              :class="{ 'tag-count-zero': !usageCounts[tag.id] }"
              :title="getUsageTooltip(tag.id)"
            >
              {{ usageCounts[tag.id] || 0 }}
            </span>
          </div>
          <button class="btn-add-tag" @click="showAddTagDialog(group.id)">
            + Add Tag
          </button>
        </div>
      </div>
    </div>

    <!-- Create Group Dialog -->
    <div v-if="showCreateGroupDialog" class="dialog-overlay" @click.self="showCreateGroupDialog = false">
      <div class="dialog">
        <h3>Create Tag Group</h3>
        <div class="form-group">
          <label>Name:</label>
          <input type="text" v-model="newGroupName" placeholder="e.g., Language" />
        </div>
        <div class="form-group">
          <label>Color:</label>
          <div class="color-presets">
            <button
              v-for="color in presetColors"
              :key="color"
              class="color-swatch"
              :style="{ backgroundColor: color }"
              :class="{ selected: newGroupColor === color }"
              @click="newGroupColor = color"
            ></button>
          </div>
          <div class="color-input-wrapper">
             <input type="color" v-model="newGroupColor" />
             <span class="color-value">{{ newGroupColor }}</span>
          </div>
        </div>
        <div class="dialog-actions">
          <button class="btn" @click="showCreateGroupDialog = false">Cancel</button>
          <button class="btn btn-primary" @click="createGroup">Create</button>
        </div>
      </div>
    </div>

    <!-- Create Tag Dialog -->
    <div v-if="showTagDialog" class="dialog-overlay" @click.self="showTagDialog = false">
      <div class="dialog">
        <h3>Create Tag</h3>
        <div class="form-group">
          <label>Tag Value:</label>
          <input
            type="text"
            v-model="newTagValue"
            placeholder="e.g., English"
            @input="handleTagSearchDebounced"
            @keydown.enter.prevent="handleEnterKey"
            @keydown.down.prevent="navigateSuggestion(1)"
            @keydown.up.prevent="navigateSuggestion(-1)"
            @keydown.escape="closeSuggestions"
            ref="tagInput"
          />
          <div v-if="searchResults.length > 0" class="search-suggestions">
            <div class="suggestion-label">Similar existing tags:</div>
            <div
              v-for="(res, index) in searchResults"
              :key="res.id"
              class="suggestion-item"
              :class="{ 'suggestion-selected': index === selectedSuggestionIndex }"
              @click="selectSuggestion(res)"
              @mouseenter="selectedSuggestionIndex = index"
            >
              <span class="suggestion-value">{{ res.value }}</span>
              <span class="suggestion-group">{{ getGroupName(res.group_id) }}</span>
            </div>
          </div>
        </div>
        <div class="dialog-actions">
          <button class="btn" @click="showTagDialog = false">Cancel</button>
          <button class="btn btn-primary" @click="createTag">Create</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { useTagsStore, type Tag } from '../../stores/tags'

const tagsStore = useTagsStore()

const tagGroups = computed(() => tagsStore.tagGroups)
const loading = computed(() => tagsStore.loading)
const error = computed(() => tagsStore.error)

const expandedGroups = ref<Set<number>>(new Set())
const showCreateGroupDialog = ref(false)
const newGroupName = ref('')
const newGroupColor = ref('#3B82F6')

// Drag and drop state
const draggingIndex = ref<number | null>(null)
const dragOverIndex = ref<number | null>(null)

const presetColors = [
  '#ef5350', // Red
  '#ec407a', // Pink
  '#ab47bc', // Purple
  '#7e57c2', // Deep Purple
  '#5c6bc0', // Indigo
  '#42a5f5', // Blue
  '#29b6f6', // Light Blue
  '#26c6da', // Cyan
  '#26a69a', // Teal
  '#66bb6a', // Green
  '#9ccc65', // Light Green
  '#d4e157', // Lime
  '#ffee58', // Yellow
  '#ffca28', // Amber
  '#ffa726', // Orange
  '#ff7043', // Deep Orange
]

onMounted(() => {
  tagsStore.loadTagGroups()
  tagsStore.loadTags()
  tagsStore.loadUsageCounts()
})

const usageCounts = computed(() => tagsStore.usageCounts)

function toggleGroup(groupId: number) {
  if (expandedGroups.value.has(groupId)) {
    expandedGroups.value.delete(groupId)
  } else {
    expandedGroups.value.add(groupId)
  }
}

function getTagsByGroup(groupId: number) {
  return tagsStore.getTagsByGroup(groupId)
}

function getUsageTooltip(tagId: number): string {
  const count = usageCounts.value[tagId] || 0
  return count === 1 ? 'Used by 1 file' : `Used by ${count} files`
}

async function createGroup() {
  if (!newGroupName.value.trim()) return

  try {
    await tagsStore.createTagGroup(newGroupName.value.trim(), newGroupColor.value)
    newGroupName.value = ''
    newGroupColor.value = '#3B82F6'
    showCreateGroupDialog.value = false
  } catch (e) {
    console.error('Failed to create group:', e)
  }
}

// Drag and drop handlers
function handleDragStart(index: number, event: DragEvent) {
  draggingIndex.value = index
  if (event.dataTransfer) {
    event.dataTransfer.effectAllowed = 'move'
    event.dataTransfer.setData('text/plain', index.toString())
  }
}

function handleDragOver(index: number, event: DragEvent) {
  event.preventDefault()
  if (event.dataTransfer) {
    event.dataTransfer.dropEffect = 'move'
  }
  dragOverIndex.value = index
}

function handleDrop(dropIndex: number, event: DragEvent) {
  event.preventDefault()
  
  if (draggingIndex.value === null || draggingIndex.value === dropIndex) {
    return
  }

  const dragIndex = draggingIndex.value
  const groups = [...tagGroups.value]
  
  // Reorder the array
  const [draggedItem] = groups.splice(dragIndex, 1)
  groups.splice(dropIndex, 0, draggedItem)
  
  // Update backend with new order
  const orderedIds = groups.map(g => g.id)
  tagsStore.reorderTagGroups(orderedIds)
}

function handleDragEnd() {
  draggingIndex.value = null
  dragOverIndex.value = null
}

// Tag Creation / Autocomplete State
const showTagDialog = ref(false)
const targetGroupId = ref<number | null>(null)
const newTagValue = ref('')
const searchResults = ref<Tag[]>([])
const selectedSuggestionIndex = ref(-1)
let searchDebounceTimer: ReturnType<typeof setTimeout> | null = null

function showAddTagDialog(groupId: number) {
  targetGroupId.value = groupId
  newTagValue.value = ''
  searchResults.value = []
  selectedSuggestionIndex.value = -1
  showTagDialog.value = true
}

function handleTagSearchDebounced() {
  if (searchDebounceTimer) {
    clearTimeout(searchDebounceTimer)
  }
  searchDebounceTimer = setTimeout(handleTagSearch, 300)
}

async function handleTagSearch() {
  if (!newTagValue.value.trim()) {
    searchResults.value = []
    selectedSuggestionIndex.value = -1
    return
  }
  // Global search (no groupId) to catch duplicates across all groups
  searchResults.value = await tagsStore.searchTags(newTagValue.value)
  selectedSuggestionIndex.value = -1
}

function getGroupName(groupId: number): string {
  const group = tagGroups.value.find(g => g.id === groupId)
  return group ? group.name : ''
}

function navigateSuggestion(direction: number) {
  if (searchResults.value.length === 0) return

  const newIndex = selectedSuggestionIndex.value + direction
  if (newIndex >= -1 && newIndex < searchResults.value.length) {
    selectedSuggestionIndex.value = newIndex
  }
}

function selectSuggestion(tag: Tag) {
  newTagValue.value = tag.value
  searchResults.value = []
  selectedSuggestionIndex.value = -1
}

function handleEnterKey() {
  if (selectedSuggestionIndex.value >= 0 && selectedSuggestionIndex.value < searchResults.value.length) {
    selectSuggestion(searchResults.value[selectedSuggestionIndex.value])
  } else {
    createTag()
  }
}

function closeSuggestions() {
  searchResults.value = []
  selectedSuggestionIndex.value = -1
}

onUnmounted(() => {
  if (searchDebounceTimer) {
    clearTimeout(searchDebounceTimer)
  }
})

async function createTag() {
  if (!newTagValue.value.trim() || !targetGroupId.value) return
  
  try {
    await tagsStore.createTag(targetGroupId.value, newTagValue.value.trim())
    showTagDialog.value = false
    newTagValue.value = ''
    searchResults.value = []
  } catch (e) {
    console.error('Failed to create tag:', e)
  }
}
</script>

<style scoped>
.tag-panel {
  height: 100%;
  display: flex;
  flex-direction: column;
}

.panel-header {
  padding: 1rem;
  border-bottom: 1px solid var(--border-color);
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.panel-header h3 {
  font-size: 14px;
  font-weight: 600;
}

.loading-state,
.error-state {
  padding: 2rem;
  text-align: center;
  color: var(--text-secondary);
}

.error-state {
  color: #d32f2f;
}

.tag-groups-list {
  flex: 1;
  overflow: auto;
  padding: 0.5rem;
}

.tag-group-item {
  margin-bottom: 0.5rem;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background: var(--background);
  cursor: move;
  transition: opacity 0.2s, border-color 0.2s;
}

.tag-group-item.dragging {
  opacity: 0.5;
}

.tag-group-item.drag-over {
  border-color: var(--primary-color);
  border-width: 2px;
}

.tag-group-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0.75rem;
}

.tag-group-header:hover {
  background: var(--surface);
}

.group-info {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.drag-handle {
  cursor: grab;
  color: var(--text-secondary);
  font-size: 14px;
  opacity: 0.5;
  user-select: none;
}

.drag-handle:active {
  cursor: grabbing;
}

.tag-group-item:hover .drag-handle {
  opacity: 1;
}

.group-color-badge {
  width: 12px;
  height: 12px;
  border-radius: 2px;
}

.group-name {
  font-weight: 500;
  font-size: 13px;
}

.tag-list {
  padding: 0.5rem;
  border-top: 1px solid var(--border-color);
  background: var(--surface);
}

.tag-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0.5rem;
  margin-bottom: 0.25rem;
  background: var(--background);
  border-radius: 4px;
  font-size: 12px;
}

.tag-value {
  color: var(--text-primary);
}

.btn-add-tag {
  width: 100%;
  padding: 0.5rem;
  margin-top: 0.5rem;
  border: 1px dashed var(--border-color);
  background: transparent;
  border-radius: 4px;
  font-size: 12px;
  color: var(--text-secondary);
  cursor: pointer;
  transition: var(--transition-fast);
}

.btn-add-tag:hover {
  border-color: var(--primary-color);
  color: var(--primary-color);
  background: rgba(25, 118, 210, 0.04);
}

.dialog-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.dialog {
  background: var(--background);
  border-radius: 8px;
  padding: 1.5rem;
  width: 400px;
  max-width: 90vw;
  box-shadow: 0 10px 30px rgba(0, 0, 0, 0.3);
}

.dialog h3 {
  margin-bottom: 1rem;
  font-size: 18px;
}

.form-group {
  margin-bottom: 1rem;
}

.form-group label {
  display: block;
  margin-bottom: 0.5rem;
  font-size: 13px;
  font-weight: 500;
}

.form-group input {
  width: 100%;
  padding: 8px 12px;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  font-size: 14px;
}

.form-group input[type="color"] {
  height: 40px;
  padding: 4px;
  cursor: pointer;
}

.color-presets {
  display: grid;
  grid-template-columns: repeat(8, 1fr);
  gap: 8px;
  margin-bottom: 12px;
}

.color-swatch {
  width: 24px;
  height: 24px;
  border-radius: 4px;
  border: 2px solid transparent;
  cursor: pointer;
  transition: transform 0.1s;
}

.color-swatch:hover {
  transform: scale(1.1);
}

.color-swatch.selected {
  border-color: var(--text-primary);
  transform: scale(1.1);
}

.color-input-wrapper {
  display: flex;
  align-items: center;
  gap: 12px;
}

.color-value {
  font-size: 13px;
  font-family: monospace;
  color: var(--text-secondary);
}

.dialog-actions {
  display: flex;
  gap: 0.5rem;
  justify-content: flex-end;
  margin-top: 1.5rem;
}

.tag-count-badge {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  min-width: 20px;
  height: 18px;
  padding: 0 6px;
  margin-left: 8px;
  font-size: 11px;
  font-weight: 500;
  color: var(--primary-color);
  background: rgba(25, 118, 210, 0.1);
  border-radius: 9px;
  cursor: default;
}

.tag-count-badge.tag-count-zero {
  color: var(--text-secondary);
  background: rgba(128, 128, 128, 0.1);
  opacity: 0.7;
}

.search-suggestions {
  margin-top: 8px;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  max-height: 150px;
  overflow-y: auto;
  background: var(--surface);
}

.suggestion-label {
  padding: 4px 8px;
  font-size: 11px;
  color: var(--text-secondary);
  background: rgba(0,0,0,0.02);
  border-bottom: 1px solid var(--border-color);
}

.suggestion-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 8px 12px;
  font-size: 13px;
  border-bottom: 1px solid var(--border-color);
  cursor: pointer;
  transition: background-color 0.15s ease;
}

.suggestion-item:last-child {
  border-bottom: none;
}

.suggestion-item:hover,
.suggestion-item.suggestion-selected {
  background: rgba(25, 118, 210, 0.08);
}

.suggestion-value {
  font-weight: 500;
  color: var(--text-primary);
}

.suggestion-group {
  font-size: 11px;
  color: var(--text-secondary);
  padding: 2px 6px;
  background: rgba(128, 128, 128, 0.1);
  border-radius: 4px;
}
</style>
