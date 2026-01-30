<template>
  <div class="tag-management-view">
    <!-- Tab Navigation -->
    <div class="tab-navigation">
      <button
        :class="['tab-btn', { active: activeTab === 'tags' }]"
        @click="activeTab = 'tags'"
      >
        Tag Management
      </button>
      <button
        :class="['tab-btn', { active: activeTab === 'templates' }]"
        @click="activeTab = 'templates'"
      >
        Templates
      </button>
    </div>

    <!-- Tag Management Tab -->
    <div v-show="activeTab === 'tags'" class="tab-content">
      <div v-if="loading" class="loading-state">Loading...</div>
      <div v-else-if="error" class="error-state">Error: {{ error }}</div>
      <div v-else class="dual-panel-layout">
        <!-- Left Panel: Tag Groups -->
        <div class="groups-panel">
          <div class="panel-header">
            <h3>Tag Groups</h3>
          </div>
          <div class="groups-list">
            <div
              v-for="group in tagGroups"
              :key="group.id"
              :class="['group-item', { selected: selectedGroupId === group.id }]"
              @click="selectGroup(group.id)"
              @contextmenu.prevent="showGroupContextMenu($event, group)"
            >
              <div class="group-info">
                <span
                  class="group-color"
                  :style="{ backgroundColor: group.color || '#9e9e9e' }"
                ></span>
                <span class="group-name">{{ group.name }}</span>
                <span class="group-count">({{ getTagCount(group.id) }})</span>
              </div>
              <button class="group-menu-btn" @click.stop="showGroupContextMenu($event, group)">
                ⋮
              </button>
            </div>
          </div>
        </div>

        <!-- Right Panel: Tag Details Table -->
        <div class="tags-panel">
          <div class="panel-header">
            <h3>{{ selectedGroupName }}</h3>
            <div class="header-actions">
              <button
                v-if="selectedTags.length > 0"
                class="btn-secondary"
                @click="showBatchActions = !showBatchActions"
              >
                Batch Actions ({{ selectedTags.length }})
              </button>
              <button class="btn-secondary" @click="showCreateGroup = true">
                + New Group
              </button>
              <button class="btn-primary" @click="handleAddTag" :disabled="!selectedGroupId">
                + Add Tag
              </button>
            </div>
          </div>

          <!-- Batch Actions Bar -->
          <div v-if="showBatchActions && selectedTags.length > 0" class="batch-actions-bar">
            <button class="batch-btn" @click="batchMoveToGroup">
              Move to Group...
            </button>
            <button class="batch-btn danger" @click="batchDelete">
              Delete Selected
            </button>
            <button class="batch-btn" @click="clearSelection">
              Clear Selection
            </button>
          </div>

          <div v-if="!selectedGroupId" class="empty-state">
            Select a tag group to view tags
          </div>
          <div v-else-if="currentGroupTags.length === 0" class="empty-state">
            No tags in this group. Click "+ Add Tag" to create one.
          </div>
          <div v-else class="tags-table">
            <table>
              <thead>
                <tr>
                  <th class="col-checkbox">
                    <input
                      type="checkbox"
                      :checked="allSelected"
                      @change="toggleSelectAll"
                    />
                  </th>
                  <th class="col-name">Tag Name</th>
                  <th class="col-usage">Usage</th>
                  <th class="col-actions">Actions</th>
                </tr>
              </thead>
              <tbody>
                <tr
                  v-for="tag in currentGroupTags"
                  :key="tag.id"
                  :class="{ selected: isTagSelected(tag.id) }"
                >
                  <td class="col-checkbox">
                    <input
                      type="checkbox"
                      :checked="isTagSelected(tag.id)"
                      @change="toggleTagSelection(tag.id)"
                    />
                  </td>
                  <td class="col-name">
                    <span class="tag-value">{{ tag.value }}</span>
                  </td>
                  <td class="col-usage">
                    <button
                      class="usage-btn"
                      @click="showTagUsage(tag)"
                      :disabled="getTagUsage(tag.id) === 0"
                    >
                      {{ getTagUsage(tag.id) }}
                    </button>
                  </td>
                  <td class="col-actions">
                    <button class="action-btn" @click="handleEditTag(tag)">
                      Edit
                    </button>
                    <button class="action-btn" @click="showMergeDialog(tag)">
                      Merge
                    </button>
                    <button class="action-btn danger" @click="handleDeleteTag(tag)">
                      Delete
                    </button>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>
      </div>
    </div>

    <!-- Templates Tab -->
    <div v-show="activeTab === 'templates'" class="tab-content">
      <TemplateManager />
    </div>

    <!-- Context Menu -->
    <ContextMenu
      :visible="contextMenu.show"
      :x="contextMenu.x"
      :y="contextMenu.y"
      :items="contextMenu.items"
      @update:visible="contextMenu.show = $event"
    />

    <!-- Dialogs -->
    <CreateGroupDialog
      v-model="showCreateGroup"
      :existing-groups="tagGroups"
      @create="handleCreateGroup"
    />

    <EditGroupDialog
      v-model="showEditGroup"
      :group="editingGroup"
      :existing-groups="tagGroups"
      @save="handleSaveGroup"
      @delete="handleDeleteGroupConfirm"
    />

    <CreateTagDialog
      v-model="showCreateTag"
      :group-id="selectedGroupId"
      :groups="tagGroups"
      :existing-tags="tags"
      @create="handleCreateTag"
    />

    <EditTagDialog
      v-model="showEditTag"
      :tag="editingTag"
      :groups="tagGroups"
      :existing-tags="tags"
      @save="handleSaveTag"
      @delete="handleDeleteTagConfirm"
    />

    <ConfirmDialog
      v-model="showConfirm"
      :title="confirmTitle"
      :message="confirmMessage"
      :description="confirmDescription"
      type="danger"
      confirm-text="Delete"
      @confirm="executeConfirm"
    />

    <!-- Merge Dialog -->
    <BaseDialog v-model="showMergeDialogModal" title="Merge Tag" width="500px">
      <div class="merge-dialog-content">
        <p class="merge-description">
          Merge tag "<strong>{{ mergingTag?.value }}</strong>" into another tag.
          All items with this tag will be updated to use the target tag instead.
        </p>

        <div class="form-group">
          <label>Select target tag:</label>
          <div class="tag-select-list">
            <label
              v-for="tag in availableMergeTags"
              :key="tag.id"
              class="tag-select-item"
              :class="{ selected: mergeTargetTagId === tag.id }"
            >
              <input
                type="radio"
                :value="tag.id"
                v-model="mergeTargetTagId"
                name="merge-target"
              />
              <span
                class="tag-color-dot"
                :style="{ backgroundColor: getTagColorById(tag.id) }"
              ></span>
              <span class="tag-name">{{ tag.value }}</span>
              <span class="tag-usage-count">({{ getTagUsage(tag.id) }} items)</span>
            </label>
          </div>
        </div>

        <div class="merge-warning">
          ⚠️ This action cannot be undone. The original tag will be deleted after merging.
        </div>
      </div>

      <template #footer>
        <button class="btn-secondary" @click="closeMergeDialog">Cancel</button>
        <button
          class="btn-primary"
          :disabled="!mergeTargetTagId"
          @click="executeMerge"
        >
          Merge
        </button>
      </template>
    </BaseDialog>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useTagsStore } from '@/stores/tags'
