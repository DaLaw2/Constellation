<template>
  <BaseDialog
    :model-value="modelValue"
    :title="title"
    :width="width"
    :close-on-click-outside="closeOnClickOutside"
    @update:model-value="emit('update:modelValue', $event)"
    @close="handleClose"
  >
    <template #default>
      <div class="alert-content">
        <div v-if="icon" class="alert-icon" :class="`alert-icon-${type}`">
          {{ icon }}
        </div>
        <div class="alert-message">
          <p class="alert-message-text">{{ message }}</p>
          <p v-if="description" class="alert-description">{{ description }}</p>
        </div>
      </div>
    </template>

    <template #footer>
      <BaseButton variant="primary" @click="handleClose">
        {{ dismissText }}
      </BaseButton>
    </template>
  </BaseDialog>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import BaseDialog from './BaseDialog.vue'
import BaseButton from './BaseButton.vue'

export type AlertDialogType = 'info' | 'warning' | 'error' | 'success'

export interface AlertDialogProps {
  modelValue: boolean
  title?: string
  message: string
  description?: string
  type?: AlertDialogType
  dismissText?: string
  width?: string | number
  closeOnClickOutside?: boolean
}

const props = withDefaults(defineProps<AlertDialogProps>(), {
  title: 'Alert',
  description: '',
  type: 'info',
  dismissText: 'OK',
  width: '450px',
  closeOnClickOutside: true,
})

const emit = defineEmits<{
  'update:modelValue': [value: boolean]
  dismiss: []
}>()

const icon = computed(() => {
  switch (props.type) {
    case 'info':
      return 'ℹ️'
    case 'warning':
      return '⚠️'
    case 'error':
      return '❌'
    case 'success':
      return '✓'
    default:
      return 'ℹ️'
  }
})

function handleClose() {
  emit('update:modelValue', false)
  emit('dismiss')
}
</script>

<style scoped>
.alert-content {
  display: flex;
  gap: 16px;
  align-items: flex-start;
}

.alert-icon {
  font-size: 32px;
  width: 48px;
  height: 48px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 8px;
  flex-shrink: 0;
}

.alert-icon-info {
  background: #e3f2fd;
}

.alert-icon-warning {
  background: #fff3e0;
}

.alert-icon-error {
  background: #ffebee;
}

.alert-icon-success {
  background: #e8f5e9;
  color: #4caf50;
  font-size: 36px;
  font-weight: bold;
}

.alert-message {
  flex: 1;
}

.alert-message-text {
  font-size: 15px;
  font-weight: 500;
  color: var(--text-primary);
  margin: 0 0 8px 0;
  line-height: 1.5;
}

.alert-description {
  font-size: 14px;
  color: var(--text-secondary);
  margin: 0;
  line-height: 1.5;
}
</style>
