<template>
  <div class="advanced-filter-panel" :class="{ expanded: isExpanded }">
    <div class="panel-content">
      <!-- Horizontal sections -->
      <div class="filter-sections">
        <!-- Tag Groups Section -->
        <div class="filter-section tag-groups-section">
          <h4 class="section-title">Tag Groups</h4>
          <div class="section-content">
            <div v-if="tagGroups.length === 0" class="empty-hint">
              No tag groups available
            </div>
            <div v-else class="tag-groups-container">
              <div
                v-for="group in tagGroups"
                :key="group.id"
                class="tag-group-block"
              >
                <div class="group-header-inline">
                  <span
                    class="group-color-dot"
                    :style="{ backgroundColor: group.color || '#9e9e9e' }"
                  ></span>
                  <span class="group-name-text">{{ group.name }}</span>
                </div>
                <div class="tags-inline">
                  <label
                    v-for="tag in getDisplayTags(group.id)"
                    :key="tag.id"
                    class="tag-chip"
                    :class="{ selected: isTagSelected(tag.id) }"
                  >
                    <input
                      type="checkbox"
                      :checked="isTagSelected(tag.id)"
                      @change="toggleTag(tag.id)"
                    />
                    <span>{{ tag.value }}</span>
                  </label>
                  <button
                    v-if="hasMoreTags(group.id)"
                    class="more-tags-btn"
                    @click="expandGroup(group.id)"
                  >
                    + More...
                  </button>
                </div>
              </div>
            </div>
          </div>
        </div>

        <!-- File Types Section -->
        <div class="filter-section file-types-section">
          <h4 class="section-title">File Types</h4>
          <div class="section-content">
            <div class="file-type-checkboxes">
              <label
                v-for="fileType in fileTypes"
                :key="fileType.value"
                class="checkbox-item"
                :class="{ checked: selectedFileTypes.includes(fileType.value) }"
              >
                <input
                  type="checkbox"
                  :value="fileType.value"
                  v-model="selectedFileTypes"
                />
                <span>{{ fileType.label }}</span>
              </label>
            </div>
          </div>
        </div>

        <!-- Attributes Section -->
        <div class="filter-section attributes-section">
          <h4 class="section-title">Attributes</h4>
          <div class="section-content">
            <!-- Size Filter -->
            <div class="attribute-group">
              <label class="attribute-label">Size</label>
              <div class="size-filter">
                <input
                  type="number"
                  v-model.number="minSize"
                  placeholder="Min (MB)"
                  class="size-input"
                />
                <span class="separator">-</span>
                <input
                  type="number"
                  v-model.number="maxSize"
                  placeholder="Max (MB)"
                  class="size-input"
                />
              </div>
            </div>

            <!-- Modified Date Filter -->
            <div class="attribute-group">
              <label class="attribute-label">Modified</label>
              <div class="date-filter">
                <input
                  type="date"
                  v-model="modifiedAfter"
                  class="date-input"
                  placeholder="After"
                />
                <span class="separator">-</span>
                <input
                  type="date"
                  v-model="modifiedBefore"
                  class="date-input"
                  placeholder="Before"
                />
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Actions Bar -->
      <div class="actions-bar">
        <button class="btn-clear" @click="clearAllFilters">
          Clear All Filters
        </button>
        <button class="btn-apply" @click="applyFilters">Apply Filters</button>
      </div>
    </div>

    <!-- Expanded Tag Group Modal -->
    <div v-if="expandedGroupId !== null" class="expanded-modal-overlay" @click="closeExpandedGroup">
      <div class="expanded-modal" @click.stop>
        <div class="modal-header">
          <h3>{{ getGroupName(expandedGroupId) }}</h3>
          <button class="close-btn" @click="closeExpandedGroup">×</button>
        </div>
        <div class="modal-content">
          <div class="tags-grid">
            <label
              v-for="tag in getAllTags(expandedGroupId)"
              :key="tag.id"
              class="tag-chip"
              :class="{ selected: isTagSelected(tag.id) }"
            >
              <input
                type="checkbox"
                :checked="isTagSelected(tag.id)"
                @change="toggleTag(tag.id)"
              />
              <span>{{ tag.value }}</span>
            </label>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useTagsStore } from '@/stores/tags'
