import { onMounted, onUnmounted, type Ref } from 'vue'

/**
 * Calls a handler when a click occurs outside the target element.
 * @param target - Ref to the target element
 * @param handler - Function to call on click outside
 * @param options - Options for the click detection
 */
export function useClickOutside(
  target: Ref<HTMLElement | null>,
  handler: (event: MouseEvent) => void,
  options: { immediate?: boolean } = {}
) {
  const { immediate = false } = options

  function listener(event: MouseEvent) {
    const el = target.value
    if (!el) return

    // Check if click was outside the element
    if (!el.contains(event.target as Node)) {
      handler(event)
    }
  }

  onMounted(() => {
    // Use capture phase if immediate, otherwise bubble phase
    document.addEventListener('click', listener, immediate)
  })

  onUnmounted(() => {
    document.removeEventListener('click', listener, immediate)
  })
}
