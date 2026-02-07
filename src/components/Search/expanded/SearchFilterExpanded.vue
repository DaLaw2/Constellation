<template>
  <div class="search-filter-expanded">
    <!-- Search Bar -->
    <div class="search-bar">
      <div class="search-input-wrapper">
        <input
          ref="searchInput"
          type="text"
          v-model="filenameInput"
          placeholder="Search by filename..."
          class="search-input"
          @focus="showHistory = true"
          @blur="handleBlur"
          @keyup.enter="executeSearch"
        />
        <!-- History Dropdown -->
        <div v-if="showHistory && appStore.searchHistory.length > 0" class="history-dropdown">
          <div class="history-header">
            <span>Recent Searches</span>
            <button class="clear-history-btn" @click.stop="clearHistory">Clear All</button>
          </div>
          <div class="history-list">
            <div
              v-for="item in appStore.searchHistory"
              :key="item.id"
              class="history-item"
              @mousedown.prevent="applyHistory(item)"
            >
              <div class="history-content">
                <span v-if="item.criteria.filename_query" class="history-query">"{{ item.criteria.filename_query }}"</span>
                <span v-if="item.criteria.tag_ids.length > 0" class="history-tags">
                  + {{ item.criteria.tag_ids.length }} tags
                </span>
                <span v-if="!item.criteria.filename_query && item.criteria.tag_ids.length === 0" class="history-empty">
                  (Empty Search)
                </span>
              </div>
              <button
                class="delete-history-btn"
                @mousedown.prevent.stop="deleteHistory(item.id)"
                title="Remove"
              >×</button>
            </div>
          </div>
        </div>
      </div>

      <div class="mode-toggle">
        <button
          :class="['mode-btn', { active: searchStore.mode === 'and' }]"
          @click="searchStore.setMode('and')"
        >AND</button>
        <button
          :class="['mode-btn', { active: searchStore.mode === 'or' }]"
          @click="searchStore.setMode('or')"
        >OR</button>
      </div>

      <button
        class="btn btn-secondary"
        @click="clearAll"
        :disabled="!hasAnyCriteria && !hasClientFilter"
      >Clear</button>

      <button
        class="btn btn-primary"
        @click="executeSearch"
        :disabled="!hasAnyCriteria || searchStore.loading"
      >
        {{ searchStore.loading ? 'Searching...' : 'Search' }}
      </button>
    </div>

    <!-- Filter Area -->
    <div class="filter-area">
      <div class="filter-column filter-tags">
        <TagGroupFilter
          :tag-groups="tagGroups"
          :tags="tags"
          :selected-tag-ids="searchStore.selectedTagIds"
          :usage-counts="usageCounts"
          @toggle-tag="searchStore.toggleTag($event)"
        />
      </div>

      <div class="filter-divider"></div>

      <div class="filter-column filter-options">
        <FilterOptionsPanel
          @update:filter="clientFilterFn = $event"
        />
      </div>
    </div>

    <!-- Results -->
    <SearchResultsPanel
      :results="searchStore.results"
      :loading="searchStore.loading"
      :error="searchStore.error"
      :has-searched="hasSearched"
      :client-filter="clientFilterFn"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue'
import { useSearchStore } from '@/stores/search'
import { useTagsStore } from '@/stores/tags'
import { useAppStore } from '@/stores/app'
import TagGroupFilter from './TagGroupFilter.vue'
import FilterOptionsPanel from './FilterOptionsPanel.vue'
import SearchResultsPanel from './SearchResultsPanel.vue'
import type { Item, SearchHistory } from '@/types'

const searchStore = useSearchStore()
const tagsStore = useTagsStore()
const appStore = useAppStore()

const filenameInput = ref('')
const showHistory = ref(false)
const hasSearched = ref(false)
const clientFilterFn = ref<((items: Item[]) => Item[]) | null>(null)

const tagGroups = computed(() => tagsStore.tagGroups)
const tags = computed(() => tagsStore.tags)
const usageCounts = computed(() => tagsStore.usageCounts)

const hasAnyCriteria = computed(() => {
  return searchStore.selectedTagIds.length > 0 || filenameInput.value.trim().length > 0
})

const hasClientFilter = computed(() => clientFilterFn.value !== null)

onMounted(async () => {
  await tagsStore.loadTagGroups()
  await tagsStore.loadTags()
  await tagsStore.loadUsageCounts()
  await appStore.loadSearchHistory()

  // Sync existing store state
  if (searchStore.filenameQuery) {
    filenameInput.value = searchStore.filenameQuery
  }
})

watch(filenameInput, (value) => {
  searchStore.setFilenameQuery(value)
})

async function executeSearch() {
  if (!hasAnyCriteria.value) return
  hasSearched.value = true
  await searchStore.executeSearch()
}

