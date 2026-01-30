<template>
  <BaseDialog
    :model-value="modelValue"
    title="Edit Tag"
    width="500px"
    @update:model-value="emit('update:modelValue', $event)"
    @confirm="handleSave"
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
      <input
        ref="valueInput"
        v-model="tagValue"
        type="text"
        placeholder="Tag Name"
        :class="{ 'input-error': isDuplicate }"
        @keyup.enter="handleSave"
      />
      <span v-if="isDuplicate" class="error-text">
        Tag already exists in this group
      </span>
    </div>

    <template #footer>
      <BaseButton variant="danger" @click="handleDelete">
        Delete Tag
      </BaseButton>
      <div style="flex: 1"></div>
      <BaseButton variant="text" @click="handleCancel">Cancel</BaseButton>
      <BaseButton
        variant="primary"
        :disabled="!tagValue.trim() || isDuplicate"
        @click="handleSave"
      >
        Save
      </BaseButton>
    </template>
  </BaseDialog>
</template>

<script setup lang="ts">
import { ref, computed, watch, nextTick } from 'vue'
import { BaseDialog, BaseButton } from '@/components/base'
import type { Tag, TagGroup } from '@/types'

interface EditTagDialogProps {
  modelValue: boolean
  tag: Tag | null
  groups: TagGroup[]
  existingTags: Tag[]
}

const props = defineProps<EditTagDialogProps>()

const emit = defineEmits<{
  'update:modelValue': [value: boolean]
  save: [id: number, value: string]
  delete: [id: number]
}>()

const valueInput = ref<HTMLInputElement | null>(null)
const tagValue = ref('')

const currentGroup = computed(() => {
  if (!props.tag) return null
  return props.groups.find(g => g.id === props.tag!.group_id)
})

const groupName = computed(() => currentGroup.value?.name || '')
const groupColor = computed(() => currentGroup.value?.color || '#9e9e9e')

const isDuplicate = computed(() => {
  if (!props.tag || !tagValue.value.trim()) return false
  const normalized = tagValue.value.trim().toLowerCase()
  return props.existingTags.some(
    t =>
      t.id !== props.tag!.id &&
      t.group_id === props.tag!.group_id &&
      t.value.toLowerCase() === normalized
  )
})

watch(
  () => props.modelValue,
  async (newValue) => {
    if (newValue && props.tag) {
      tagValue.value = props.tag.value
      await nextTick()
      valueInput.value?.focus()
    }
  }
)

function handleSave() {
  if (!props.tag || !tagValue.value.trim() || isDuplicate.value) return
  emit('save', props.tag.id, tagValue.value.trim())
  emit('update:modelValue', false)
}

function handleDelete() {
  if (!props.tag) return
  emit('delete', props.tag.id)
}

function handleCancel() {
  emit('update:modelValue', false)
}
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
</style>