import { useSearchStore } from '@/stores/search'
import type { Tag } from '@/types'

interface Props {
  isExpanded: boolean
}

defineProps<Props>()

const emit = defineEmits<{
  close: []
}>()

const tagsStore = useTagsStore()
const searchStore = useSearchStore()

// Tag Groups
const tagGroups = computed(() => tagsStore.tagGroups)
const expandedGroupId = ref<number | null>(null)

// File Types
const fileTypes = [
  { label: 'Image', value: 'image' },
  { label: 'Video', value: 'video' },
  { label: 'Document', value: 'document' },
  { label: 'Audio', value: 'audio' },
  { label: 'Archive', value: 'archive' },
]
const selectedFileTypes = ref<string[]>([])

// Attributes
const minSize = ref<number | null>(null)
const maxSize = ref<number | null>(null)
const modifiedAfter = ref<string>('')
const modifiedBefore = ref<string>('')

// Display first 5-10 tags per group
const TAG_DISPLAY_LIMIT = 8

onMounted(async () => {
  if (tagsStore.tagGroups.length === 0) {
    await tagsStore.loadTagGroups()
  }
  if (tagsStore.tags.length === 0) {
    await tagsStore.loadTags()
  }
})

function getDisplayTags(groupId: number): Tag[] {
  const allTags = tagsStore.getTagsByGroup(groupId)
  return allTags.slice(0, TAG_DISPLAY_LIMIT)
}

function hasMoreTags(groupId: number): boolean {
  const allTags = tagsStore.getTagsByGroup(groupId)
  return allTags.length > TAG_DISPLAY_LIMIT
}

function getAllTags(groupId: number): Tag[] {
  return tagsStore.getTagsByGroup(groupId)
}

function getGroupName(groupId: number): string {
  const group = tagGroups.value.find((g) => g.id === groupId)
  return group?.name || ''
}

function expandGroup(groupId: number) {
  expandedGroupId.value = groupId
}

function closeExpandedGroup() {
  expandedGroupId.value = null
}

function isTagSelected(tagId: number): boolean {
  return searchStore.selectedTagIds.includes(tagId)
}

function toggleTag(tagId: number) {
  searchStore.toggleTag(tagId)
}

function clearAllFilters() {
  // Clear tags
  searchStore.clearSelectedTags()
  // Clear file types
  selectedFileTypes.value = []
  // Clear attributes
  minSize.value = null
  maxSize.value = null
  modifiedAfter.value = ''
  modifiedBefore.value = ''
}

function applyFilters() {
  // In a real implementation, this would build a JQL query or execute search
  // For now, just close the panel
  emit('close')
}
</script>

<style scoped>
.advanced-filter-panel {
  width: 100%;
  max-height: 0;
  overflow: hidden;
  background: var(--surface);
  border-bottom: 1px solid var(--border-color);
  transition: max-height 0.3s ease;
}

.advanced-filter-panel.expanded {
  max-height: 500px;
  overflow: visible;
}

.panel-content {
  padding: 1.5rem;
}

.filter-sections {
  display: grid;
  grid-template-columns: 2fr 1fr 1fr;
  gap: 2rem;
  margin-bottom: 1.5rem;
}

.filter-section {
  min-width: 0;
}

.section-title {
  font-size: 13px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0 0 0.75rem 0;
}

.section-content {
  font-size: 12px;
}