function clearAll() {
  filenameInput.value = ''
  searchStore.clearSearch()
  clientFilterFn.value = null
  hasSearched.value = false
}

function handleBlur() {
  setTimeout(() => {
    showHistory.value = false
  }, 200)
}

function applyHistory(item: SearchHistory) {
  if (item.criteria.filename_query) {
    filenameInput.value = item.criteria.filename_query
  } else {
    filenameInput.value = ''
  }

  searchStore.setMode(item.criteria.mode)
  searchStore.clearSelectedTags()
  item.criteria.tag_ids.forEach(tagId => {
    searchStore.selectTag(tagId)
  })

  showHistory.value = false
  executeSearch()
}

async function deleteHistory(id: number) {
  await appStore.deleteSearchHistory(id)
}

async function clearHistory() {
  await appStore.clearSearchHistory()
}
</script>

<style scoped>
.search-filter-expanded {
  height: 100%;
  display: flex;
  flex-direction: column;
  background: var(--surface);
}

/* Search Bar */
.search-bar {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 12px 20px;
  border-bottom: 1px solid var(--border-color);
  background: var(--background);
  flex-shrink: 0;
}

.search-input-wrapper {
  flex: 1;
  position: relative;
}

.search-input {
  width: 100%;
  padding: 8px 12px;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  font-size: 13px;
  background: var(--surface);
}

.search-input:focus {
  outline: none;
  border-color: var(--primary-color);
}

/* Mode Toggle */
.mode-toggle {
  display: flex;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  overflow: hidden;
  flex-shrink: 0;
}

.mode-btn {
  padding: 8px 14px;
  border: none;
  background: var(--surface);
  font-size: 12px;
  font-weight: 600;
  cursor: pointer;
  transition: var(--transition-fast);
  color: var(--text-secondary);
}

.mode-btn:first-child {
  border-right: 1px solid var(--border-color);
}

.mode-btn.active {
  background: var(--primary-color);
  color: white;
}

/* Buttons */
.btn {
  padding: 8px 16px;
  border: none;
  border-radius: 4px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: var(--transition-fast);
  flex-shrink: 0;
}

.btn-primary {
  background: var(--primary-color);
  color: white;
}

.btn-primary:hover:not(:disabled) {
  background: var(--primary-hover);
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
}

.btn-secondary:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

/* Filter Area */
.filter-area {
  display: flex;
  padding: 16px 20px 24px;
  gap: 0;
  border-bottom: 1px solid var(--border-color);
  flex-shrink: 0;
  min-height: 180px;
  max-height: 35vh;
}

.filter-column {
  padding: 0 16px;
  overflow-y: auto;
}

.filter-tags {
  flex: 2;
  padding-left: 0;
}

.filter-options {
  flex: 1;
  padding-right: 0;
}

.filter-divider {
  width: 1px;
  background: var(--border-color);
  flex-shrink: 0;
  align-self: stretch;
}

/* History Dropdown */
.history-dropdown {
  position: absolute;
  top: 100%;
  left: 0;
  right: 0;
  background: var(--surface);
  border: 1px solid var(--border-color);
  border-radius: 4px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  z-index: 100;
  margin-top: 4px;
  max-height: 300px;
  display: flex;
  flex-direction: column;
}

.history-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px 12px;
  border-bottom: 1px solid var(--border-color);
  font-size: 11px;
  color: var(--text-secondary);
  font-weight: 500;
  background: var(--background);
}

.clear-history-btn {
  background: none;
  border: none;
  font-size: 11px;
  color: var(--primary-color);
  cursor: pointer;
}

.clear-history-btn:hover {
  text-decoration: underline;
}

.history-list {
  overflow-y: auto;
  max-height: 250px;
}

.history-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 8px 12px;
  cursor: pointer;
  border-bottom: 1px solid var(--border-color);
}

.history-item:last-child {
  border-bottom: none;
}

.history-item:hover {
  background: rgba(0, 0, 0, 0.04);
}

.history-content {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 13px;
  color: var(--text-primary);
}

.history-query {
  font-weight: 500;
}

.history-tags {
  font-size: 11px;
  background: rgba(0, 0, 0, 0.05);
  padding: 2px 6px;
  border-radius: 4px;
  color: var(--text-secondary);
}

.history-empty {
  font-style: italic;
  color: var(--text-secondary);
}

.delete-history-btn {
  background: none;
  border: none;
  font-size: 18px;
  color: var(--text-secondary);
  cursor: pointer;
  padding: 0 4px;
  line-height: 1;
  opacity: 0.5;
}

.delete-history-btn:hover {
  opacity: 1;
  color: #ef4444;
}
</style>
