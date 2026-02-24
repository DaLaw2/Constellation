<template>
  <Teleport to="body">
    <div class="batch-tag-selector-overlay" @click="emit('close')">
      <div class="batch-tag-selector" @click.stop>
        <div class="selector-header">
          <span class="selector-title">
            {{ mode === 'add' ? 'Add Tags' : 'Remove Tags' }}
          </span>
          <span class="file-count">{{ selectedPaths.length }} file{{ selectedPaths.length > 1 ? 's' : '' }}</span>
          <button class="btn-close" @click="emit('close')">
            <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <line x1="18" y1="6" x2="6" y2="18"></line>
              <line x1="6" y1="6" x2="18" y2="18"></line>
            </svg>
          </button>
        </div>

        <div class="selector-body">
          <!-- Loading state -->
          <div v-if="loading" class="loading-state">
            <div class="loading-spinner"></div>
            Loading tags...
          </div>

          <!-- For 'remove' mode: Show only common tags -->
          <template v-else-if="mode === 'remove'">
            <div v-if="commonTags.length === 0" class="empty-state">
              No common tags across selected files
            </div>
            <div v-else class="tag-list">
              <label v-for="tag in commonTags" :key="tag.id" class="tag-option">
                <input type="checkbox" v-model="selectedTagIds" :value="tag.id" />
                <span class="tag-badge" :style="{ backgroundColor: getTagColor(tag.group_id) }">
                  {{ tag.value }}
                </span>
              </label>
            </div>
          </template>

          <!-- For 'add' mode: Show all available tags grouped -->
          <template v-else>
            <div v-if="tagGroups.length === 0" class="empty-state">
              No tags available. Create tags first.
            </div>
            <div v-else class="tag-groups">
              <div v-for="group in tagGroups" :key="group.id" class="tag-group">
                <div class="group-header">
                  <span class="group-color" :style="{ backgroundColor: group.color || '#9e9e9e' }"></span>
                  <span class="group-name">{{ group.name }}</span>
                </div>
                <div class="group-tags">
                  <label
                    v-for="tag in getTagsByGroup(group.id)"
                    :key="tag.id"
                    class="tag-option"
                  >
                    <input type="checkbox" v-model="selectedTagIds" :value="tag.id" />
                    <span class="tag-label">{{ tag.value }}</span>
                  </label>
                </div>
              </div>
            </div>
          </template>
        </div>

        <div class="selector-footer">
          <span class="selected-count">{{ selectedTagIds.length }} tag{{ selectedTagIds.length !== 1 ? 's' : '' }} selected</span>
          <div class="footer-actions">
            <button class="btn-cancel" @click="emit('close')">Cancel</button>
            <button
              class="btn-apply"
              :disabled="selectedTagIds.length === 0"
              @click="handleApply"
            >
              {{ mode === 'add' ? 'Add Tags' : 'Remove Tags' }}
            </button>
          </div>
        </div>
      </div>
    </div>
  </Teleport>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useTagsStore } from '@/stores/tags'
import { useItemsStore } from '@/stores/items'
import type { Tag } from '@/types'

interface Props {
  mode: 'add' | 'remove'
  selectedPaths: string[]
}

const props = defineProps<Props>()

const emit = defineEmits<{
  apply: [paths: string[], tagIds: number[]]
  close: []
}>()

const tagsStore = useTagsStore()
const itemsStore = useItemsStore()

const loading = ref(false)
const commonTags = ref<Tag[]>([])
const selectedTagIds = ref<number[]>([])

const tagGroups = computed(() => tagsStore.tagGroups)
const tags = computed(() => tagsStore.tags)

function getTagsByGroup(groupId: number): Tag[] {
  return tags.value.filter(t => t.group_id === groupId)
}

function getTagColor(groupId: number): string {
  const group = tagGroups.value.find(g => g.id === groupId)
  return group?.color || '#9e9e9e'
}

function handleApply() {
  emit('apply', props.selectedPaths, selectedTagIds.value)
}

onMounted(async () => {
  // Ensure tags are loaded
  if (tagsStore.tagGroups.length === 0) {
    await tagsStore.loadTagGroups()
  }
  if (tagsStore.tags.length === 0) {
    await tagsStore.loadTags()
  }

  // For remove mode, load common tags
  if (props.mode === 'remove') {
    loading.value = true
    try {
      commonTags.value = await itemsStore.getCommonTagsForPaths(props.selectedPaths)
    } catch (e) {
      console.error('Failed to get common tags:', e)
      commonTags.value = []
    } finally {
      loading.value = false
    }
  }
})
</script>

<style scoped>
.batch-tag-selector-overlay {
  position: fixed;
  inset: 0;
  z-index: 1001;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
}

.batch-tag-selector {
  width: 400px;
  max-width: 90vw;
  max-height: 70vh;
  background: var(--surface);
  border-radius: 12px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.2);
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.selector-header {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 16px 20px;
  border-bottom: 1px solid var(--border-color);
}

.selector-title {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
}

.file-count {
  font-size: 13px;
  color: var(--text-secondary);
  margin-left: auto;
}

.btn-close {
  background: none;
  border: none;
  padding: 4px;
  cursor: pointer;
  color: var(--text-secondary);
  border-radius: 4px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.btn-close:hover {
  background: rgba(0, 0, 0, 0.04);
  color: var(--text-primary);
}

.selector-body {
  flex: 1;
  overflow-y: auto;
  padding: 16px 20px;
  min-height: 200px;
}

.loading-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 12px;
  padding: 32px;
  color: var(--text-secondary);
}

.loading-spinner {
  width: 24px;
  height: 24px;
  border: 2px solid var(--border-color);
  border-top-color: var(--primary-color);
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

.empty-state {
  text-align: center;
  padding: 32px;
  color: var(--text-secondary);
  font-size: 14px;
}

.tag-list {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}

.tag-groups {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.tag-group {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.group-header {
  display: flex;
  align-items: center;
  gap: 8px;
  padding-bottom: 4px;
  border-bottom: 1px solid var(--border-color);
}

.group-color {
  width: 12px;
  height: 12px;
  border-radius: 3px;
}

.group-name {
  font-size: 13px;
  font-weight: 500;
  color: var(--text-secondary);
}

.group-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
  padding-left: 4px;
}

.tag-option {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 4px 10px;
  border-radius: 4px;
  cursor: pointer;
  transition: var(--transition-fast);
}

.tag-option:hover {
  background: rgba(0, 0, 0, 0.04);
}

.tag-option input[type="checkbox"] {
  cursor: pointer;
}

.tag-badge {
  padding: 2px 8px;
  border-radius: 4px;
  font-size: 12px;
  color: white;
}

.tag-label {
  font-size: 13px;
  color: var(--text-primary);
}

.selector-footer {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 20px;
  border-top: 1px solid var(--border-color);
  background: var(--background);
}

.selected-count {
  font-size: 13px;
  color: var(--text-secondary);
}

.footer-actions {
  display: flex;
  gap: 8px;
}

.btn-cancel,
.btn-apply {
  padding: 8px 16px;
  border-radius: 6px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: var(--transition-fast);
}

.btn-cancel {
  background: transparent;
  border: 1px solid var(--border-color);
  color: var(--text-primary);
}

.btn-cancel:hover {
  background: rgba(0, 0, 0, 0.04);
}

.btn-apply {
  background: var(--primary-color);
  border: none;
  color: white;
}

.btn-apply:hover:not(:disabled) {
  filter: brightness(0.85);
}

.btn-apply:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}
</style>
