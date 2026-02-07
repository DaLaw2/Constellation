<template>
  <div class="tag-management-expanded">
    <!-- Header -->
    <div class="expanded-header">
      <div class="header-left">
        <h2 class="header-title">Tag Management</h2>
        <div class="tab-strip">
          <button
            :class="['tab-btn', { active: activeTab === 'tags' }]"
            @click="activeTab = 'tags'"
          >
            Tags
          </button>
          <button
            :class="['tab-btn', { active: activeTab === 'templates' }]"
            @click="activeTab = 'templates'"
          >
            Templates
          </button>
        </div>
      </div>
      <div class="header-actions">
        <BaseButton
          v-if="activeTab === 'tags'"
          variant="primary"
          size="medium"
          @click="showCreateGroup = true"
        >
          + New Group
        </BaseButton>
      </div>
    </div>

    <!-- Tags Tab: Dual-panel layout -->
    <div v-if="activeTab === 'tags'" class="dual-panel">
      <div class="panel-left" :style="{ width: panelWidth + 'px' }">
        <GroupListPanel
          :groups="tagGroups"
          :tags="tags"
          :selected-group-id="selectedGroupId"
          @select-group="selectedGroupId = $event"
          @reorder="handleReorder"
          @edit-group="handleEditGroup"
          @delete-group="handleDeleteGroup"
        />
      </div>

      <div
        class="panel-resizer"
        @mousedown="startResize"
      ></div>

      <div class="panel-right">
        <TagDetailTable
          :group-id="selectedGroupId"
          :tags="tags"
          :usage-counts="usageCounts"
          :groups="tagGroups"
          @add-tag="handleAddTag"
          @edit-tag="handleEditTag"
          @delete-tag="handleDeleteTag"
          @move-tag="handleMoveTag"
          @merge-tag="handleMergeTag"
          @batch-move="handleBatchMove"
          @batch-delete="handleBatchDelete"
        />
      </div>
    </div>

    <!-- Templates Tab -->
    <div v-else class="templates-panel">
      <TemplateManager />
    </div>

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
      ref="createTagDialog"
      v-model="showCreateTag"
      :group-id="targetGroupId"
      :groups="tagGroups"
      :existing-tags="tags"
      @create="handleCreateTag"
      @search="handleTagSearch"
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

    <MoveTagsDialog
      v-model="showMoveDialog"
      :tag-ids="moveTagIds"
      :groups="tagGroups"
      :current-group-id="selectedGroupId"
      @confirm="executeMoveToGroup"
    />

    <MergeTagDialog
      v-model="showMergeDialog"
      :source-tag="mergingTag"
      :tags="tags"
      :groups="tagGroups"
      :usage-counts="usageCounts"
      @confirm="executeMerge"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useTagsStore } from '@/stores/tags'
import { BaseButton, ConfirmDialog } from '@/components/base'
import { useResizablePanel } from '@/composables'
import GroupListPanel from './GroupListPanel.vue'
import TagDetailTable from './TagDetailTable.vue'
import MoveTagsDialog from './MoveTagsDialog.vue'
import MergeTagDialog from './MergeTagDialog.vue'
import TemplateManager from '../TemplateManager.vue'
import {
  CreateGroupDialog,
  EditGroupDialog,
  CreateTagDialog,
  EditTagDialog,
} from '../dialogs'
import type { Tag, TagGroup } from '@/types'

const tagsStore = useTagsStore()

const tagGroups = computed(() => tagsStore.tagGroups)
const tags = computed(() => tagsStore.tags)
const usageCounts = computed(() => tagsStore.usageCounts)

// UI state
const activeTab = ref<'tags' | 'templates'>('tags')
const selectedGroupId = ref<number | null>(null)

// Resizable panel
const { width: panelWidth, startResize } = useResizablePanel({
  minWidth: 180,
  maxWidth: 400,
  initialWidth: 250,
})

// Create Group Dialog
const showCreateGroup = ref(false)

// Edit Group Dialog
const showEditGroup = ref(false)
const editingGroup = ref<TagGroup | null>(null)

// Create Tag Dialog
const showCreateTag = ref(false)
const targetGroupId = ref<number | null>(null)
const createTagDialog = ref<InstanceType<typeof CreateTagDialog> | null>(null)

// Edit Tag Dialog
const showEditTag = ref(false)
const editingTag = ref<Tag | null>(null)

// Move Dialog
const showMoveDialog = ref(false)
const moveTagIds = ref<number[]>([])

// Merge Dialog
const showMergeDialog = ref(false)
const mergingTag = ref<Tag | null>(null)

// Confirm Dialog
const showConfirm = ref(false)
const confirmTitle = ref('')
const confirmMessage = ref('')
const confirmDescription = ref('')
const pendingConfirmAction = ref<(() => Promise<void>) | null>(null)

