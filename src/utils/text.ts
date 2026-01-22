/**
 * Text measurement utility functions.
 */

// Cached canvas context for text measurement
let measureContext: CanvasRenderingContext2D | null = null

function getMeasureContext(): CanvasRenderingContext2D | null {
  if (!measureContext) {
    const canvas = document.createElement('canvas')
    measureContext = canvas.getContext('2d')
    if (measureContext) {
      measureContext.font = '500 11px sans-serif'
    }
  }
  return measureContext
}

/**
 * Calculates the exact pixel width of a tag badge.
 * @param text - The tag text
 * @param maxWidth - Maximum width constraint (default: 120)
 * @returns The calculated width in pixels
 */
export function getTagTextWidth(text: string, maxWidth = 120): number {
  const context = getMeasureContext()
  if (!context) {
    // Fallback calculation
    return Math.min(maxWidth, text.length * 7 + 18)
  }

  const textMetrics = context.measureText(text)
  // padding-left: 10px + padding-right: 8px = 18px
  const width = Math.ceil(textMetrics.width) + 18

  return Math.min(maxWidth, width)
}
