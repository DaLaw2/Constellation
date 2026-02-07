<template>
  <BaseDialog
    :model-value="modelValue"
    title="Edit Tag Group"
    width="500px"
    @update:model-value="emit('update:modelValue', $event)"
    @confirm="handleSave"
    @cancel="handleCancel"
  >
    <div class="form-group">
      <label>Name:</label>
      <input
        ref="nameInput"
        v-model="groupName"
        type="text"
        placeholder="Group Name"
        :class="{ 'input-error': isDuplicate }"
        @keyup.enter="handleSave"
      />
      <span v-if="isDuplicate" class="error-text">Group already exists</span>
    </div>

    <div class="form-group">
      <label>Color:</label>
      <ColorPicker v-model="groupColor" :presets="presetColors" />
    </div>

    <template #footer>
      <BaseButton variant="danger" @click="handleDelete">
        Delete Group
      </BaseButton>
      <div style="flex: 1"></div>
      <BaseButton variant="text" @click="handleCancel">Cancel</BaseButton>
      <BaseButton
        variant="primary"
        :disabled="!groupName.trim() || isDuplicate"
        @click="handleSave"
      >
        Save
      </BaseButton>
    </template>
  </BaseDialog>
</template>

<script setup lang="ts">
import { ref, computed, watch, nextTick } from 'vue'
import { BaseDialog, BaseButton, ColorPicker } from '@/components/base'
import type { TagGroup } from '@/types'

interface EditGroupDialogProps {
  modelValue: boolean
  group: TagGroup | null
  existingGroups: TagGroup[]
}

const props = defineProps<EditGroupDialogProps>()

const emit = defineEmits<{
  'update:modelValue': [value: boolean]
  save: [id: number, name: string, color: string]
  delete: [id: number]
}>()

const nameInput = ref<HTMLInputElement | null>(null)
const groupName = ref('')
const groupColor = ref('#3B82F6')

const presetColors = [
  '#ef5350', '#ec407a', '#ab47bc', '#7e57c2',
  '#5c6bc0', '#42a5f5', '#29b6f6', '#26c6da',
  '#26a69a', '#66bb6a', '#9ccc65', '#d4e157',
  '#ffee58', '#ffca28', '#ffa726', '#ff7043',
]

const isDuplicate = computed(() => {
  if (!props.group || !groupName.value.trim()) return false
  const normalized = groupName.value.trim().toLowerCase()
  return props.existingGroups.some(
    g => g.id !== props.group!.id && g.name.toLowerCase() === normalized
  )
})

watch(
  () => props.modelValue,
  async (newValue) => {
    if (newValue && props.group) {
      groupName.value = props.group.name
      groupColor.value = props.group.color || '#3B82F5'
      await nextTick()
      nameInput.value?.focus()
    }
  }
)

function handleSave() {
  if (!props.group || !groupName.value.trim() || isDuplicate.value) return
  emit('save', props.group.id, groupName.value.trim(), groupColor.value)
  emit('update:modelValue', false)
}

function handleDelete() {
  if (!props.group) return
  emit('delete', props.group.id)
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
