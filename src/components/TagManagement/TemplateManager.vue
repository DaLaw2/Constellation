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
      <table>
        <thead>
          <tr>
            <th class="col-name">Template Name</th>
            <th class="col-tags">Tags</th>
            <th class="col-actions">Actions</th>
          </tr>
        </thead>
        <tbody>
          <tr
            v-for="template in templates"
            :key="template.id"
            class="template-row"
          >
            <td class="col-name">{{ template.name }}</td>
            <td class="col-tags">
              <div class="template-tags">
                <span
                  v-for="tag in getTemplateTags(template)"
                  :key="tag.id"
                  class="tag-badge"
                  :style="{ backgroundColor: getTagColor(tag) }"
                >
                  {{ tag.value }}
                </span>
              </div>
            </td>
            <td class="col-actions">
              <button class="action-btn" @click="handleEdit(template)">
                Edit
              </button>
              <button class="action-btn danger" @click="handleDelete(template.id)">
                Delete
              </button>
            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <!-- Delete Confirmation Dialog -->
    <ConfirmDialog
      v-model="showDeleteDialog"
      title="Delete Template"
      :message="`Are you sure you want to delete &quot;${deletingTemplateName}&quot;?`"
      description="This action cannot be undone."
      type="danger"
      confirm-text="Delete"
      @confirm="confirmDelete"
      @cancel="cancelDelete"
    />

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
            @input="nameError = ''"
          />
          <div v-if="nameError" class="error-message">{{ nameError }}</div>
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
            :disabled="!newTemplateName.trim() || selectedTagIds.length === 0 || !!nameError"
            @click="createTemplate"
          >
            Create
          </button>
        </div>
      </div>
    </div>

    <!-- Edit Template Dialog -->
    <div v-if="showEditDialog" class="dialog-overlay" @click.self="closeEditDialog">
      <div class="dialog">
        <h3>Edit Template</h3>

        <div class="form-group">
          <label>Template Name:</label>
          <input
            v-model="editingTemplateName"
            type="text"
            placeholder="e.g., Work Documents"
            @input="nameError = ''"
          />
          <div v-if="nameError" class="error-message">{{ nameError }}</div>
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
                    :checked="editingTemplateTagIds.includes(tag.id)"
                    @change="toggleEditTag(tag.id)"
                  />
                  {{ tag.value }}
                </label>
              </div>
            </div>
          </div>
        </div>

        <div class="dialog-actions">
          <button class="btn" @click="closeEditDialog">Cancel</button>
          <button
            class="btn btn-primary"
            :disabled="!editingTemplateName.trim() || editingTemplateTagIds.length === 0 || !!nameError"
            @click="saveEditedTemplate"
          >
            Save
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useTagTemplatesStore } from '@/stores/tagTemplates'
import { useTagsStore } from '@/stores/tags'
import { ConfirmDialog } from '@/components/base'
import type { Tag } from '@/types'

const templatesStore = useTagTemplatesStore()
const tagsStore = useTagsStore()

const templates = computed(() => templatesStore.templates)
const loading = computed(() => templatesStore.loading)
const tagGroups = computed(() => tagsStore.tagGroups)

const showCreateDialog = ref(false)
const newTemplateName = ref('')
const selectedTagIds = ref<number[]>([])

const showDeleteDialog = ref(false)
const deletingTemplateId = ref<number | null>(null)
const deletingTemplateName = ref('')

const showEditDialog = ref(false)
const editingTemplateId = ref<number | null>(null)
const editingTemplateName = ref('')
const editingTemplateTagIds = ref<number[]>([])
const nameError = ref('')

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

