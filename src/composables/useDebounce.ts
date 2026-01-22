import { ref, onUnmounted } from 'vue'

/**
 * Creates a debounced function that delays invoking the callback.
 * @param callback - The function to debounce
 * @param delay - The delay in milliseconds
 * @returns Object with debounced function and cancel method
 */
export function useDebounce<T extends (...args: unknown[]) => void>(
  callback: T,
  delay: number
) {
  const timer = ref<ReturnType<typeof setTimeout> | null>(null)

  function debounced(...args: Parameters<T>) {
    if (timer.value) {
      clearTimeout(timer.value)
    }
    timer.value = setTimeout(() => {
      callback(...args)
      timer.value = null
    }, delay)
  }

  function cancel() {
    if (timer.value) {
      clearTimeout(timer.value)
      timer.value = null
    }
  }

  onUnmounted(() => {
    cancel()
  })

  return {
    debounced,
    cancel,
  }
}
