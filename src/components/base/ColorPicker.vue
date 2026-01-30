<template>
  <div class="color-picker">
    <label v-if="label" class="color-picker-label">{{ label }}</label>

    <!-- Preset colors -->
    <div class="color-presets">
      <button
        v-for="color in presets"
        :key="color"
        type="button"
        class="color-swatch"
        :style="{ backgroundColor: color }"
        :class="{ selected: modelValue === color }"
        :title="color"
        @click="selectColor(color)"
      >
        <svg
          v-if="modelValue === color"
          xmlns="http://www.w3.org/2000/svg"
          width="16"
          height="16"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="3"
          stroke-linecap="round"
          stroke-linejoin="round"
        >
          <polyline points="20 6 9 17 4 12" />
        </svg>
      </button>
    </div>

    <!-- Custom color input -->
    <div class="color-input-wrapper">
      <input
        type="color"
        :value="modelValue"
        @input="handleColorInput"
        class="color-input"
      />
      <span class="color-value">{{ modelValue }}</span>
    </div>
  </div>
</template>

<script setup lang="ts">
export interface ColorPickerProps {
  modelValue: string
  label?: string
  presets?: string[]
}

withDefaults(defineProps<ColorPickerProps>(), {
  label: '',
  presets: () => [
    '#f44336', // Red
    '#e91e63', // Pink
    '#9c27b0', // Purple
    '#673ab7', // Deep Purple
    '#3f51b5', // Indigo
    '#2196f3', // Blue
    '#03a9f4', // Light Blue
    '#00bcd4', // Cyan
    '#009688', // Teal
    '#4caf50', // Green
    '#8bc34a', // Light Green
    '#cddc39', // Lime
    '#ffeb3b', // Yellow
    '#ffc107', // Amber
    '#ff9800', // Orange
    '#ff5722', // Deep Orange
    '#795548', // Brown
    '#9e9e9e', // Grey
    '#607d8b', // Blue Grey
  ],
})

const emit = defineEmits<{
  'update:modelValue': [value: string]
}>()

function selectColor(color: string) {
  emit('update:modelValue', color)
}

function handleColorInput(event: Event) {
  const target = event.target as HTMLInputElement
  emit('update:modelValue', target.value)
}
</script>

<style scoped>
.color-picker {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.color-picker-label {
  font-size: 14px;
  font-weight: 500;
  color: var(--text-primary);
}

.color-presets {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(36px, 1fr));
  gap: 8px;
}

.color-swatch {
  width: 36px;
  height: 36px;
  border: 2px solid transparent;
  border-radius: 6px;
  cursor: pointer;
  transition: var(--transition-fast);
  display: flex;
  align-items: center;
  justify-content: center;
  position: relative;
  padding: 0;
}

.color-swatch:hover {
  transform: scale(1.1);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2);
}

.color-swatch.selected {
  border-color: var(--text-primary);
  box-shadow: 0 0 0 2px var(--background), 0 0 0 4px var(--text-primary);
}

.color-swatch svg {
  color: white;
  filter: drop-shadow(0 1px 2px rgba(0, 0, 0, 0.3));
}

.color-input-wrapper {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 8px 12px;
  background: var(--surface);
  border: 1px solid var(--border-color);
  border-radius: 4px;
}

.color-input {
  width: 48px;
  height: 32px;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  background: transparent;
}

.color-input::-webkit-color-swatch-wrapper {
  padding: 0;
}

.color-input::-webkit-color-swatch {
  border: 2px solid var(--border-color);
  border-radius: 4px;
}

.color-value {
  font-family: 'Courier New', monospace;
  font-size: 14px;
  color: var(--text-secondary);
  user-select: all;
}
</style>
