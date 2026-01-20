<template>
  <div class="filter-panel">
    <div class="panel-header">
      <h3>Search & Filter</h3>
      <button class="btn btn-secondary" @click="clearAll" :disabled="!hasAnyCriteria">
        Clear
      </button>
    </div>

    <!-- Filename Search -->
    <div class="filter-section">
      <label class="section-label">Filename</label>
      <input
        type="text"
        v-model="filenameInput"
        placeholder="Search by filename..."
        class="search-input"
        @keyup.enter="executeSearch"
      />
    </div>

    <!-- Search Mode Toggle -->
    <div class="filter-section">
      <label class="section-label">Tag Match Mode</label>
      <div class="mode-toggle">
        <button
          :class="['mode-btn', { active: searchStore.mode === 'and' }]"
          @click="searchStore.setMode('and')"
        >
          AND
        </button>
        <button
          :class="['mode-btn', { active: searchStore.mode === 'or' }]"
          @click="searchStore.setMode('or')"
        >
          OR
        </button>
      </div>
      <div class="mode-hint">
        {{ searchStore.mode === 'and' ? 'Items must have ALL selected tags' : 'Items must have ANY selected tag' }}
      </div>
    </div>

    <!-- Tag Selection -->
    <div class="filter-section tags-section">
      <label class="section-label">
        Tags
        <span v-if="searchStore.selectedTagIds.length > 0" class="selected-count">
          ({{ searchStore.selectedTagIds.length }} selected)
        </span>
      </label>

      <div v-if="tagsLoading" class="loading-state">Loading tags...</div>

      <div v-else-if="tagGroups.length === 0" class="empty-state">
        No tags available. Create tags in Tag Management first.
      </div>

      <div v-else class="tag-groups-list">
        <div v-for="group in tagGroups" :key="group.id" class="tag-group">
          <div class="group-header">
            <span
              class="group-color"
              :style="{ backgroundColor: group.color || '#9e9e9e' }"
            ></span>
            <span class="group-name">{{ group.name }}</span>
          </div>
          <div class="tag-checkboxes">
            <label
              v-for="tag in getTagsByGroup(group.id)"
              :key="tag.id"
              class="tag-checkbox"
              :class="{ checked: isTagSelected(tag.id) }"
            >
              <input
                type="checkbox"
                :checked="isTagSelected(tag.id)"
                @change="searchStore.toggleTag(tag.id)"
              />
              <span class="tag-label">{{ tag.value }}</span>
            </label>
          </div>
        </div>
      </div>
    </div>

    <!-- Search Button -->
    <div class="search-actions">
      <button
        class="btn btn-primary search-btn"
        @click="executeSearch"
        :disabled="!hasAnyCriteria || searchStore.loading"
      >
        {{ searchStore.loading ? 'Searching...' : 'Search' }}
      </button>
    </div>

    <!-- Results Summary -->
    <div v-if="searchStore.results.length > 0 || hasSearched" class="results-summary">
      <div class="results-count">
        {{ searchStore.resultCount }} {{ searchStore.resultCount === 1 ? 'item' : 'items' }} found
      </div>
    </div>

    <!-- Results List -->
    <div v-if="searchStore.results.length > 0" class="results-list">
      <div
        v-for="item in searchStore.results"
        :key="item.id"
        class="result-item"
        @click="openItem(item)"
      >
        <span class="result-icon">{{ item.is_directory ? '📁' : '📄' }}</span>
        <span class="result-name" :title="item.path">{{ getFileName(item.path) }}</span>
      </div>
    </div>

    <!-- Error State -->
    <div v-if="searchStore.error" class="error-state">
      Error: {{ searchStore.error }}
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue'
import { useSearchStore, type Item } from '../../stores/search'
import { useTagsStore } from '../../stores/tags'
import { useAppStore } from '../../stores/app'

const searchStore = useSearchStore()
const tagsStore = useTagsStore()
const appStore = useAppStore()

const filenameInput = ref('')
const hasSearched = ref(false)
const tagsLoading = ref(false)

const tagGroups = computed(() => tagsStore.tagGroups)

const hasAnyCriteria = computed(() => {
  return searchStore.selectedTagIds.length > 0 || filenameInput.value.trim().length > 0
})

onMounted(async () => {
  tagsLoading.value = true
  await tagsStore.loadTagGroups()
  await tagsStore.loadTags()
  tagsLoading.value = false
})

// Sync filename input with store
watch(filenameInput, (value) => {
  searchStore.setFilenameQuery(value)
})

function getTagsByGroup(groupId: number) {
  return tagsStore.getTagsByGroup(groupId)
}

function isTagSelected(tagId: number): boolean {
  return searchStore.selectedTagIds.includes(tagId)
}

async function executeSearch() {
  if (!hasAnyCriteria.value) return
  hasSearched.value = true
  await searchStore.executeSearch()
}

function clearAll() {
  filenameInput.value = ''
  searchStore.clearSearch()
  hasSearched.value = false
}