import { ConfirmDialog, ContextMenu, BaseDialog } from '@/components/base'
import TemplateManager from './TemplateManager.vue'
import {
  CreateGroupDialog,
  EditGroupDialog,
  CreateTagDialog,
  EditTagDialog,
} from './dialogs'
import type { Tag, TagGroup } from '@/types'

const tagsStore = useTagsStore()

// Tab state
const activeTab = ref<'tags' | 'templates'>('tags')

// Data
const tagGroups = computed(() => tagsStore.tagGroups)
const tags = computed(() => tagsStore.tags)
const loading = computed(() => tagsStore.loading)
const error = computed(() => tagsStore.error)
const usageCounts = computed(() => tagsStore.usageCounts)

// Selection state
const selectedGroupId = ref<number | null>(null)
const selectedTags = ref<number[]>([])
const showBatchActions = ref(false)

// Computed
const selectedGroupName = computed(() => {
  if (!selectedGroupId.value) return 'No Group Selected'
  const group = tagGroups.value.find((g) => g.id === selectedGroupId.value)
  return group ? `Tags in "${group.name}" Group` : 'Unknown Group'
})

const currentGroupTags = computed(() => {
  if (!selectedGroupId.value) return []
  return tagsStore.getTagsByGroup(selectedGroupId.value)
})

