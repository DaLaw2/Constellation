<template>
  <Teleport to="body">
    <Transition name="slide-up">
      <div v-if="shouldShow" class="batch-action-bar">
        <div class="batch-info">
          <span class="batch-count">{{ selectedCount }} file{{ selectedCount > 1 ? 's' : '' }} selected</span>
        </div>

        <div class="batch-actions">
          <button class="batch-btn batch-btn-primary" @click="openAddTags">
            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path d="M12 5v14M5 12h14"/>
            </svg>
            Add Tags
          </button>
          <button class="batch-btn batch-btn-outline" @click="openRemoveTags">
            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path d="M5 12h14"/>
            </svg>
            Remove Tags
          </button>
          <button class="batch-btn batch-btn-text" @click="emit('clear')">
            Cancel
          </button>
        </div>

        <!-- Tag Selector Dialog -->
        <BatchTagSelector
          v-if="showTagSelector"
          :mode="tagSelectorMode"
          :selected-paths="Array.from(selectedPaths)"
          @apply="handleApplyTags"
          @close="closeTagSelector"
        />
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useItemsStore } from '@/stores/items'
import { useAppStore } from '@/stores/app'
import BatchTagSelector from '@/components/TagManagement/BatchTagSelector.vue'

interface Props {
  selectedPaths: Set<string>
  selectedCount: number
}

const props = defineProps<Props>()
const appStore = useAppStore()

// Hide when sidebar is expanded (Tags/Search panel is open)
const shouldShow = computed(() => {
  return props.selectedCount > 0 && !appStore.sidebarExpanded
})

const emit = defineEmits<{
  clear: []
}>()

const itemsStore = useItemsStore()

const showTagSelector = ref(false)
const tagSelectorMode = ref<'add' | 'remove'>('add')

function openAddTags() {
  tagSelectorMode.value = 'add'
  showTagSelector.value = true
}

function openRemoveTags() {
  tagSelectorMode.value = 'remove'
  showTagSelector.value = true
}

function closeTagSelector() {
  showTagSelector.value = false
}

async function handleApplyTags(paths: string[], tagIds: number[]) {
  closeTagSelector()

  if (tagIds.length === 0 || paths.length === 0) return

  try {
    if (tagSelectorMode.value === 'add') {
      // Add each selected tag to all paths
      for (const tagId of tagIds) {
        await itemsStore.batchAddTagToItems(paths, tagId)
      }
    } else {
      // Remove each selected tag from all paths
      for (const tagId of tagIds) {
        await itemsStore.batchRemoveTagFromItems(paths, tagId)
      }
    }
  } catch (e) {
    console.error('Batch tag operation failed:', e)
  }
}
</script>

<style scoped>
.batch-action-bar {
  position: fixed;
  bottom: 0;
  left: 0;
  right: 0;
  z-index: 1000;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 24px;
  background: var(--surface);
  border-top: 1px solid var(--border-color);
  box-shadow: 0 -2px 10px rgba(0, 0, 0, 0.1);
}

.batch-info {
  display: flex;
  align-items: center;
  gap: 8px;
}

.batch-count {
  font-size: 14px;
  font-weight: 500;
  color: var(--primary-color);
}

.batch-actions {
  display: flex;
  align-items: center;
  gap: 8px;
}

.batch-btn {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 8px 16px;
  border-radius: 6px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: var(--transition-fast);
  border: none;
}

.batch-btn-primary {
  background: var(--primary-color);
  color: white;
}

.batch-btn-primary:hover {
  filter: brightness(0.85);
}

.batch-btn-outline {
  background: transparent;
  color: var(--text-primary);
  border: 1px solid var(--border-color);
}

.batch-btn-outline:hover {
  background: rgba(0, 0, 0, 0.04);
}

.batch-btn-text {
  background: transparent;
  color: var(--text-secondary);
}

.batch-btn-text:hover {
  color: var(--text-primary);
  background: rgba(0, 0, 0, 0.04);
}

/* Slide up animation */
.slide-up-enter-active,
.slide-up-leave-active {
  transition: transform 0.2s ease, opacity 0.2s ease;
}

.slide-up-enter-from,
.slide-up-leave-to {
  transform: translateY(100%);
  opacity: 0;
}
</style>