function getFileName(path: string): string {
  const parts = path.replace(/\\/g, '/').split('/')
  return parts[parts.length - 1] || path
}

function openItem(item: Item) {
  // Navigate to the item's directory in file browser
  const dirPath = item.is_directory ? item.path : getParentPath(item.path)
  appStore.setCurrentPath(dirPath)
  appStore.setLeftPanelMode('file-browser')
}

function getParentPath(path: string): string {
  const normalized = path.replace(/\\/g, '/')
  const lastSlash = normalized.lastIndexOf('/')
  return lastSlash > 0 ? normalized.substring(0, lastSlash) : normalized
}
</script>

<style scoped>
.filter-panel {
  height: 100%;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.panel-header {
  padding: 1rem;
  border-bottom: 1px solid var(--border-color);
  display: flex;
  align-items: center;
  justify-content: space-between;
  flex-shrink: 0;
}

.panel-header h3 {
  font-size: 14px;
  font-weight: 600;
  margin: 0;
}

.filter-section {
  padding: 0.75rem 1rem;
  border-bottom: 1px solid var(--border-color);
}

.section-label {
  display: block;
  font-size: 12px;
  font-weight: 500;
  color: var(--text-secondary);
  margin-bottom: 0.5rem;
}

.selected-count {
  color: var(--primary-color);
  font-weight: 600;
}

.search-input {
  width: 100%;
  padding: 8px 12px;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  font-size: 13px;
  background: var(--background);
}

.search-input:focus {
  outline: none;
  border-color: var(--primary-color);
}

.mode-toggle {
  display: flex;
  gap: 0;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  overflow: hidden;
}

.mode-btn {
  flex: 1;
  padding: 6px 12px;
  border: none;
  background: var(--background);
  font-size: 12px;
  font-weight: 500;
  cursor: pointer;
  transition: var(--transition-fast);
  color: var(--text-secondary);
}

.mode-btn:first-child {
  border-right: 1px solid var(--border-color);
}

.mode-btn:hover {
  background: var(--surface);
}

.mode-btn.active {
  background: var(--primary-color);
  color: white;
}

.mode-hint {
  margin-top: 0.5rem;
  font-size: 11px;
  color: var(--text-secondary);
  font-style: italic;
}

.tags-section {
  flex: 1;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.tag-groups-list {
  flex: 1;
  overflow-y: auto;
}

.tag-group {
  margin-bottom: 0.75rem;
}

.group-header {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  margin-bottom: 0.5rem;
}

.group-color {
  width: 10px;
  height: 10px;
  border-radius: 2px;
}

.group-name {
  font-size: 12px;
  font-weight: 500;
  color: var(--text-primary);
}

.tag-checkboxes {
  display: flex;
  flex-wrap: wrap;
  gap: 0.25rem;
}

.tag-checkbox {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 4px 8px;
  border-radius: 4px;
  font-size: 12px;
  cursor: pointer;
  background: var(--background);
  border: 1px solid var(--border-color);
  transition: var(--transition-fast);
}

.tag-checkbox:hover {
  border-color: var(--primary-color);
}

.tag-checkbox.checked {
  background: rgba(25, 118, 210, 0.1);
  border-color: var(--primary-color);
  color: var(--primary-color);
}

.tag-checkbox input {
  display: none;
}

.tag-label {
  white-space: nowrap;
}

.search-actions {
  padding: 0.75rem 1rem;
  border-top: 1px solid var(--border-color);
  flex-shrink: 0;
}

.search-btn {
  width: 100%;
  padding: 10px;
  font-size: 13px;
}

.results-summary {
  padding: 0.5rem 1rem;
  background: var(--surface);
  border-bottom: 1px solid var(--border-color);
  flex-shrink: 0;
}

.results-count {
  font-size: 12px;
  color: var(--text-secondary);
}

.results-list {
  flex: 1;
  overflow-y: auto;
  padding: 0.5rem;
}

.result-item {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.5rem;
  border-radius: 4px;
  cursor: pointer;
  transition: var(--transition-fast);
}

.result-item:hover {
  background: rgba(0, 0, 0, 0.04);
}

.result-icon {
  flex-shrink: 0;
}

.result-name {
  font-size: 12px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.loading-state,
.empty-state,
.error-state {
  padding: 1rem;
  text-align: center;
  font-size: 12px;
  color: var(--text-secondary);
}

.error-state {
  color: #d32f2f;
}

.btn {
  padding: 6px 12px;
  border: none;
  border-radius: 4px;
  font-size: 12px;
  cursor: pointer;
  transition: var(--transition-fast);
}

.btn-primary {
  background: var(--primary-color);
  color: white;
}

.btn-primary:hover:not(:disabled) {
  background: var(--primary-dark);
}

.btn-primary:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.btn-secondary {
  background: var(--surface);
  color: var(--text-secondary);
  border: 1px solid var(--border-color);
}

.btn-secondary:hover:not(:disabled) {
  background: var(--background);
  color: var(--text-primary);
}

.btn-secondary:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}
</style>
