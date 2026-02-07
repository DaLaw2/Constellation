<template>
  <BaseDialog
    :model-value="modelValue"
    @update:model-value="emit('update:modelValue', $event)"
    title="Move Tags to Group"
    width="400px"
    :confirm-disabled="!selectedGroupId"
    confirm-text="Move"
    @confirm="handleConfirm"
    @cancel="emit('update:modelValue', false)"
  >
    <div class="move-dialog-content">
      <p class="move-description">
        Select a target group for {{ tagIds.length }} tag{{ tagIds.length === 1 ? '' : 's' }}:
      </p>

      <div class="group-list">
        <div
          v-for="group in availableGroups"
          :key="group.id"
          class="group-option"
          :class="{ selected: selectedGroupId === group.id }"
          @click="selectedGroupId = group.id"
        >
          <span
            class="group-color"
            :style="{ backgroundColor: group.color || '#9e9e9e' }"
          ></span>
          <span class="group-name">{{ group.name }}</span>
        </div>
      </div>

      <div v-if="availableGroups.length === 0" class="no-groups">
        No other groups available
      </div>
    </div>
  </BaseDialog>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { BaseDialog } from '@/components/base'
import type { TagGroup } from '@/types'

interface Props {
  modelValue: boolean
  tagIds: number[]
  groups: TagGroup[]
  currentGroupId: number | null
}

const props = defineProps<Props>()

const emit = defineEmits<{
  'update:modelValue': [value: boolean]
  confirm: [targetGroupId: number]
}>()

const selectedGroupId = ref<number | null>(null)

const availableGroups = computed(() => {
  return props.groups.filter(g => g.id !== props.currentGroupId)
})

watch(() => props.modelValue, (visible) => {
  if (visible) {
    selectedGroupId.value = null
  }
})

function handleConfirm() {
  if (selectedGroupId.value) {
    emit('confirm', selectedGroupId.value)
    emit('update:modelValue', false)
  }
}
</script>

<style scoped>
.move-dialog-content {
  padding: 8px 0;
}

.move-description {
  margin: 0 0 16px 0;
  font-size: 14px;
  color: var(--text-secondary);
}

.group-list {
  display: flex;
  flex-direction: column;
  gap: 4px;
  max-height: 300px;
  overflow-y: auto;
}

.group-option {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 10px 12px;
  border-radius: 6px;
  cursor: pointer;
  transition: var(--transition-fast);
  border: 2px solid transparent;
}

.group-option:hover {
  background: rgba(0, 0, 0, 0.04);
}

.group-option.selected {
  background: rgba(25, 118, 210, 0.08);
  border-color: var(--primary-color);
}

.group-color {
  width: 16px;
  height: 16px;
  border-radius: 3px;
  flex-shrink: 0;
}

.group-name {
  font-size: 14px;
  font-weight: 500;
  color: var(--text-primary);
}

.no-groups {
  padding: 24px;
  text-align: center;
  color: var(--text-secondary);
  font-size: 13px;
}
</style>
