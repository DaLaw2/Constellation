<template>
  <Teleport to="body">
    <Transition name="context-menu-fade">
      <div
        v-if="visible"
        ref="menuRef"
        class="context-menu"
        :style="menuStyle"
        @contextmenu.prevent
      >
        <div
          v-for="(item, index) in items"
          :key="index"
          class="context-menu-item"
          :class="{
            'context-menu-item-danger': item.danger,
            'context-menu-item-disabled': item.disabled,
            'context-menu-divider': item.divider,
          }"
          @click="handleItemClick(item)"
        >
          <template v-if="!item.divider">
            <span v-if="item.icon" class="context-menu-icon">{{ item.icon }}</span>
            <span class="context-menu-label">{{ item.label }}</span>
            <span v-if="item.shortcut" class="context-menu-shortcut">
              {{ item.shortcut }}
            </span>
          </template>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
import { ref, computed, watch, nextTick, onMounted, onUnmounted } from 'vue'

export interface ContextMenuItem {
  label?: string
  icon?: string
  shortcut?: string
  danger?: boolean
  disabled?: boolean
  divider?: boolean
  action?: () => void
}

export interface ContextMenuProps {
  visible: boolean
  x: number
  y: number
  items: ContextMenuItem[]
}

const props = defineProps<ContextMenuProps>()

const emit = defineEmits<{
  'update:visible': [value: boolean]
  select: [item: ContextMenuItem]
}>()

const menuRef = ref<HTMLDivElement | null>(null)

const menuStyle = computed(() => {
  if (!menuRef.value) {
    return {
      left: `${props.x}px`,
      top: `${props.y}px`,
    }
  }

  const menuRect = menuRef.value.getBoundingClientRect()
  const viewportWidth = window.innerWidth
  const viewportHeight = window.innerHeight

  let left = props.x
  let top = props.y

  // Adjust if menu would overflow viewport horizontally
  if (left + menuRect.width > viewportWidth) {
    left = viewportWidth - menuRect.width - 8
  }

  // Adjust if menu would overflow viewport vertically
  if (top + menuRect.height > viewportHeight) {
    top = viewportHeight - menuRect.height - 8
  }

  // Ensure menu is not off-screen on the left or top
  left = Math.max(8, left)
  top = Math.max(8, top)

  return {
    left: `${left}px`,
    top: `${top}px`,
  }
})

function handleItemClick(item: ContextMenuItem) {
  if (item.disabled || item.divider) {
    return
  }

  emit('select', item)

  if (item.action) {
    item.action()
  }

  closeMenu()
}

function closeMenu() {
  emit('update:visible', false)
}

function handleClickOutside(event: MouseEvent) {
  if (menuRef.value && !menuRef.value.contains(event.target as Node)) {
    closeMenu()
  }
}

function handleEscape(event: KeyboardEvent) {
  if (event.key === 'Escape' && props.visible) {
    closeMenu()
  }
}

watch(
  () => props.visible,
  async (newVisible) => {
    if (newVisible) {
      await nextTick()
      document.addEventListener('click', handleClickOutside)
      document.addEventListener('keydown', handleEscape)
    } else {
      document.removeEventListener('click', handleClickOutside)
      document.removeEventListener('keydown', handleEscape)
    }
  }
)

onMounted(() => {
  if (props.visible) {
    document.addEventListener('click', handleClickOutside)
    document.addEventListener('keydown', handleEscape)
  }
})

onUnmounted(() => {
  document.removeEventListener('click', handleClickOutside)
  document.removeEventListener('keydown', handleEscape)
})
</script>

<style scoped>
.context-menu {
  position: fixed;
  background: var(--background);
  border: 1px solid var(--border-color);
  border-radius: 6px;
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.15);
  padding: 4px;
  min-width: 180px;
  z-index: 2000;
  user-select: none;
}

.context-menu-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 8px 12px;
  border-radius: 4px;
  cursor: pointer;
  transition: var(--transition-fast);
  font-size: 14px;
  color: var(--text-primary);
}

.context-menu-item:hover:not(.context-menu-item-disabled):not(.context-menu-divider) {
  background: var(--secondary-color);
}

.context-menu-item-danger {
  color: #dc3545;
}

.context-menu-item-danger:hover:not(.context-menu-item-disabled) {
  background: #ffebee;
}

.context-menu-item-disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.context-menu-divider {
  height: 1px;
  background: var(--border-color);
  margin: 4px 0;
  padding: 0;
  cursor: default;
}

.context-menu-icon {
  font-size: 16px;
  width: 20px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.context-menu-label {
  flex: 1;
}

.context-menu-shortcut {
  font-size: 12px;
  color: var(--text-secondary);
  font-family: 'Courier New', monospace;
}

/* Transition */
.context-menu-fade-enter-active,
.context-menu-fade-leave-active {
  transition: opacity 0.15s ease, transform 0.15s ease;
}

.context-menu-fade-enter-from {
  opacity: 0;
  transform: scale(0.95);
}

.context-menu-fade-leave-to {
  opacity: 0;
  transform: scale(0.95);
}
</style>
