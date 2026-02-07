<template>
  <BaseDialog
    :model-value="modelValue"
    title="Create Tag"
    width="500px"
    @update:model-value="emit('update:modelValue', $event)"
    @confirm="handleCreate"
    @cancel="handleCancel"
  >
    <div class="form-group">
      <label>Group:</label>
      <div class="group-display">
        <span
          class="group-badge"
          :style="{ backgroundColor: groupColor }"
        ></span>
        <span>{{ groupName }}</span>
      </div>
    </div>

    <div class="form-group">
      <label>Tag Value:</label>
      <div class="tag-input-container">
        <input
          ref="valueInput"
          v-model="tagValue"
          type="text"
          placeholder="Enter tag value"
          :class="{ 'input-error': isDuplicate }"
          @input="handleInput"
          @keydown.down.prevent="navigateSuggestion(1)"
          @keydown.up.prevent="navigateSuggestion(-1)"
          @keydown.enter.prevent="handleEnterKey"
          @keydown.esc="closeSuggestions"
        />
        <span v-if="isDuplicate" class="error-text">
          Tag already exists in this group
        </span>

        <!-- Search suggestions -->
        <div v-if="suggestions.length > 0" class="suggestions-dropdown">
          <div class="suggestions-header">Existing tags in other groups:</div>
          <div
            v-for="(result, index) in suggestions"
            :key="result.id"
            class="suggestion-item"
            :class="{ selected: index === selectedIndex }"
            @click="selectSuggestion(result)"
          >
            <span class="suggestion-value">{{ result.value }}</span>
            <span class="suggestion-group">{{ getGroupName(result.group_id) }}</span>
          </div>
        </div>
      </div>
    </div>

    <template #footer>
      <BaseButton variant="text" @click="handleCancel">Cancel</BaseButton>
      <BaseButton
        variant="primary"
        :disabled="!tagValue.trim() || isDuplicate"
        @click="handleCreate"
      >
        Create
      </BaseButton>
    </template>
  </BaseDialog>
</template>

<script setup lang="ts">
import { ref, computed, watch, nextTick, onUnmounted } from 'vue'
import { BaseDialog, BaseButton } from '@/components/base'
import type { Tag, TagGroup } from '@/types'

interface CreateTagDialogProps {
  modelValue: boolean
  groupId: number | null
  groups: TagGroup[]
  existingTags: Tag[]
}

const props = defineProps<CreateTagDialogProps>()

const emit = defineEmits<{
  'update:modelValue': [value: boolean]
  create: [groupId: number, value: string]
  search: [query: string]
}>()

const valueInput = ref<HTMLInputElement | null>(null)
const tagValue = ref('')
const suggestions = ref<Tag[]>([])
const selectedIndex = ref(-1)
let searchDebounceTimer: ReturnType<typeof setTimeout> | null = null

const currentGroup = computed(() => {
  if (!props.groupId) return null
  return props.groups.find(g => g.id === props.groupId)
})

const groupName = computed(() => currentGroup.value?.name || '')
const groupColor = computed(() => currentGroup.value?.color || '#9e9e9e')

const isDuplicate = computed(() => {
  if (!tagValue.value.trim() || !props.groupId) return false
  const normalized = tagValue.value.trim().toLowerCase()
  return props.existingTags.some(
    t => t.group_id === props.groupId && t.value.toLowerCase() === normalized
  )
})

watch(
  () => props.modelValue,
  async (newValue) => {
    if (newValue) {
      tagValue.value = ''
      suggestions.value = []
      selectedIndex.value = -1
      await nextTick()
      valueInput.value?.focus()
    }
  }
)

function handleInput() {
  if (searchDebounceTimer) {
    clearTimeout(searchDebounceTimer)
  }
  searchDebounceTimer = setTimeout(() => {
    if (tagValue.value.trim()) {
      emit('search', tagValue.value.trim())
    } else {
      suggestions.value = []
      selectedIndex.value = -1
    }
  }, 300)
}

function updateSuggestions(results: Tag[]) {
  suggestions.value = results
  selectedIndex.value = -1
}

function getGroupName(groupId: number): string {
  const group = props.groups.find(g => g.id === groupId)
  return group ? group.name : ''
}

function navigateSuggestion(direction: number) {
  if (suggestions.value.length === 0) return
  const newIndex = selectedIndex.value + direction
  if (newIndex >= -1 && newIndex < suggestions.value.length) {
    selectedIndex.value = newIndex
  }
}

function selectSuggestion(tag: Tag) {
  tagValue.value = tag.value
  closeSuggestions()
}

function handleEnterKey() {
  if (selectedIndex.value >= 0 && selectedIndex.value < suggestions.value.length) {
    selectSuggestion(suggestions.value[selectedIndex.value])
  } else {
    handleCreate()
  }
}

function closeSuggestions() {
  suggestions.value = []
  selectedIndex.value = -1
}

function handleCreate() {
  if (!props.groupId || !tagValue.value.trim() || isDuplicate.value) return
  emit('create', props.groupId, tagValue.value.trim())
  emit('update:modelValue', false)
}

function handleCancel() {
  emit('update:modelValue', false)
}

onUnmounted(() => {
  if (searchDebounceTimer) {
    clearTimeout(searchDebounceTimer)
  }
})

defineExpose({
  updateSuggestions,
})
</script>

<style scoped>
.form-group {
  margin-bottom: 20px;
}

.form-group:last-child {
  margin-bottom: 0;
}

label {
  display: block;
  margin-bottom: 8px;
  font-size: 14px;
  font-weight: 500;
  color: var(--text-primary);
}

.group-display {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 12px;
  background: var(--surface);
  border-radius: 4px;
}

.group-badge {
  width: 16px;
  height: 16px;
  border-radius: 3px;
}

.tag-input-container {
  position: relative;
}

input[type='text'] {
  width: 100%;
  padding: 8px 12px;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  font-size: 14px;
  transition: var(--transition-fast);
}

input[type='text']:focus {
  outline: none;
  border-color: var(--primary-color);
}

input.input-error {
  border-color: #dc3545;
}

.error-text {
  display: block;
  margin-top: 4px;
  font-size: 12px;
  color: #dc3545;
}

.suggestions-dropdown {
  position: absolute;
  top: 100%;
  left: 0;
  right: 0;
  margin-top: 4px;
  background: var(--background);
  border: 1px solid var(--border-color);
  border-radius: 4px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  max-height: 200px;
  overflow-y: auto;
  z-index: 1000;
}

.suggestions-header {
  padding: 8px 12px;
  font-size: 12px;
  font-weight: 500;
  color: var(--text-secondary);
  background: var(--surface);
  border-bottom: 1px solid var(--border-color);
}

.suggestion-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 8px 12px;
  cursor: pointer;
  transition: var(--transition-fast);
}

.suggestion-item:hover,
.suggestion-item.selected {
  background: var(--secondary-color);
}

.suggestion-value {
  font-size: 14px;
  color: var(--text-primary);
}

.suggestion-group {
  font-size: 12px;
  color: var(--text-secondary);
  padding: 2px 8px;
  background: var(--surface);
  border-radius: 3px;
}
</style>
