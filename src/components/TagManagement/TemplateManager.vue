<template>
  <div class="template-manager">
    <div class="manager-header">
      <h3>Tag Templates</h3>
      <button class="btn btn-primary btn-sm" @click="showCreateDialog = true">
        + New Template
      </button>
    </div>

    <div v-if="loading" class="loading-state">
      Loading templates...
    </div>

    <div v-else-if="templates.length === 0" class="empty-state">
      <div class="empty-icon">📋</div>
      <div class="empty-text">No templates yet</div>
      <div class="empty-description">
        Create templates to quickly apply common tag combinations
      </div>
    </div>

    <div v-else class="template-list">
      <div
        v-for="template in templates"
        :key="template.id"
        class="template-item"
      >
        <div class="template-info">
          <div class="template-name">{{ template.name }}</div>
          <div class="template-tags">
            <span
              v-for="tag in template.tags"
              :key="tag.id"
              class="tag-badge"
              :style="{ backgroundColor: getTagColor(tag) }"
            >
              {{ tag.value }}
            </span>
          </div>
        </div>
        <div class="template-actions">
          <button class="btn-icon" title="Delete" @click="handleDelete(template.id)">
            🗑️
          </button>
        </div>
      </div>
    </div>

    <!-- Create Template Dialog -->
    <div v-if="showCreateDialog" class="dialog-overlay" @click.self="closeDialog">
      <div class="dialog">
        <h3>Create Template</h3>

        <div class="form-group">
          <label>Template Name:</label>
          <input
            v-model="newTemplateName"
            type="text"
            placeholder="e.g., Work Documents"
          />
        </div>

        <div class="form-group">
          <label>Select Tags:</label>
          <div class="tag-selection">
            <div v-for="group in tagGroups" :key="group.id" class="tag-group">
              <div class="group-label">
                <span
                  class="group-color"
                  :style="{ backgroundColor: group.color || '#9e9e9e' }"
                ></span>
                {{ group.name }}
              </div>
              <div class="group-tags">
                <label
                  v-for="tag in getTagsByGroup(group.id)"
                  :key="tag.id"
                  class="tag-checkbox"
                >
                  <input
                    type="checkbox"
                    :checked="selectedTagIds.includes(tag.id)"
                    @change="toggleTag(tag.id)"
                  />
                  {{ tag.value }}
                </label>
              </div>
            </div>
          </div>
        </div>

        <div class="dialog-actions">
          <button class="btn" @click="closeDialog">Cancel</button>
          <button
            class="btn btn-primary"
            :disabled="!newTemplateName.trim() || selectedTagIds.length === 0"
            @click="createTemplate"
          >
            Create
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useTagTemplatesStore } from '../../stores/tagTemplates'
import { useTagsStore, type Tag } from '../../stores/tags'

const templatesStore = useTagTemplatesStore()
const tagsStore = useTagsStore()

const templates = computed(() => templatesStore.templates)
const loading = computed(() => templatesStore.loading)
const tagGroups = computed(() => tagsStore.tagGroups)

const showCreateDialog = ref(false)
const newTemplateName = ref('')
const selectedTagIds = ref<number[]>([])

onMounted(() => {
  templatesStore.loadTemplates()
  if (tagsStore.tagGroups.length === 0) {
    tagsStore.loadTagGroups()
  }
  if (tagsStore.tags.length === 0) {
    tagsStore.loadTags()
  }
})

function getTagsByGroup(groupId: number) {
  return tagsStore.getTagsByGroup(groupId)
}

function getTagColor(tag: Tag): string {
  const group = tagGroups.value.find(g => g.id === tag.group_id)
  return group?.color || '#9e9e9e'
}

function toggleTag(tagId: number) {
  const index = selectedTagIds.value.indexOf(tagId)
  if (index === -1) {
    selectedTagIds.value.push(tagId)
  } else {
    selectedTagIds.value.splice(index, 1)
  }
}

function closeDialog() {
  showCreateDialog.value = false
  newTemplateName.value = ''
  selectedTagIds.value = []
}

