import { onMounted, onUnmounted } from 'vue'

type EventTarget = Window | Document | HTMLElement

/**
 * Adds an event listener with automatic cleanup on unmount.
 * @param target - The event target (window, document, or element)
 * @param event - The event name
 * @param handler - The event handler
 * @param options - Event listener options
 */
export function useEventListener<K extends keyof WindowEventMap>(
  target: Window,
  event: K,
  handler: (event: WindowEventMap[K]) => void,
  options?: boolean | AddEventListenerOptions
): void

export function useEventListener<K extends keyof DocumentEventMap>(
  target: Document,
  event: K,
  handler: (event: DocumentEventMap[K]) => void,
  options?: boolean | AddEventListenerOptions
): void

export function useEventListener<K extends keyof HTMLElementEventMap>(
  target: HTMLElement,
  event: K,
  handler: (event: HTMLElementEventMap[K]) => void,
  options?: boolean | AddEventListenerOptions
): void

export function useEventListener(
  target: EventTarget,
  event: string,
  handler: (event: Event) => void,
  options?: boolean | AddEventListenerOptions
) {
  onMounted(() => {
    target.addEventListener(event, handler, options)
  })

  onUnmounted(() => {
    target.removeEventListener(event, handler, options)
  })
}