const allSelected = computed(() => {
  return (
    currentGroupTags.value.length > 0 &&
    currentGroupTags.value.every((tag) => selectedTags.value.includes(tag.id))
  )
})

const availableMergeTags = computed(() => {
  if (!mergingTag.value) return []
  return tags.value.filter((tag) => tag.id !== mergingTag.value!.id)
})

// Context Menu
const contextMenu = ref({
  show: false,
  x: 0,
  y: 0,
  items: [] as Array<{ label: string; action: () => void; danger?: boolean }>,
})

// Dialogs
const showCreateGroup = ref(false)
const showEditGroup = ref(false)
const editingGroup = ref<TagGroup | null>(null)
const showCreateTag = ref(false)
const showEditTag = ref(false)
const editingTag = ref<Tag | null>(null)
const showConfirm = ref(false)
const confirmTitle = ref('')
const confirmMessage = ref('')
const confirmDescription = ref('')
const pendingConfirmAction = ref<(() => Promise<void>) | null>(null)

// Merge Dialog
const showMergeDialogModal = ref(false)
const mergingTag = ref<Tag | null>(null)
const mergeTargetTagId = ref<number | null>(null)

onMounted(async () => {
  await tagsStore.loadTagGroups()
  await tagsStore.loadTags()
  await tagsStore.loadUsageCounts()

  // Select first group by default
  if (tagGroups.value.length > 0) {
    selectedGroupId.value = tagGroups.value[0].id
  }
})

function selectGroup(groupId: number) {
  selectedGroupId.value = groupId
  selectedTags.value = []
  showBatchActions.value = false
}

function getTagCount(groupId: number): number {
  return tagsStore.getTagsByGroup(groupId).length
}

function getTagUsage(tagId: number): number {
  return usageCounts.value[tagId] || 0
}

function isTagSelected(tagId: number): boolean {
  return selectedTags.value.includes(tagId)
}

function toggleTagSelection(tagId: number) {
  const index = selectedTags.value.indexOf(tagId)
  if (index > -1) {
    selectedTags.value.splice(index, 1)
  } else {
    selectedTags.value.push(tagId)
  }
}

function toggleSelectAll() {
  if (allSelected.value) {
    selectedTags.value = []
  } else {
    selectedTags.value = currentGroupTags.value.map((tag) => tag.id)
  }
}

function clearSelection() {
  selectedTags.value = []
  showBatchActions.value = false
}

// Group Actions
function showGroupContextMenu(event: MouseEvent, group: TagGroup) {
  contextMenu.value = {
    show: true,
    x: event.clientX,
    y: event.clientY,
    items: [
      { label: 'Edit Group', action: () => handleEditGroup(group) },
      { label: 'Change Color', action: () => handleEditGroup(group) },
      {
        label: 'Delete Group',
        action: () => handleDeleteGroup(group),
        danger: true,
      },
    ],
  }
}

async function handleCreateGroup(name: string, color: string) {
  try {
    await tagsStore.createTagGroup(name, color)
  } catch (e) {
    console.error('Failed to create group:', e)
  }
}

function handleEditGroup(group: TagGroup) {
  editingGroup.value = group
  showEditGroup.value = true
}

