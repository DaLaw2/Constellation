<template>
  <div class="search-filter-expanded">
    <!-- Search Bar -->
    <div class="search-bar">
      <!-- Input Mode Toggle -->
      <div class="input-mode-toggle">
        <button
          :class="['input-mode-btn', { active: searchStore.searchInputMode === 'simple' }]"
          @click="searchStore.setSearchInputMode('simple')"
        >Simple</button>
        <button
          :class="['input-mode-btn', { active: searchStore.searchInputMode === 'cql' }]"
          @click="searchStore.setSearchInputMode('cql')"
        >CQL</button>
      </div>

      <!-- Simple Mode: Filename Input -->
      <template v-if="searchStore.searchInputMode === 'simple'">
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
                >&times;</button>
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
      </template>

      <!-- CQL Mode: Query Input -->
      <template v-else>
        <div class="search-input-wrapper">
          <input
            type="text"
            v-model="cqlInput"
            placeholder='tag = "vacation" AND size > 10MB'
            class="search-input cql-input"
            @keyup.enter="executeSearch"
          />
        </div>

        <div class="cql-help-wrapper">
          <button
            class="cql-help-btn"
            @click="showCqlHelp = !showCqlHelp"
            title="CQL Syntax Help"
          >?</button>
          <div v-if="showCqlHelp" class="cql-help-dropdown">
            <div class="cql-help-content">
              <h4>CQL Syntax</h4>
              <div class="cql-help-section">
                <span class="cql-help-label">Fields:</span>
                <code>tag</code> <code>name</code> <code>size</code> <code>modified</code> <code>type</code>
              </div>
              <div class="cql-help-section">
                <span class="cql-help-label">Operators:</span>
                <code>=</code> <code>!=</code> <code>~</code> <code>&gt;</code> <code>&lt;</code> <code>&gt;=</code> <code>&lt;=</code> <code>IN</code>
              </div>
              <div class="cql-help-section">
                <span class="cql-help-label">Logic:</span>
                <code>AND</code> <code>OR</code> <code>NOT</code> <code>( )</code>
              </div>
              <div class="cql-help-section">
                <span class="cql-help-label">Types:</span>
                <code>image</code> <code>video</code> <code>document</code> <code>audio</code> <code>archive</code> <code>directory</code>
              </div>
              <div class="cql-help-examples">
                <span class="cql-help-label">Examples:</span>
                <div class="cql-example" @click="applyCqlExample('tag = &quot;vacation&quot; AND tag = &quot;2024&quot;')">tag = "vacation" AND tag = "2024"</div>
                <div class="cql-example" @click="applyCqlExample('name ~ &quot;*.jpg&quot; OR name ~ &quot;*.png&quot;')">name ~ "*.jpg" OR name ~ "*.png"</div>
                <div class="cql-example" @click="applyCqlExample('size > 10MB AND modified > &quot;2024-01-01&quot;')">size > 10MB AND modified > "2024-01-01"</div>
                <div class="cql-example" @click="applyCqlExample('type = &quot;image&quot; AND size > 5MB')">type = "image" AND size > 5MB</div>
                <div class="cql-example" @click="applyCqlExample('tag IN (&quot;work&quot;, &quot;project&quot;) AND NOT tag = &quot;archived&quot;')">tag IN ("work", "project") AND NOT tag = "archived"</div>
              </div>
            </div>
          </div>
        </div>
      </template>

      <button
        class="btn btn-secondary"
        @click="clearAll"
        :disabled="!canClear"
      >Clear</button>

      <button
        class="btn btn-primary"
        @click="executeSearch"
        :disabled="!canSearch || searchStore.loading"
        :title="searchHint"
      >
        {{ searchStore.loading ? 'Searching...' : 'Search' }}
      </button>
    </div>

    <!-- CQL Error -->
    <div v-if="searchStore.searchInputMode === 'cql' && searchStore.cqlError" class="cql-error">
      {{ searchStore.cqlError }}
    </div>

    <!-- Filter Area (Simple mode only) -->
    <div v-if="searchStore.searchInputMode === 'simple'" class="filter-area">
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
      :client-filter="searchStore.searchInputMode === 'simple' ? clientFilterFn : null"
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
const cqlInput = ref('')
const showHistory = ref(false)
const showCqlHelp = ref(false)
const hasSearched = ref(false)
const clientFilterFn = ref<((items: Item[]) => Item[]) | null>(null)

const tagGroups = computed(() => tagsStore.tagGroups)
const tags = computed(() => tagsStore.tags)
const usageCounts = computed(() => tagsStore.usageCounts)

const canSearch = computed(() => {
  if (searchStore.searchInputMode === 'cql') {
    // CQL must contain tag condition
    const query = cqlInput.value.trim().toLowerCase()
    return query.length > 0 && query.includes('tag')
  }
  // Simple mode must have at least one tag selected
  return searchStore.selectedTagIds.length > 0
})

