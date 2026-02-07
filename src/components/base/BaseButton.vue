<template>
  <button
    :type="type"
    :class="buttonClass"
    :disabled="disabled || loading"
    @click="handleClick"
  >
    <span v-if="loading" class="button-spinner"></span>
    <slot v-else></slot>
  </button>
</template>

<script setup lang="ts">
import { computed } from 'vue'

export type ButtonVariant = 'default' | 'primary' | 'danger' | 'text' | 'icon'
export type ButtonSize = 'small' | 'medium' | 'large'

export interface BaseButtonProps {
  variant?: ButtonVariant
  size?: ButtonSize
  type?: 'button' | 'submit' | 'reset'
  disabled?: boolean
  loading?: boolean
}

const props = withDefaults(defineProps<BaseButtonProps>(), {
  variant: 'default',
  size: 'medium',
  type: 'button',
  disabled: false,
  loading: false,
})

const emit = defineEmits<{
  click: [event: MouseEvent]
}>()

const buttonClass = computed(() => [
  'btn',
  `btn-${props.variant}`,
  `btn-${props.size}`,
  {
    'btn-loading': props.loading,
  },
])

function handleClick(event: MouseEvent) {
  if (!props.disabled && !props.loading) {
    emit('click', event)
  }
}
</script>

<style scoped>
.btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  border: none;
  border-radius: 4px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: var(--transition-fast);
  user-select: none;
  white-space: nowrap;
}

.btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

/* Variants */
.btn-default {
  background: var(--secondary-color);
  color: var(--text-primary);
}

.btn-default:hover:not(:disabled) {
  background: var(--border-color);
}

.btn-primary {
  background: var(--primary-color);
  color: white;
}

.btn-primary:hover:not(:disabled) {
  background: var(--primary-hover);
}

.btn-danger {
  background: #dc3545;
  color: white;
}

.btn-danger:hover:not(:disabled) {
  background: #c82333;
}

.btn-text {
  background: transparent;
  color: var(--text-secondary);
  padding: 4px 8px;
}

.btn-text:hover:not(:disabled) {
  color: var(--text-primary);
  background: var(--secondary-color);
}

.btn-icon {
  padding: 8px;
  background: transparent;
  border: none;
  color: var(--text-secondary);
}

.btn-icon:hover:not(:disabled) {
  background: var(--secondary-color);
  color: var(--text-primary);
}

/* Sizes */
.btn-small {
  padding: 4px 12px;
  font-size: 12px;
}

.btn-medium {
  padding: 8px 16px;
  font-size: 14px;
}

.btn-large {
  padding: 12px 24px;
  font-size: 16px;
}

.btn-icon.btn-small {
  padding: 4px;
}

.btn-icon.btn-medium {
  padding: 8px;
}

.btn-icon.btn-large {
  padding: 12px;
}

/* Loading state */
.btn-loading {
  pointer-events: none;
}

.button-spinner {
  display: inline-block;
  width: 14px;
  height: 14px;
  border: 2px solid currentColor;
  border-right-color: transparent;
  border-radius: 50%;
  animation: spin 0.6s linear infinite;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}
</style>
