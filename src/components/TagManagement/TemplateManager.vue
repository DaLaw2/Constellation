<template>
  <div class="template-manager">
    <div class="manager-header">
      <h3>Tag Templates</h3>
      <button class="btn btn-primary btn-sm" @click="openCreateDialog">
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
        @contextmenu.prevent="openContextMenu($event, template)"
      >
        <div class="template-info">
          <div class="template-name">{{ template.name }}</div>
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
        </div>
      </div>
    </div>

    <!-- Context menu -->
    <ContextMenu
      :visible="contextMenu.visible"
      :x="contextMenu.x"
      :y="contextMenu.y"
      :items="contextMenuItems"
      @update:visible="contextMenu.visible = $event"
      @select="contextMenu.visible = false"
    />

    <ConfirmDialog
      v-model="showDeleteConfirm"
      title="Delete Template"
      message="Are you sure you want to delete this template?"
      type="danger"
      confirm-text="Delete"
      @confirm="confirmDelete"
    />

    <!-- Create / Edit Template Dialog -->
    <div v-if="showDialog" class="dialog-overlay" @click.self="closeDialog">
      <div class="dialog">
        <h3>{{ editingTemplate ? 'Edit Template' : 'Create Template' }}</h3>

        <div class="form-group">
          <label>Template Name:</label>
          <input
            v-model="dialogName"
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
                    :checked="dialogTagIds.includes(tag.id)"
                    @change="toggleDialogTag(tag.id)"
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
            :disabled="!dialogName.trim() || dialogTagIds.length === 0"
            @click="saveTemplate"
          >
            {{ editingTemplate ? 'Save' : 'Create' }}
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
import { ConfirmDialog, ContextMenu } from '@/components/base'
import type { ContextMenuItem } from '@/components/base'
import type { Tag, TagTemplate } from '@/types'

const templatesStore = useTagTemplatesStore()
const tagsStore = useTagsStore()

const templates = computed(() => templatesStore.templates)
const loading = computed(() => templatesStore.loading)
const tagGroups = computed(() => tagsStore.tagGroups)

// Dialog state (shared for create and edit)
const showDialog = ref(false)
const editingTemplate = ref<TagTemplate | null>(null)
const dialogName = ref('')
const dialogTagIds = ref<number[]>([])

// Delete confirmation
const showDeleteConfirm = ref(false)
const pendingDeleteId = ref<number | null>(null)

// Context menu
const contextMenu = ref({
  visible: false,
  x: 0,
  y: 0,
  target: null as TagTemplate | null,
})

const contextMenuItems = computed((): ContextMenuItem[] => [
  { label: 'Edit', icon: '✏️', action: () => {
    if (contextMenu.value.target) openEditDialog(contextMenu.value.target)
  }},
  { divider: true },
  { label: 'Delete', icon: '🗑️', danger: true, action: () => {
    if (contextMenu.value.target) handleDelete(contextMenu.value.target.id)
  }},
])

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
  return template.tag_ids
    .map(tagId => tagsStore.tags.find(tag => tag.id === tagId))
    .filter((tag): tag is Tag => tag !== undefined)
}

function getTagColor(tag: Tag): string {
  const group = tagGroups.value.find(g => g.id === tag.group_id)
  return group?.color || '#9e9e9e'
}

// Dialog helpers
function openCreateDialog() {
  editingTemplate.value = null
  dialogName.value = ''
  dialogTagIds.value = []
  showDialog.value = true
}

function openEditDialog(template: TagTemplate) {
  editingTemplate.value = template
  dialogName.value = template.name
  dialogTagIds.value = [...template.tag_ids]
  showDialog.value = true
}

function closeDialog() {
  showDialog.value = false
  editingTemplate.value = null
  dialogName.value = ''
  dialogTagIds.value = []
}

function toggleDialogTag(tagId: number) {
  const index = dialogTagIds.value.indexOf(tagId)
  if (index === -1) {
    dialogTagIds.value.push(tagId)
  } else {
    dialogTagIds.value.splice(index, 1)
  }
}

async function saveTemplate() {
  if (!dialogName.value.trim() || dialogTagIds.value.length === 0) return

  try {
    if (editingTemplate.value) {
      await templatesStore.updateTemplate(
        editingTemplate.value.id,
        dialogName.value.trim(),
        dialogTagIds.value
      )
    } else {
      await templatesStore.createTemplate(dialogName.value.trim(), dialogTagIds.value)
    }
    closeDialog()
  } catch (e) {
    console.error('Failed to save template:', e)
  }
}

// Context menu
function openContextMenu(event: MouseEvent, template: TagTemplate) {
  contextMenu.value = {
    visible: true,
    x: event.clientX,
    y: event.clientY,
    target: template,
  }
}

// Delete
function handleDelete(id: number) {
  pendingDeleteId.value = id
  showDeleteConfirm.value = true
}

async function confirmDelete() {
  if (pendingDeleteId.value === null) return
  try {
    await templatesStore.deleteTemplate(pendingDeleteId.value)
  } catch (e) {
    console.error('Failed to delete template:', e)
  } finally {
    showDeleteConfirm.value = false
    pendingDeleteId.value = null
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
  padding: 12px;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  background: var(--background);
  cursor: default;
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