async function createTemplate() {
  if (!newTemplateName.value.trim() || selectedTagIds.value.length === 0) return

  try {
    await templatesStore.createTemplate(newTemplateName.value.trim(), selectedTagIds.value)
    closeDialog()
  } catch (e) {
    console.error('Failed to create template:', e)
  }
}

async function handleDelete(id: number) {
  if (confirm('Delete this template?')) {
    try {
      await templatesStore.deleteTemplate(id)
    } catch (e) {
      console.error('Failed to delete template:', e)
    }
  }
}
</script>

<style scoped>
.template-manager {
  padding: 16px;
}

.manager-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 16px;
}

.manager-header h3 {
  font-size: 14px;
  font-weight: 600;
  margin: 0;
}

.btn-sm {
  padding: 4px 12px;
  font-size: 12px;
}

.loading-state {
  padding: 24px;
  text-align: center;
  color: var(--text-secondary);
}

.empty-state {
  padding: 32px;
  text-align: center;
}

.empty-icon {
  font-size: 48px;
  margin-bottom: 8px;
}

.empty-text {
  font-size: 14px;
  font-weight: 500;
  margin-bottom: 4px;
}

.empty-description {
  font-size: 12px;
  color: var(--text-secondary);
}

.template-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.template-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  background: var(--background);
}

.template-item:hover {
  background: var(--surface);
}

.template-info {
  flex: 1;
  min-width: 0;
}

.template-name {
  font-size: 14px;
  font-weight: 500;
  margin-bottom: 4px;
}

.template-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 4px;
}

.tag-badge {
  display: inline-flex;
  align-items: center;
  padding: 2px 8px;
  border-radius: 12px;
  font-size: 11px;
  color: white;
}

.template-actions {
  display: flex;
  gap: 4px;
}

.btn-icon {
  padding: 4px 8px;
  border: none;
  background: none;
  cursor: pointer;
  border-radius: 4px;
  opacity: 0.6;
}

.btn-icon:hover {
  opacity: 1;
  background: rgba(0, 0, 0, 0.05);
}

/* Dialog styles */
.dialog-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.dialog {
  background: var(--background);
  border-radius: 8px;
  padding: 24px;
  width: 450px;
  max-width: 90vw;
  max-height: 80vh;
  overflow-y: auto;
  box-shadow: 0 10px 30px rgba(0, 0, 0, 0.3);
}

.dialog h3 {
  margin: 0 0 16px 0;
  font-size: 18px;
}

.form-group {
  margin-bottom: 16px;
}

.form-group label {
  display: block;
  margin-bottom: 8px;
  font-size: 13px;
  font-weight: 500;
}

.form-group input[type="text"] {
  width: 100%;
  padding: 8px 12px;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  font-size: 14px;
}

.tag-selection {
  max-height: 250px;
  overflow-y: auto;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  padding: 8px;
}

.tag-group {
  margin-bottom: 12px;
}

.tag-group:last-child {
  margin-bottom: 0;
}

.group-label {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 12px;
  font-weight: 600;
  color: var(--text-secondary);
  text-transform: uppercase;
  margin-bottom: 6px;
}

.group-color {
  width: 10px;
  height: 10px;
  border-radius: 2px;
}

.group-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  padding-left: 16px;
}

.tag-checkbox {
  display: flex;
  align-items: center;
  gap: 4px;
  font-size: 13px;
  cursor: pointer;
}

.dialog-actions {
  display: flex;
  gap: 8px;
  justify-content: flex-end;
  margin-top: 24px;
}

.btn {
  padding: 8px 16px;
  border: 1px solid var(--border-color);
  background: var(--background);
  border-radius: 4px;
  font-size: 13px;
  cursor: pointer;
}

.btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn-primary {
  background: var(--primary-color);
  border-color: var(--primary-color);
  color: white;
}

.btn-primary:hover:not(:disabled) {
  opacity: 0.9;
}
</style>