const searchHint = computed(() => {
  if (canSearch.value) return ''
  if (searchStore.searchInputMode === 'cql') {
    return 'CQL query must include a tag condition'
  }
  return 'Select at least one tag to search'
})

const canClear = computed(() => {
  if (searchStore.searchInputMode === 'cql') {
    return cqlInput.value.length > 0 || searchStore.results.length > 0
  }
  return searchStore.selectedTagIds.length > 0 || filenameInput.value.length > 0 || clientFilterFn.value !== null
})

onMounted(async () => {
  await tagsStore.loadTagGroups()
  await tagsStore.loadTags()
  await tagsStore.loadUsageCounts()
  await appStore.loadSearchHistory()

  // Sync existing store state
  if (searchStore.filenameQuery) {
    filenameInput.value = searchStore.filenameQuery
  }
  if (searchStore.cqlQuery) {
    cqlInput.value = searchStore.cqlQuery
  }
})

watch(filenameInput, (value) => {
  searchStore.setFilenameQuery(value)
})

watch(cqlInput, (value) => {
  searchStore.setCqlQuery(value)
})

async function executeSearch() {
  if (!canSearch.value) return
  hasSearched.value = true

  if (searchStore.searchInputMode === 'cql') {
    await searchStore.executeCqlSearch()
  } else {
    await searchStore.executeSearch()
  }
}

function clearAll() {
  filenameInput.value = ''
  cqlInput.value = ''
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

function applyCqlExample(query: string) {
  cqlInput.value = query
  showCqlHelp.value = false
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

/* Input Mode Toggle (Simple/CQL) */
.input-mode-toggle {
  display: flex;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  overflow: hidden;
  flex-shrink: 0;
}

.input-mode-btn {
  padding: 8px 12px;
  border: none;
  background: var(--surface);
  font-size: 11px;
  font-weight: 600;
  cursor: pointer;
  transition: var(--transition-fast);
  color: var(--text-secondary);
}

.input-mode-btn:first-child {
  border-right: 1px solid var(--border-color);
}

.input-mode-btn.active {
  background: var(--primary-color);
  color: white;
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

.cql-input {
  font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
}

/* Mode Toggle (AND/OR) */
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

/* CQL Help */
.cql-help-wrapper {
  position: relative;
  flex-shrink: 0;
}

.cql-help-btn {
  width: 28px;
  height: 34px;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background: var(--surface);
  font-size: 14px;
  font-weight: 700;
  cursor: pointer;
  color: var(--text-secondary);
  display: flex;
  align-items: center;
  justify-content: center;
  transition: var(--transition-fast);
}

.cql-help-btn:hover {
  border-color: var(--primary-color);
  color: var(--primary-color);
}

.cql-help-dropdown {
  position: absolute;
  top: 100%;
  right: 0;
  margin-top: 6px;
  width: 380px;
  background: var(--surface);
  border: 1px solid var(--border-color);
  border-radius: 6px;
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.15);
  z-index: 200;
  padding: 14px 16px;
}

.cql-help-content h4 {
  margin: 0 0 10px 0;
  font-size: 13px;
  font-weight: 600;
  color: var(--text-primary);
}

.cql-help-section {
  margin-bottom: 8px;
  font-size: 12px;
  color: var(--text-primary);
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 4px;
}

.cql-help-label {
  font-weight: 600;
  color: var(--text-secondary);
  font-size: 11px;
  min-width: 70px;
}

.cql-help-section code {
  background: rgba(0, 0, 0, 0.06);
  padding: 1px 5px;
  border-radius: 3px;
  font-size: 11px;
  font-family: 'Consolas', 'Monaco', monospace;
}

.cql-help-examples {
  margin-top: 10px;
  border-top: 1px solid var(--border-color);
  padding-top: 8px;
}

.cql-help-examples .cql-help-label {
  display: block;
  margin-bottom: 6px;
}

.cql-example {
  padding: 4px 8px;
  font-size: 11px;
  font-family: 'Consolas', 'Monaco', monospace;
  color: var(--text-primary);
  cursor: pointer;
  border-radius: 3px;
  margin-bottom: 2px;
  transition: var(--transition-fast);
}

.cql-example:hover {
  background: rgba(0, 0, 0, 0.06);
  color: var(--primary-color);
}

/* CQL Error */
.cql-error {
  padding: 8px 20px;
  background: #fef2f2;
  border-bottom: 1px solid #fecaca;
  font-size: 12px;
  color: #dc2626;
  font-family: 'Consolas', 'Monaco', monospace;
  flex-shrink: 0;
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
  padding-right: 12px;
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