async function handleSaveGroup(id: number, name: string, color: string) {
  try {
    await tagsStore.updateTagGroup(id, name, color)
    showEditGroup.value = false
    editingGroup.value = null
  } catch (e) {
    console.error('Failed to update group:', e)
  }
}

function handleDeleteGroup(group: TagGroup) {
  confirmTitle.value = 'Delete Tag Group'
  confirmMessage.value = `Are you sure you want to delete "${group.name}"?`
  confirmDescription.value = 'All tags within this group will also be deleted.'
  pendingConfirmAction.value = async () => {
    await tagsStore.deleteTagGroup(group.id)
    if (selectedGroupId.value === group.id) {
      selectedGroupId.value = tagGroups.value[0]?.id || null
    }
  }
  showConfirm.value = true
}

function handleDeleteGroupConfirm(id: number) {
  const group = tagGroups.value.find((g) => g.id === id)
  if (group) {
    handleDeleteGroup(group)
    showEditGroup.value = false
    editingGroup.value = null
  }
}

// Tag Actions
function handleAddTag() {
  if (!selectedGroupId.value) return
  showCreateTag.value = true
}

async function handleCreateTag(groupId: number, value: string) {
  try {
    await tagsStore.createTag(groupId, value)
  } catch (e) {
    console.error('Failed to create tag:', e)
  }
}

function handleEditTag(tag: Tag) {
  editingTag.value = tag
  showEditTag.value = true
}

async function handleSaveTag(id: number, value: string) {
  try {
    await tagsStore.updateTag(id, value)
    showEditTag.value = false
    editingTag.value = null
  } catch (e) {
    console.error('Failed to update tag:', e)
  }
}

function handleDeleteTag(tag: Tag) {
  confirmTitle.value = 'Delete Tag'
  confirmMessage.value = `Are you sure you want to delete "${tag.value}"?`
  const usage = getTagUsage(tag.id)
  confirmDescription.value =
    usage > 0 ? `This tag is used by ${usage} item(s).` : ''
  pendingConfirmAction.value = async () => {
    await tagsStore.deleteTag(tag.id)
  }
  showConfirm.value = true
}

function handleDeleteTagConfirm(id: number) {
  const tag = tags.value.find((t) => t.id === id)
  if (tag) {
    handleDeleteTag(tag)
    showEditTag.value = false
    editingTag.value = null
  }
}

function showTagUsage(tag: Tag) {
  // TODO: Implement showing items with this tag
  console.log('Show usage for tag:', tag)
}

function showMergeDialog(tag: Tag) {
  mergingTag.value = tag
  mergeTargetTagId.value = null
  showMergeDialogModal.value = true
}

function closeMergeDialog() {
  showMergeDialogModal.value = false
  mergingTag.value = null
  mergeTargetTagId.value = null
}

async function executeMerge() {
  if (!mergingTag.value || !mergeTargetTagId.value) return

  try {
    await tagsStore.mergeTags(mergingTag.value.id, mergeTargetTagId.value)
    closeMergeDialog()
    await tagsStore.loadUsageCounts()
  } catch (e) {
    console.error('Failed to merge tags:', e)
  }
}

function getTagColorById(tagId: number): string {
  const tag = tags.value.find((t) => t.id === tagId)
  if (!tag) return '#9e9e9e'
  const group = tagGroups.value.find((g) => g.id === tag.group_id)
  return group?.color || '#9e9e9e'
}

// Batch Actions
function batchMoveToGroup() {
  // TODO: Implement batch move
  console.log('Batch move tags:', selectedTags.value)
}

function batchDelete() {
  confirmTitle.value = 'Delete Multiple Tags'
  confirmMessage.value = `Are you sure you want to delete ${selectedTags.value.length} tag(s)?`
  confirmDescription.value = 'This action cannot be undone.'
  pendingConfirmAction.value = async () => {
    for (const tagId of selectedTags.value) {
      await tagsStore.deleteTag(tagId)
    }
    clearSelection()
  }
  showConfirm.value = true
}

