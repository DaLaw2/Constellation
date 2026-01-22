import { ref, watch, type Ref } from 'vue'

/**
 * Creates a ref that syncs with localStorage.
 * @param key - The localStorage key
 * @param defaultValue - The default value if key doesn't exist
 * @returns A reactive ref that persists to localStorage
 */
export function useLocalStorage<T>(key: string, defaultValue: T): Ref<T> {
  // Read initial value from localStorage
  const storedValue = localStorage.getItem(key)
  const initialValue = storedValue !== null
    ? parseValue(storedValue, defaultValue)
    : defaultValue

  const data = ref(initialValue) as Ref<T>

  // Watch for changes and persist to localStorage
  watch(
    data,
    (newValue) => {
      if (newValue === null || newValue === undefined) {
        localStorage.removeItem(key)
      } else {
        localStorage.setItem(key, stringifyValue(newValue))
      }
    },
    { deep: true }
  )

  return data
}

function parseValue<T>(value: string, defaultValue: T): T {
  try {
    // Handle primitive types
    if (typeof defaultValue === 'number') {
      return parseFloat(value) as T
    }
    if (typeof defaultValue === 'boolean') {
      return (value === 'true') as T
    }
    if (typeof defaultValue === 'string') {
      return value as T
    }
    // Handle objects/arrays
    return JSON.parse(value) as T
  } catch {
    return defaultValue
  }
}

function stringifyValue<T>(value: T): string {
  if (typeof value === 'string') {
    return value
  }
  if (typeof value === 'number' || typeof value === 'boolean') {
    return String(value)
  }
  return JSON.stringify(value)
}
