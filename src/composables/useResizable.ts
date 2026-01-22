import { ref, onUnmounted } from 'vue'

export interface UseResizableOptions {
  /** Minimum width allowed */
  minWidth?: number
  /** Maximum width allowed */
  maxWidth?: number
  /** Callback when resize ends */
  onResizeEnd?: (width: number) => void
}

/**
 * Provides drag-to-resize functionality.
 * @param initialWidth - The initial width
 * @param options - Resize options
 * @returns Reactive width and resize handlers
 */
export function useResizable(initialWidth: number, options: UseResizableOptions = {}) {
  const { minWidth = 0, maxWidth = Infinity, onResizeEnd } = options

  const width = ref(initialWidth)
  const isResizing = ref(false)
  const startX = ref(0)
  const startWidth = ref(0)

  function handleResizeStart(e: MouseEvent) {
    isResizing.value = true
    startX.value = e.clientX
    startWidth.value = width.value

    document.addEventListener('mousemove', handleResize)
    document.addEventListener('mouseup', handleResizeEnd)
  }

  function handleResize(e: MouseEvent) {
    if (!isResizing.value) return

    const delta = startX.value - e.clientX
    const newWidth = Math.min(maxWidth, Math.max(minWidth, startWidth.value + delta))
    width.value = newWidth
  }

  function handleResizeEnd() {
    if (isResizing.value) {
      isResizing.value = false
      document.removeEventListener('mousemove', handleResize)
      document.removeEventListener('mouseup', handleResizeEnd)

      if (onResizeEnd) {
        onResizeEnd(width.value)
      }
    }
  }

  onUnmounted(() => {
    document.removeEventListener('mousemove', handleResize)
    document.removeEventListener('mouseup', handleResizeEnd)
  })

  return {
    width,
    isResizing,
    handleResizeStart,
  }
}
