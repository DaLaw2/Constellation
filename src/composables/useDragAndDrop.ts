import { ref, type Ref } from 'vue'

export interface DragState<T = unknown> {
  isDragging: boolean
  draggedItem: T | null
  dragOverItem: T | null
}

export interface UseDragAndDropOptions<T> {
  /** Callback when drag starts */
  onDragStart?: (item: T) => void
  /** Callback when drag ends */
  onDragEnd?: (item: T) => void
  /** Callback when items are reordered */
  onReorder?: (items: T[]) => void | Promise<void>
  /** Callback when drag enters an item */
  onDragEnter?: (item: T) => void
  /** Callback when drag leaves an item */
  onDragLeave?: (item: T) => void
}

/**
 * Composable for managing drag and drop state.
 * Works with vuedraggable or native drag events.
 *
 * @param options - Drag and drop callbacks
 * @returns Drag state and handler functions
 */
export function useDragAndDrop<T = unknown>(options: UseDragAndDropOptions<T> = {}) {
  const { onDragStart, onDragEnd, onReorder, onDragEnter, onDragLeave } = options

  const state: Ref<DragState<T>> = ref({
    isDragging: false,
    draggedItem: null,
    dragOverItem: null,
  })

  /**
   * Handle drag start event
   */
  function handleDragStart(item: T) {
    state.value.isDragging = true
    state.value.draggedItem = item

    if (onDragStart) {
      onDragStart(item)
    }
  }

  /**
   * Handle drag end event
   */
  function handleDragEnd() {
    const draggedItem = state.value.draggedItem

    state.value.isDragging = false
    state.value.draggedItem = null
    state.value.dragOverItem = null

    if (draggedItem && onDragEnd) {
      onDragEnd(draggedItem)
    }
  }

  /**
   * Handle drag enter event
   */
  function handleDragEnter(item: T) {
    state.value.dragOverItem = item

    if (onDragEnter) {
      onDragEnter(item)
    }
  }

  /**
   * Handle drag leave event
   */
  function handleDragLeave(item: T) {
    if (state.value.dragOverItem === item) {
      state.value.dragOverItem = null
    }

    if (onDragLeave) {
      onDragLeave(item)
    }
  }

  /**
   * Handle reorder event (for use with vuedraggable)
   * @param items - The new ordered array of items
   */
  async function handleReorder(items: T[]) {
    if (onReorder) {
      await onReorder(items)
    }
  }

  /**
   * Reset drag state
   */
  function reset() {
    state.value = {
      isDragging: false,
      draggedItem: null,
      dragOverItem: null,
    }
  }

  return {
    state,
    handleDragStart,
    handleDragEnd,
    handleDragEnter,
    handleDragLeave,
    handleReorder,
    reset,
  }
}

/**
 * Helper for vuedraggable integration
 * Extracts IDs from reordered items for backend sync
 */
export function extractIds<T extends { id: number | string }>(items: T[]): (number | string)[] {
  return items.map((item) => item.id)
}