onMounted(() => {
  tagsStore.loadTagGroups()
  tagsStore.loadTags()
  tagsStore.loadUsageCounts()
})

// Group handlers
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
      selectedGroupId.value = null
    }
  }
  showConfirm.value = true
}

function handleDeleteGroupConfirm(id: number) {
  const group = tagGroups.value.find(g => g.id === id)
  if (group) {
    handleDeleteGroup(group)
    showEditGroup.value = false
    editingGroup.value = null
  }
}

async function handleReorder(groupIds: number[]) {
  try {
    await tagsStore.reorderTagGroups(groupIds)
  } catch (e) {
    console.error('Failed to reorder groups:', e)
    await tagsStore.loadTagGroups()
  }
}

// Tag handlers
function handleAddTag(groupId: number) {
  targetGroupId.value = groupId
  showCreateTag.value = true
}

async function handleCreateTag(groupId: number, value: string) {
  try {
    await tagsStore.createTag(groupId, value)
    await tagsStore.loadUsageCounts()
  } catch (e) {
    console.error('Failed to create tag:', e)
  }
}

async function handleTagSearch(query: string) {
  try {
    const results = await tagsStore.searchTags(query)
    createTagDialog.value?.updateSuggestions(results)
  } catch (e) {
    console.error('Failed to search tags:', e)
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
  const count = usageCounts.value[tag.id] || 0
  confirmDescription.value = count > 0
    ? `This tag is used by ${count} file${count === 1 ? '' : 's'}. It will be removed from all files.`
    : ''
  pendingConfirmAction.value = async () => {
    await tagsStore.deleteTag(tag.id)
  }
  showConfirm.value = true
}

function handleDeleteTagConfirm(id: number) {
  const tag = tags.value.find(t => t.id === id)
  if (tag) {
    handleDeleteTag(tag)
    showEditTag.value = false
    editingTag.value = null
  }
}

// Move tag
function handleMoveTag(tag: Tag) {
  moveTagIds.value = [tag.id]
  showMoveDialog.value = true
}

// Merge tag
function handleMergeTag(tag: Tag) {
  mergingTag.value = tag
  showMergeDialog.value = true
}

// Batch operations
function handleBatchMove(tagIds: number[]) {
  moveTagIds.value = tagIds
  showMoveDialog.value = true
}

async function executeMoveToGroup(targetGroupId: number) {
  try {
    for (const id of moveTagIds.value) {
      await tagsStore.updateTag(id, undefined, targetGroupId)
    }
    await tagsStore.loadTags()
  } catch (e) {
    console.error('Failed to move tags:', e)
  }
}

async function executeMerge(targetId: number) {
  if (!mergingTag.value) return
  try {
    await tagsStore.mergeTags(mergingTag.value.id, targetId)
    mergingTag.value = null
  } catch (e) {
    console.error('Failed to merge tags:', e)
  }
}

function handleBatchDelete(tagIds: number[]) {
  const count = tagIds.length
  confirmTitle.value = 'Delete Tags'
  confirmMessage.value = `Are you sure you want to delete ${count} tag${count === 1 ? '' : 's'}?`
  confirmDescription.value = 'Tags will be removed from all associated files.'
  pendingConfirmAction.value = async () => {
    for (const id of tagIds) {
      await tagsStore.deleteTag(id)
    }
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
.tag-management-expanded {
  height: 100%;
  display: flex;
  flex-direction: column;
  background: var(--surface);
}

.expanded-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 20px;
  border-bottom: 1px solid var(--border-color);
  background: var(--background);
  flex-shrink: 0;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 24px;
}

.header-title {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
}

.tab-strip {
  display: flex;
  gap: 0;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  overflow: hidden;
}

.tab-btn {
  padding: 6px 16px;
  border: none;
  background: var(--background);
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: var(--transition-fast);
  color: var(--text-secondary);
}

.tab-btn:not(:last-child) {
  border-right: 1px solid var(--border-color);
}

.tab-btn:hover {
  background: var(--surface);
}

.tab-btn.active {
  background: var(--primary-color);
  color: white;
}

.header-actions {
  display: flex;
  align-items: center;
  gap: 8px;
}

.dual-panel {
  flex: 1;
  display: flex;
  overflow: hidden;
}

.panel-left {
  flex-shrink: 0;
  border-right: 1px solid var(--border-color);
  overflow: hidden;
}

.panel-resizer {
  width: 4px;
  cursor: col-resize;
  background: transparent;
  transition: background 0.2s;
  flex-shrink: 0;
}

.panel-resizer:hover,
.panel-resizer:active {
  background: var(--primary-color);
}

.panel-right {
  flex: 1;
  overflow: hidden;
}

.templates-panel {
  flex: 1;
  overflow: auto;
}
</style>