async function executeConfirm() {
  if (pendingConfirmAction.value) {
    try {
      await pendingConfirmAction.value()
    } catch (e) {
      console.error('Confirm action failed:', e)
    } finally {
      showConfirm.value = false
      pendingConfirmAction.value = null
    }
  }
}
</script>

<style scoped>
.tag-management-view {
  height: 100%;
  display: flex;
  flex-direction: column;
  background: var(--background);
}

/* Tab Navigation */
.tab-navigation {
  display: flex;
  gap: 0.5rem;
  background: var(--background);
  padding: 1rem 1.5rem 0 1.5rem;
  border-bottom: 1px solid var(--border-color);
}

.tab-btn {
  padding: 0.75rem 1.5rem;
  border: none;
  background: transparent;
  font-size: 14px;
  font-weight: 500;
  color: var(--text-secondary);
  cursor: pointer;
  border-radius: 8px 8px 0 0;
  position: relative;
  transition: all 0.2s ease;
  border: 1px solid transparent;
  border-bottom: none;
}

.tab-btn:hover:not(.active) {
  color: var(--text-primary);
  background: rgba(0, 0, 0, 0.03);
}

.tab-btn.active {
  color: var(--primary-color);
  background: var(--surface);
  border-color: var(--border-color);
  border-bottom-color: var(--surface);
  font-weight: 600;
}

.tab-btn.active::after {
  content: '';
  position: absolute;
  bottom: -1px;
  left: 0;
  right: 0;
  height: 2px;
  background: var(--surface);
}

/* Tab Content */
.tab-content {
  flex: 1;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

/* Dual Panel Layout */
.dual-panel-layout {
  flex: 1;
  display: flex;
  overflow: hidden;
}

.groups-panel {
  width: 280px;
  border-right: 1px solid var(--border-color);
  display: flex;
  flex-direction: column;
  background: var(--surface);
}

.tags-panel {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-width: 0;
}

.panel-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 1rem 1.5rem;
  border-bottom: 1px solid var(--border-color);
  background: var(--background);
}

.panel-header h3 {
  margin: 0;
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
}

.header-actions {
  display: flex;
  gap: 0.5rem;
}

/* Groups List */
.groups-list {
  flex: 1;
  overflow-y: auto;
  padding: 0.5rem;
}

.group-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0.75rem 1rem;
  margin-bottom: 4px;
  border-radius: 6px;
  cursor: pointer;
  transition: background 0.15s;
}

.group-item:hover {
  background: rgba(0, 0, 0, 0.04);
}

.group-item.selected {
  background: rgba(25, 118, 210, 0.1);
  border-left: 3px solid var(--primary-color);
}

.group-info {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  flex: 1;
  min-width: 0;
}

.group-color {
  width: 12px;
  height: 12px;
  border-radius: 3px;
  flex-shrink: 0;
}