function getTemplateTags(template: { tag_ids: number[] }): Tag[] {
  // Resolve tag_ids to full Tag objects from the tags store
  return template.tag_ids
    .map(tagId => tagsStore.tags.find(tag => tag.id === tagId))
    .filter((tag): tag is Tag => tag !== undefined)
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

function validateTemplateName(name: string, excludeId: number | null = null): boolean {
  const trimmedName = name.trim()
  if (!trimmedName) {
    nameError.value = 'Template name cannot be empty'
    return false
  }

  const duplicate = templates.value.find(
    t => t.name.toLowerCase() === trimmedName.toLowerCase() && t.id !== excludeId
  )

  if (duplicate) {
    nameError.value = 'A template with this name already exists'
    return false
  }

  nameError.value = ''
  return true
}

function closeDialog() {
  showCreateDialog.value = false
  newTemplateName.value = ''
  selectedTagIds.value = []
  nameError.value = ''
}

async function createTemplate() {
  if (!newTemplateName.value.trim() || selectedTagIds.value.length === 0) return

  if (!validateTemplateName(newTemplateName.value)) {
    return
  }

  try {
    await templatesStore.createTemplate(newTemplateName.value.trim(), selectedTagIds.value)
    closeDialog()
  } catch (e) {
    console.error('Failed to create template:', e)
  }
}

function handleEdit(template: { id: number; name: string; tag_ids: number[] }) {
  editingTemplateId.value = template.id
  editingTemplateName.value = template.name
  editingTemplateTagIds.value = [...template.tag_ids]
  showEditDialog.value = true
  nameError.value = ''
}

function closeEditDialog() {
  showEditDialog.value = false
  editingTemplateId.value = null
  editingTemplateName.value = ''
  editingTemplateTagIds.value = []
  nameError.value = ''
}

function toggleEditTag(tagId: number) {
  const index = editingTemplateTagIds.value.indexOf(tagId)
  if (index === -1) {
    editingTemplateTagIds.value.push(tagId)
  } else {
    editingTemplateTagIds.value.splice(index, 1)
  }
}

async function saveEditedTemplate() {
  if (!editingTemplateName.value.trim() || editingTemplateTagIds.value.length === 0) return

  if (!validateTemplateName(editingTemplateName.value, editingTemplateId.value)) {
    return
  }

  try {
    await templatesStore.updateTemplate(
      editingTemplateId.value!,
      editingTemplateName.value.trim(),
      editingTemplateTagIds.value
    )
    closeEditDialog()
  } catch (e) {
    console.error('Failed to update template:', e)
  }
}

function handleDelete(id: number) {
  const template = templates.value.find(t => t.id === id)
  if (template) {
    showDeleteDialog.value = true
    deletingTemplateId.value = id
    deletingTemplateName.value = template.name
  }
}

async function confirmDelete() {
  if (deletingTemplateId.value !== null) {
    try {
      await templatesStore.deleteTemplate(deletingTemplateId.value)
      showDeleteDialog.value = false
      deletingTemplateId.value = null
      deletingTemplateName.value = ''
    } catch (e) {
      console.error('Failed to delete template:', e)
    }
  }
}

function cancelDelete() {
  showDeleteDialog.value = false
  deletingTemplateId.value = null
  deletingTemplateName.value = ''
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
  overflow-y: auto;
  flex: 1;
}

.template-list table {
  width: 100%;
  border-collapse: collapse;
}

.template-list thead {
  position: sticky;
  top: 0;
  background: var(--background);
  z-index: 1;
}

.template-list th {
  text-align: left;
  font-size: 12px;
  font-weight: 600;
  color: var(--text-secondary);
  padding: 0.75rem 0.5rem;
  border-bottom: 2px solid var(--border-color);
}

.template-list td {
  padding: 1rem 0.5rem;
  border-bottom: 1px solid var(--border-color);
}

.template-row:hover {
  background: rgba(0, 0, 0, 0.02);
}

.col-name {
  width: 30%;
  font-size: 14px;
  font-weight: 500;
  color: var(--text-primary);
}

.col-tags {
  width: 50%;
}

.col-actions {
  width: 20%;
  text-align: right;
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

.action-btn {
  background: none;
  border: none;
  font-size: 13px;
  color: var(--primary-color);
  cursor: pointer;
  padding: 6px 10px;
  border-radius: 4px;
  transition: all 0.15s;
}

.action-btn:hover {
  background: rgba(25, 118, 210, 0.1);
}

.action-btn.danger {
  color: #dc3545;
}

.action-btn.danger:hover {
  background: rgba(220, 53, 69, 0.1);
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

.error-message {
  margin-top: 4px;
  font-size: 12px;
  color: #dc3545;
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
