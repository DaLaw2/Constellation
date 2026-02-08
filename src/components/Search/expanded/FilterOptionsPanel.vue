<template>
  <div class="filter-options">
    <!-- File Types -->
    <div class="filter-section">
      <h4 class="section-title">File Types</h4>
      <div class="checkbox-list">
        <label
          v-for="ft in fileTypes"
          :key="ft.key"
          class="checkbox-item"
          :class="{ checked: selectedFileTypes.has(ft.key) }"
        >
          <input
            type="checkbox"
            :checked="selectedFileTypes.has(ft.key)"
            @change="toggleFileType(ft.key)"
          />
          <span class="checkbox-label">{{ ft.label }}</span>
        </label>
      </div>
      <p class="filter-hint">Directories excluded when active</p>
    </div>

    <!-- Size Filter -->
    <div class="filter-section">
      <h4 class="section-title">Size</h4>
      <div class="range-inputs">
        <input
          type="number"
          v-model.number="sizeMin"
          placeholder="Min (MB)"
          class="range-input"
          min="0"
        />
        <span class="range-separator">-</span>
        <input
          type="number"
          v-model.number="sizeMax"
          placeholder="Max (MB)"
          class="range-input"
          min="0"
        />
      </div>
      <p class="filter-hint">Directories excluded when active</p>
    </div>

    <!-- Date Filter -->
    <div class="filter-section">
      <h4 class="section-title">Modified After</h4>
      <div class="date-inputs">
        <input
          type="number"
          v-model.number="dateYear"
          placeholder="YYYY"
          class="date-part date-year"
          min="2000"
          max="2099"
        />
        <span class="date-separator">/</span>
        <input
          type="number"
          v-model.number="dateMonth"
          placeholder="MM"
          class="date-part date-month"
          min="1"
          max="12"
        />
        <span class="date-separator">/</span>
        <input
          type="number"
          v-model.number="dateDay"
          placeholder="DD"
          class="date-part date-day"
          min="1"
          max="31"
        />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import type { Item } from '@/types'

const FILE_TYPE_EXTENSIONS: Record<string, string[]> = {
  image: ['.jpg', '.jpeg', '.png', '.gif', '.bmp', '.webp', '.svg', '.ico', '.tiff', '.tif'],
  video: ['.mp4', '.avi', '.mkv', '.mov', '.wmv', '.flv', '.webm', '.m4v'],
  document: ['.pdf', '.doc', '.docx', '.xls', '.xlsx', '.ppt', '.pptx', '.txt', '.csv', '.rtf'],
  audio: ['.mp3', '.wav', '.flac', '.aac', '.ogg', '.wma', '.m4a'],
  archive: ['.zip', '.rar', '.7z', '.tar', '.gz', '.bz2', '.xz'],
}

const fileTypes = [
  { key: 'image', label: 'Images' },
  { key: 'video', label: 'Videos' },
  { key: 'document', label: 'Documents' },
  { key: 'audio', label: 'Audio' },
  { key: 'archive', label: 'Archives' },
]

const selectedFileTypes = ref<Set<string>>(new Set())
const sizeMin = ref<number | null>(null)
const sizeMax = ref<number | null>(null)
const dateYear = ref<number | null>(null)
const dateMonth = ref<number | null>(null)
const dateDay = ref<number | null>(null)

const modifiedAfter = computed(() => {
  if (dateYear.value && dateMonth.value && dateDay.value) {
    const y = dateYear.value
    const m = String(dateMonth.value).padStart(2, '0')
    const d = String(dateDay.value).padStart(2, '0')
    return `${y}-${m}-${d}`
  }
  return ''
})

const emit = defineEmits<{
  'update:filter': [filterFn: ((items: Item[]) => Item[]) | null]
}>()

function toggleFileType(key: string) {
  if (selectedFileTypes.value.has(key)) {
    selectedFileTypes.value.delete(key)
  } else {
    selectedFileTypes.value.add(key)
  }
  selectedFileTypes.value = new Set(selectedFileTypes.value)
  emitFilter()
}

