<template>
  <BaseDialog
    :model-value="modelValue"
    @update:model-value="emit('update:modelValue', $event)"
    title="Merge Tag"
    width="450px"
    :confirm-disabled="!selectedTargetId"
    confirm-text="Merge"
    @confirm="handleConfirm"
    @cancel="emit('update:modelValue', false)"
  >
    <div class="merge-dialog-content">
      <p class="merge-description">
        Merge "<strong>{{ sourceTag?.value }}</strong>" into another tag.
        All file associations will be transferred to the target tag.
      </p>

      <div class="search-box">
        <input
          ref="searchInput"
          type="text"
          v-model="searchQuery"
          placeholder="Search tags..."
          class="search-input"
        />
      </div>

      <div class="tag-list">
        <div v-for="group in filteredGroups" :key="group.id" class="tag-group-section">
          <div class="group-header">
            <span
              class="group-color"
              :style="{ backgroundColor: group.color || '#9e9e9e' }"
            ></span>
            <span class="group-name">{{ group.name }}</span>
          </div>
          <div class="group-tags">
            <div
              v-for="tag in getFilteredTags(group.id)"
              :key="tag.id"
              class="tag-option"
              :class="{ selected: selectedTargetId === tag.id }"
              @click="selectedTargetId = tag.id"
            >
              <span class="tag-value">{{ tag.value }}</span>
              <span class="tag-usage">{{ usageCounts[tag.id] || 0 }} files</span>
            </div>
          </div>
        </div>
      </div>

      <div v-if="filteredGroups.length === 0" class="no-results">
        No matching tags found
      </div>
    </div>
  </BaseDialog>
</template>

<script setup lang="ts">
import { ref, computed, watch, nextTick } from 'vue'
import { BaseDialog } from '@/components/base'
import type { Tag, TagGroup } from '@/types'

interface Props {
  modelValue: boolean
  sourceTag: Tag | null
  tags: Tag[]
  groups: TagGroup[]
  usageCounts: Record<number, number>
}

const props = defineProps<Props>()

const emit = defineEmits<{
  'update:modelValue': [value: boolean]
  confirm: [targetId: number]
}>()

const searchQuery = ref('')
const selectedTargetId = ref<number | null>(null)
const searchInput = ref<HTMLInputElement | null>(null)

const availableTags = computed(() => {
  return props.tags.filter(t => t.id !== props.sourceTag?.id)
})

const filteredGroups = computed(() => {
  return props.groups.filter(group => {
    const tags = getFilteredTags(group.id)
    return tags.length > 0
  })
})

function getFilteredTags(groupId: number): Tag[] {
  const query = searchQuery.value.trim().toLowerCase()
  let tags = availableTags.value.filter(t => t.group_id === groupId)

  if (query) {
    tags = tags.filter(t => t.value.toLowerCase().includes(query))
  }

  return tags
}

watch(() => props.modelValue, (visible) => {
  if (visible) {
    searchQuery.value = ''
    selectedTargetId.value = null
    nextTick(() => {
      searchInput.value?.focus()
    })
  }
})

function handleConfirm() {
  if (selectedTargetId.value) {
    emit('confirm', selectedTargetId.value)
    emit('update:modelValue', false)
  }
}
</script>

<style scoped>
.merge-dialog-content {
  padding: 8px 0;
}

.merge-description {
  margin: 0 0 16px 0;
  font-size: 14px;
  color: var(--text-secondary);
  line-height: 1.5;
}

.search-box {
  margin-bottom: 12px;
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

.tag-list {
  max-height: 300px;
  overflow-y: auto;
}

.tag-group-section {
  margin-bottom: 12px;
}

.group-header {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 4px 0;
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
  letter-spacing: 0.5px;
}

.group-tags {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.tag-option {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 8px 12px;
  border-radius: 4px;
  cursor: pointer;
  transition: var(--transition-fast);
  border: 2px solid transparent;
}

.tag-option:hover {
  background: rgba(0, 0, 0, 0.04);
}

.tag-option.selected {
  background: rgba(25, 118, 210, 0.08);
  border-color: var(--primary-color);
}

.tag-value {
  font-size: 14px;
  font-weight: 500;
  color: var(--text-primary);
}

.tag-usage {
  font-size: 12px;
  color: var(--text-secondary);
}

.no-results {
  padding: 24px;
  text-align: center;
  color: var(--text-secondary);
  font-size: 13px;
}
</style>
