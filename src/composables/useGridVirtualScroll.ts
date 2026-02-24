/**
 * Grid virtual scrolling composable.
 *
 * Groups items into rows for use with RecycleScroller.
 * Computes column count from container width via ResizeObserver.
 * Reacts to changes in minCardWidth/gap (e.g., zoom level).
 */

import { ref, computed, watch, onMounted, onUnmounted, toValue, type Ref, type MaybeRefOrGetter } from 'vue'

export interface GridRow<T> {
  /** Unique row key for RecycleScroller */
  id: string
  /** Items in this row (1 to columnCount) */
  items: T[]
  /** Index of first item in the original array */
  startIndex: number
}

export interface UseGridVirtualScrollOptions {
  /** Minimum card width in pixels (can be reactive for zoom support) */
  minCardWidth: MaybeRefOrGetter<number>
  /** Gap between cards in pixels (can be reactive for zoom support) */
  gap: MaybeRefOrGetter<number>
  /** Container element ref for ResizeObserver */
  containerRef: Ref<HTMLElement | null>
}

export function useGridVirtualScroll<T>(
  items: Ref<T[]>,
  options: UseGridVirtualScrollOptions,
) {
  const columnCount = ref(4)
  let containerWidth = 0
  let resizeObserver: ResizeObserver | null = null

  function recalculate() {
    if (containerWidth <= 0) return
    const minWidth = toValue(options.minCardWidth)
    const gapVal = toValue(options.gap)
    const cols = Math.max(1, Math.floor((containerWidth + gapVal) / (minWidth + gapVal)))
    columnCount.value = cols
  }

  onMounted(() => {
    const el = options.containerRef.value
    if (el) {
      resizeObserver = new ResizeObserver((entries) => {
        for (const entry of entries) {
          containerWidth = entry.contentRect.width
          recalculate()
        }
      })
      resizeObserver.observe(el)
      containerWidth = el.clientWidth
      recalculate()
    }
  })

  onUnmounted(() => {
    resizeObserver?.disconnect()
  })

  // Recalculate when minCardWidth or gap changes (e.g., zoom)
  watch(
    () => [toValue(options.minCardWidth), toValue(options.gap)],
    () => recalculate(),
  )

  const rows = computed<GridRow<T>[]>(() => {
    const cols = columnCount.value
    const allItems = items.value
    const result: GridRow<T>[] = []

    for (let i = 0; i < allItems.length; i += cols) {
      result.push({
        id: `row-${i}`,
        items: allItems.slice(i, i + cols),
        startIndex: i,
      })
    }
    return result
  })

  return {
    rows,
    columnCount,
  }
}