function getExtension(path: string): string {
  const dot = path.lastIndexOf('.')
  if (dot === -1) return ''
  return path.slice(dot).toLowerCase()
}

function emitFilter() {
  const types = new Set(selectedFileTypes.value)
  const minBytes = sizeMin.value ? sizeMin.value * 1024 * 1024 : null
  const maxBytes = sizeMax.value ? sizeMax.value * 1024 * 1024 : null
  const afterDate = modifiedAfter.value ? new Date(modifiedAfter.value).getTime() / 1000 : null

  const hasAnyFilter = types.size > 0 || minBytes !== null || maxBytes !== null || afterDate !== null

  if (!hasAnyFilter) {
    emit('update:filter', null)
    return
  }

  const filterFn = (items: Item[]): Item[] => {
    return items.filter(item => {
      // Exclude directories when File Types or Size filter is active
      if (item.is_directory && (types.size > 0 || minBytes !== null || maxBytes !== null)) {
        return false
      }

      // File type filter
      if (types.size > 0) {
        const ext = getExtension(item.path)
        const matchesType = [...types].some(type => {
          const extensions = FILE_TYPE_EXTENSIONS[type]
          return extensions && extensions.includes(ext)
        })
        if (!matchesType) return false
      }

      // Size filter
      if (minBytes !== null || maxBytes !== null) {
        if (minBytes !== null && (item.size === null || item.size === undefined || item.size < minBytes)) {
          return false
        }
        if (maxBytes !== null && (item.size === null || item.size === undefined || item.size > maxBytes)) {
          return false
        }
      }

      // Date filter
      if (afterDate !== null && (item.modified_time === null || item.modified_time === undefined || item.modified_time < afterDate)) {
        return false
      }

      return true
    })
  }

  emit('update:filter', filterFn)
}

watch([sizeMin, sizeMax, dateYear, dateMonth, dateDay], () => {
  emitFilter()
})
</script>

<style scoped>
.filter-options {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.filter-section {
  /* Individual filter section */
}

.section-title {
  margin: 0 0 8px 0;
  font-size: 13px;
  font-weight: 600;
  color: var(--text-primary);
}

.checkbox-list {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.checkbox-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 4px 8px;
  border-radius: 4px;
  font-size: 13px;
  cursor: pointer;
  transition: var(--transition-fast);
}

.checkbox-item:hover {
  background: rgba(0, 0, 0, 0.04);
}

.checkbox-item.checked {
  color: var(--primary-color);
}

.checkbox-item input[type="checkbox"] {
  cursor: pointer;
}

.checkbox-label {
  color: var(--text-primary);
}

.filter-hint {
  margin: 6px 0 0 0;
  font-size: 11px;
  color: var(--text-secondary);
  opacity: 0.7;
}

.range-inputs {
  display: flex;
  align-items: center;
  gap: 8px;
}

.range-input {
  flex: 1;
  padding: 6px 10px;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  font-size: 12px;
  background: var(--background);
  width: 80px;
}

.range-input:focus {
  outline: none;
  border-color: var(--primary-color);
}

.range-separator {
  font-size: 12px;
  color: var(--text-secondary);
}

.date-inputs {
  display: flex;
  align-items: center;
  gap: 4px;
}

.date-part {
  padding: 6px 8px;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  font-size: 12px;
  background: var(--background);
  text-align: center;
  -moz-appearance: textfield;
}

.date-part::-webkit-inner-spin-button,
.date-part::-webkit-outer-spin-button {
  -webkit-appearance: none;
  margin: 0;
}

.date-year {
  width: 60px;
}

.date-month,
.date-day {
  width: 44px;
}

.date-part:focus {
  outline: none;
  border-color: var(--primary-color);
}

.date-separator {
  font-size: 12px;
  color: var(--text-secondary);
}
</style>
