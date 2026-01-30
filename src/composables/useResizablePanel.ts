import { ref, onUnmounted } from 'vue'

export interface UseResizablePanelOptions {
  /** Minimum width in pixels */
  minWidth?: number
  /** Maximum width in pixels */
  maxWidth?: number
  /** Initial width in pixels */
  initialWidth?: number
  /** Callback when resize ends */
  onResizeEnd?: (width: number) => void
  /** Callback during resize */
  onResize?: (width: number) => void
}

/**
 * Provides drag-to-resize functionality for panels (e.g., sidebars).
 * Handles cursor styles and user-select during resize.
 *
 * @param options - Resize options
 * @returns Reactive width, resizing state, and handlers
 */
export function useResizablePanel(options: UseResizablePanelOptions = {}) {
  const {
    minWidth = 150,
    maxWidth = 600,
    initialWidth = 280,
    onResizeEnd,
    onResize,
  } = options

  const width = ref(initialWidth)
  const isResizing = ref(false)

  function startResize(event: MouseEvent) {
    isResizing.value = true

    // Set cursor styles
    document.body.style.cursor = 'col-resize'
    document.body.style.userSelect = 'none'

    document.addEventListener('mousemove', handleResize)
    document.addEventListener('mouseup', stopResize)

    event.preventDefault()
  }

  function handleResize(event: MouseEvent) {
    if (!isResizing.value) return

    // Calculate new width based on mouse X position
    const newWidth = Math.min(Math.max(event.clientX, minWidth), maxWidth)
    width.value = newWidth

    if (onResize) {
      onResize(newWidth)
    }
  }

  function stopResize() {
    if (!isResizing.value) return

    isResizing.value = false

    // Restore cursor styles
    document.body.style.cursor = ''
    document.body.style.userSelect = ''

    document.removeEventListener('mousemove', handleResize)
    document.removeEventListener('mouseup', stopResize)

    if (onResizeEnd) {
      onResizeEnd(width.value)
    }
  }

  onUnmounted(() => {
    // Clean up event listeners
    document.removeEventListener('mousemove', handleResize)
    document.removeEventListener('mouseup', stopResize)

    // Restore cursor styles
    document.body.style.cursor = ''
    document.body.style.userSelect = ''
  })

  return {
    width,
    isResizing,
    startResize,
  }
}
