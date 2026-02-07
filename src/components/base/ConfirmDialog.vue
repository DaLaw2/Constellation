<template>
  <BaseDialog
    :model-value="modelValue"
    :title="title"
    :width="width"
    :close-on-click-outside="closeOnClickOutside"
    @update:model-value="emit('update:modelValue', $event)"
    @confirm="handleConfirm"
    @cancel="handleCancel"
  >
    <template #default>
      <div class="confirm-content">
        <div v-if="icon" class="confirm-icon" :class="`confirm-icon-${type}`">
          {{ icon }}
        </div>
        <div class="confirm-message">
          <p class="confirm-message-text">{{ message }}</p>
          <p v-if="description" class="confirm-description">{{ description }}</p>
        </div>
      </div>
    </template>

    <template #footer>
      <BaseButton variant="text" @click="handleCancel">
        {{ cancelText }}
      </BaseButton>
      <BaseButton
        :variant="confirmVariant"
        @click="handleConfirm"
        :loading="loading"
      >
        {{ confirmText }}
      </BaseButton>
    </template>
  </BaseDialog>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import BaseDialog from './BaseDialog.vue'
import BaseButton from './BaseButton.vue'
import type { ButtonVariant } from './BaseButton.vue'

export type ConfirmDialogType = 'info' | 'warning' | 'danger' | 'success'

export interface ConfirmDialogProps {
  modelValue: boolean
  title?: string
  message: string
  description?: string
  type?: ConfirmDialogType
  confirmText?: string
  cancelText?: string
  width?: string | number
  closeOnClickOutside?: boolean
  loading?: boolean
}

const props = withDefaults(defineProps<ConfirmDialogProps>(), {
  title: 'Confirm',
  description: '',
  type: 'warning',
  confirmText: 'Confirm',
  cancelText: 'Cancel',
  width: '450px',
  closeOnClickOutside: true,
  loading: false,
})

const emit = defineEmits<{
  'update:modelValue': [value: boolean]
  confirm: []
  cancel: []
}>()

const icon = computed(() => {
  switch (props.type) {
    case 'info':
      return 'ℹ️'
    case 'warning':
      return '⚠️'
    case 'danger':
      return '🗑️'
    case 'success':
      return '✓'
    default:
      return '❓'
  }
})

const confirmVariant = computed((): ButtonVariant => {
  switch (props.type) {
    case 'danger':
      return 'danger'
    case 'success':
      return 'primary'
    default:
      return 'primary'
  }
})

function handleConfirm() {
  emit('confirm')
}

function handleCancel() {
  emit('cancel')
  emit('update:modelValue', false)
}
</script>

<style scoped>
.confirm-content {
  display: flex;
  gap: 16px;
  align-items: flex-start;
}

.confirm-icon {
  font-size: 32px;
  width: 48px;
  height: 48px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 8px;
  flex-shrink: 0;
}

.confirm-icon-info {
  background: #e3f2fd;
}

.confirm-icon-warning {
  background: #fff3e0;
}

.confirm-icon-danger {
  background: #ffebee;
}

.confirm-icon-success {
  background: #e8f5e9;
  color: #4caf50;
  font-size: 36px;
  font-weight: bold;
}

.confirm-message {
  flex: 1;
}

.confirm-message-text {
  font-size: 15px;
  font-weight: 500;
  color: var(--text-primary);
  margin: 0 0 8px 0;
  line-height: 1.5;
}

.confirm-description {
  font-size: 14px;
  color: var(--text-secondary);
  margin: 0;
  line-height: 1.5;
}
</style>