.group-name {
  font-size: 14px;
  font-weight: 500;
  color: var(--text-primary);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.group-count {
  font-size: 12px;
  color: var(--text-secondary);
  flex-shrink: 0;
}

.group-menu-btn {
  background: none;
  border: none;
  font-size: 18px;
  color: var(--text-secondary);
  cursor: pointer;
  padding: 4px 8px;
  border-radius: 4px;
  opacity: 0;
  transition: all 0.15s;
}

.group-item:hover .group-menu-btn {
  opacity: 1;
}

.group-menu-btn:hover {
  background: rgba(0, 0, 0, 0.08);
}

/* Batch Actions Bar */
.batch-actions-bar {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.75rem 1.5rem;
  background: rgba(25, 118, 210, 0.08);
  border-bottom: 1px solid var(--border-color);
}

.batch-btn {
  padding: 6px 12px;
  border-radius: 4px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  border: 1px solid var(--border-color);
  background: var(--background);
  color: var(--text-primary);
  transition: all 0.15s;
}

.batch-btn:hover {
  background: var(--surface);
}

.batch-btn.danger {
  color: #dc3545;
  border-color: #dc3545;
}

.batch-btn.danger:hover {
  background: #dc3545;
  color: white;
}

/* Tags Table */
.tags-table {
  flex: 1;
  overflow-y: auto;
  padding: 1rem 1.5rem;
}

table {
  width: 100%;
  border-collapse: collapse;
}

thead {
  position: sticky;
  top: 0;
  background: var(--background);
  z-index: 1;
}

th {
  text-align: left;
  font-size: 12px;
  font-weight: 600;
  color: var(--text-secondary);
  padding: 0.75rem 0.5rem;
  border-bottom: 2px solid var(--border-color);
}

td {
  padding: 1rem 0.5rem;
  border-bottom: 1px solid var(--border-color);
}

tbody tr:hover {
  background: rgba(0, 0, 0, 0.02);
}

tbody tr.selected {
  background: rgba(25, 118, 210, 0.05);
}

.col-checkbox {
  width: 40px;
}

.col-name {
  flex: 1;
}

.col-usage {
  width: 80px;
  text-align: center;
}

.col-actions {
  width: 200px;
  text-align: right;
}

.tag-value {
  font-size: 14px;
  color: var(--text-primary);
}

.usage-btn {
  background: none;
  border: none;
  font-size: 13px;
  color: var(--primary-color);
  cursor: pointer;
  padding: 4px 8px;
  border-radius: 4px;
  transition: background 0.15s;
}

.usage-btn:hover:not(:disabled) {
  background: rgba(25, 118, 210, 0.1);
}

.usage-btn:disabled {
  color: var(--text-secondary);
  cursor: default;
}

.action-btn {
  background: none;
  border: none;
  font-size: 13px;
  color: var(--primary-color);
  cursor: pointer;
  padding: 6px 10px;
  border-radius: 4px;
  margin-left: 4px;
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

/* States */
.loading-state,
.error-state,
.empty-state {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--text-secondary);
  font-size: 14px;
  padding: 2rem;
}

.error-state {
  color: #dc3545;
}

/* Buttons */
.btn-primary,
.btn-secondary {
  padding: 8px 16px;
  border-radius: 6px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.15s;
  border: none;
}

.btn-primary {
  background: var(--primary-color);
  color: white;
}

.btn-primary:hover:not(:disabled) {
  background: var(--primary-dark);
}

.btn-primary:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.btn-secondary {
  background: var(--surface);
  color: var(--text-primary);
  border: 1px solid var(--border-color);
}

.btn-secondary:hover {
  background: var(--background);
}

/* Merge Dialog */
.merge-dialog-content {
  padding: 0.5rem 0;
}

.merge-description {
  margin: 0 0 1.5rem 0;
  font-size: 14px;
  line-height: 1.5;
  color: var(--text-primary);
}

.form-group {
  margin-bottom: 1rem;
}

.form-group label {
  display: block;
  font-size: 13px;
  font-weight: 500;
  color: var(--text-primary);
  margin-bottom: 0.5rem;
}

.tag-select-list {
  max-height: 300px;
  overflow-y: auto;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  padding: 0.5rem;
}

.tag-select-item {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 0.75rem;
  margin-bottom: 0.25rem;
  border-radius: 4px;
  cursor: pointer;
  transition: background 0.15s;
}

.tag-select-item:hover {
  background: rgba(0, 0, 0, 0.04);
}

.tag-select-item.selected {
  background: rgba(25, 118, 210, 0.1);
  border: 1px solid var(--primary-color);
}

.tag-select-item input[type="radio"] {
  margin: 0;
  cursor: pointer;
}

.tag-color-dot {
  width: 12px;
  height: 12px;
  border-radius: 50%;
  flex-shrink: 0;
}

.tag-name {
  flex: 1;
  font-size: 14px;
  color: var(--text-primary);
}

.tag-usage-count {
  font-size: 12px;
  color: var(--text-secondary);
}

.merge-warning {
  margin-top: 1rem;
  padding: 0.75rem;
  background: #fff3cd;
  border: 1px solid #ffc107;
  border-radius: 4px;
  font-size: 13px;
  color: #856404;
}
</style>
