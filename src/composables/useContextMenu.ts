import { ref, type Ref } from 'vue'
import type { ContextMenuItem } from '@/components/base/ContextMenu.vue'

export interface ContextMenuState {
  visible: boolean
  x: number
  y: number
  items: ContextMenuItem[]
  target: unknown | null
}

/**
 * Composable for managing context menu state and behavior.
 * Provides methods to show/hide context menu and track the target.
 *
 * @returns Context menu state and control methods
 */
export function useContextMenu<T = unknown>() {
  const state: Ref<ContextMenuState> = ref({
    visible: false,
    x: 0,
    y: 0,
    items: [],
    target: null,
  })

  /**
   * Show context menu at specified position
   * @param event - Mouse event from contextmenu listener
   * @param items - Menu items to display
   * @param target - Optional target object (e.g., the file, tag, or group)
   */
  function show(event: MouseEvent, items: ContextMenuItem[], target?: T) {
    event.preventDefault()
    event.stopPropagation()

    state.value = {
      visible: true,
      x: event.clientX,
      y: event.clientY,
      items,
      target: target ?? null,
    }
  }

  /**
   * Hide the context menu
   */
  function hide() {
    state.value.visible = false
  }

  /**
   * Get the current target
   */
  function getTarget(): T | null {
    return state.value.target as T | null
  }

  /**
   * Update menu items dynamically
   */
  function updateItems(items: ContextMenuItem[]) {
    state.value.items = items
  }

  return {
    state,
    show,
    hide,
    getTarget,
    updateItems,
  }
}
