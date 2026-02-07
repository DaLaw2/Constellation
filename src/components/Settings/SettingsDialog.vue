<template>
  <BaseDialog
    :model-value="modelValue"
    @update:model-value="emit('update:modelValue', $event)"
    title="Settings"
    width="620px"
  >
    <!-- Tab Strip -->
    <div class="settings-tabs">
      <button
        :class="['tab-btn', { active: activeTab === 'tracking' }]"
        @click="activeTab = 'tracking'"
      >
        File Tracking
      </button>
      <button
        :class="['tab-btn', { active: activeTab === 'preview' }]"
        @click="activeTab = 'preview'"
      >
        Preview
      </button>
      <button
        :class="['tab-btn', { active: activeTab === 'about' }]"
        @click="activeTab = 'about'"
      >
        About
      </button>
    </div>

    <!-- Tab Content -->
    <div class="settings-content">
      <FileTrackingSettings v-if="activeTab === 'tracking'" />
      <PreviewSettings v-else-if="activeTab === 'preview'" />
      <AboutTab v-else />
    </div>
  </BaseDialog>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { BaseDialog } from '@/components/base'
import FileTrackingSettings from './FileTrackingSettings.vue'
import PreviewSettings from './PreviewSettings.vue'
import AboutTab from './AboutTab.vue'

defineProps<{
  modelValue: boolean
}>()

const emit = defineEmits<{
  'update:modelValue': [value: boolean]
}>()

const activeTab = ref<'tracking' | 'preview' | 'about'>('tracking')
</script>

<style scoped>
.settings-tabs {
  display: flex;
  gap: 0;
  border-bottom: 1px solid var(--border-color);
  margin: -24px -24px 16px -24px;
  padding: 0 24px;
}

.tab-btn {
  padding: 10px 20px;
  border: none;
  background: transparent;
  font-size: 13px;
  font-weight: 500;
  color: var(--text-secondary);
  cursor: pointer;
  border-bottom: 2px solid transparent;
  transition: var(--transition-fast);
  margin-bottom: -1px;
}

.tab-btn:hover {
  color: var(--text-primary);
}

.tab-btn.active {
  color: var(--primary-color);
  border-bottom-color: var(--primary-color);
}

.settings-content {
  min-height: 280px;
}
</style>
