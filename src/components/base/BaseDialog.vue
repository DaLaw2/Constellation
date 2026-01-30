<template>
  <Teleport to="body">
    <Transition name="dialog-fade">
      <div
        v-if="modelValue"
        class="dialog-overlay"
        @click.self="handleOverlayClick"
      >
        <Transition name="dialog-slide">
          <div
            v-if="modelValue"
            class="dialog"
            :class="dialogClass"
            :style="dialogStyle"
          >
            <!-- Header -->
            <div class="dialog-header">
              <h3>{{ title }}</h3>
              <button
                v-if="showClose"
                class="btn-icon dialog-close"
                @click="handleClose"
                aria-label="Close dialog"
              >
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  width="20"
                  height="20"
                  viewBox="0 0 24 24"
                  fill="none"
                  stroke="currentColor"
                  stroke-width="2"
                  stroke-linecap="round"
                  stroke-linejoin="round"
                >
                  <line x1="18" y1="6" x2="6" y2="18" />
                  <line x1="6" y1="6" x2="18" y2="18" />
                </svg>
              </button>
            </div>

            <!-- Content -->
            <div class="dialog-content">
              <slot></slot>
            </div>

            <!-- Footer -->
            <div v-if="$slots.footer || showDefaultFooter" class="dialog-footer">
              <slot name="footer">
                <button class="btn" @click="handleCancel">
                  {{ cancelText }}
                </button>
                <button
                  class="btn btn-primary"
                  @click="handleConfirm"
                  :disabled="confirmDisabled"
                >
                  {{ confirmText }}
                </button>
              </slot>
            </div>
          </div>
        </Transition>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
import { computed } from 'vue'

export interface BaseDialogProps {
  modelValue: boolean
  title?: string
  width?: string | number
  showClose?: boolean
  closeOnClickOutside?: boolean
  showDefaultFooter?: boolean
  confirmText?: string
  cancelText?: string
  confirmDisabled?: boolean
}

const props = withDefaults(defineProps<BaseDialogProps>(), {
  title: '',
  width: '500px',
  showClose: true,
  closeOnClickOutside: true,
  showDefaultFooter: false,
  confirmText: 'Confirm',
  cancelText: 'Cancel',
  confirmDisabled: false,
})

const emit = defineEmits<{
  'update:modelValue': [value: boolean]
  confirm: []
  cancel: []
  close: []
}>()

const dialogClass = computed(() => ({
  // Add custom classes here if needed
}))

const dialogStyle = computed(() => ({
  width: typeof props.width === 'number' ? `${props.width}px` : props.width,
}))

function handleClose() {
  emit('update:modelValue', false)
  emit('close')
}

function handleOverlayClick() {
  if (props.closeOnClickOutside) {
    handleClose()
  }
}

function handleConfirm() {
  emit('confirm')
}

function handleCancel() {
  emit('cancel')
  handleClose()
}
</script>

<style scoped>
.dialog-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  backdrop-filter: blur(2px);
}

.dialog {
  background: var(--background);
  border-radius: 8px;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.15);
  max-width: 90vw;
  max-height: 90vh;
  display: flex;
  flex-direction: column;
}

.dialog-header {
  padding: 20px 24px;
  border-bottom: 1px solid var(--border-color);
  display: flex;
  align-items: center;
  justify-content: space-between;
  flex-shrink: 0;
}

.dialog-header h3 {
  margin: 0;
  font-size: 18px;
  font-weight: 600;
  color: var(--text-primary);
}

.dialog-close {
  margin-left: auto;
  color: var(--text-secondary);
}

.dialog-close:hover {
  color: var(--text-primary);
}

.dialog-content {
  padding: 24px;
  overflow-y: auto;
  flex: 1;
}

.dialog-footer {
  padding: 16px 24px;
  border-top: 1px solid var(--border-color);
  display: flex;
  align-items: center;
  justify-content: flex-end;
  gap: 12px;
  flex-shrink: 0;
}

/* Transitions */
.dialog-fade-enter-active,
.dialog-fade-leave-active {
  transition: opacity 0.2s ease;
}

.dialog-fade-enter-from,
.dialog-fade-leave-to {
  opacity: 0;
}

.dialog-slide-enter-active,
.dialog-slide-leave-active {
  transition: all 0.3s ease;
}

.dialog-slide-enter-from {
  transform: translateY(-20px);
  opacity: 0;
}

.dialog-slide-leave-to {
  transform: translateY(20px);
  opacity: 0;
}
</style>