/* Tag Groups Section */
.tag-groups-container {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.tag-group-block {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.group-header-inline {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.group-color-dot {
  width: 10px;
  height: 10px;
  border-radius: 2px;
  flex-shrink: 0;
}

.group-name-text {
  font-size: 12px;
  font-weight: 500;
  color: var(--text-secondary);
}

.tags-inline {
  display: flex;
  flex-wrap: wrap;
  gap: 0.375rem;
}

.tag-chip {
  display: inline-flex;
  align-items: center;
  padding: 4px 10px;
  border-radius: 4px;
  font-size: 12px;
  cursor: pointer;
  background: var(--background);
  border: 1px solid var(--border-color);
  transition: all 0.15s;
}

.tag-chip input {
  display: none;
}

.tag-chip:hover {
  border-color: var(--primary-color);
}

.tag-chip.selected {
  background: rgba(25, 118, 210, 0.12);
  border-color: var(--primary-color);
  color: var(--primary-color);
  font-weight: 500;
}

.more-tags-btn {
  display: inline-flex;
  align-items: center;
  padding: 4px 10px;
  border-radius: 4px;
  font-size: 12px;
  cursor: pointer;
  background: transparent;
  border: 1px dashed var(--border-color);
  color: var(--text-secondary);
  transition: all 0.15s;
}

.more-tags-btn:hover {
  border-color: var(--primary-color);
  color: var(--primary-color);
}

.empty-hint {
  font-size: 12px;
  color: var(--text-secondary);
  font-style: italic;
}

/* File Types Section */
.file-type-checkboxes {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.checkbox-item {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 6px 10px;
  border-radius: 4px;
  cursor: pointer;
  transition: background 0.15s;
}

.checkbox-item:hover {
  background: rgba(0, 0, 0, 0.04);
}

.checkbox-item.checked {
  background: rgba(25, 118, 210, 0.08);
  color: var(--primary-color);
  font-weight: 500;
}

.checkbox-item input {
  margin: 0;
  cursor: pointer;
}

/* Attributes Section */
.attribute-group {
  margin-bottom: 1rem;
}

.attribute-group:last-child {
  margin-bottom: 0;
}

.attribute-label {
  display: block;
  font-size: 12px;
  font-weight: 500;
  color: var(--text-secondary);
  margin-bottom: 0.5rem;
}

.size-filter,
.date-filter {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.size-input,
.date-input {
  flex: 1;
  padding: 6px 8px;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  font-size: 12px;
  background: var(--background);
}

.size-input:focus,
.date-input:focus {
  outline: none;
  border-color: var(--primary-color);
}

.separator {
  color: var(--text-secondary);
  font-size: 12px;
}

/* Actions Bar */
.actions-bar {
  display: flex;
  justify-content: flex-end;
  gap: 0.75rem;
  padding-top: 1rem;
  border-top: 1px solid var(--border-color);
}

.btn-clear,
.btn-apply {
  padding: 8px 16px;
  border-radius: 4px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  border: none;
  transition: all 0.15s;
}

.btn-clear {
  background: transparent;
  color: var(--text-secondary);
  border: 1px solid var(--border-color);
}

.btn-clear:hover {
  background: var(--background);
  color: var(--text-primary);
}

.btn-apply {
  background: var(--primary-color);
  color: white;
}

.btn-apply:hover {
  background: var(--primary-dark);
}

/* Expanded Modal */
.expanded-modal-overlay {
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

.expanded-modal {
  background: var(--surface);
  border-radius: 8px;
  width: 90%;
  max-width: 600px;
  max-height: 80vh;
  display: flex;
  flex-direction: column;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.2);
}

.modal-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 1rem 1.5rem;
  border-bottom: 1px solid var(--border-color);
}

.modal-header h3 {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
}

.close-btn {
  background: none;
  border: none;
  font-size: 24px;
  color: var(--text-secondary);
  cursor: pointer;
  padding: 0;
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 4px;
  transition: background 0.15s;
}

.close-btn:hover {
  background: rgba(0, 0, 0, 0.05);
}

.modal-content {
  padding: 1.5rem;
  overflow-y: auto;
}

.tags-grid {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
}

/* Responsive */
@media (max-width: 1024px) {
  .filter-sections {
    grid-template-columns: 1fr;
  }
}
</style>
