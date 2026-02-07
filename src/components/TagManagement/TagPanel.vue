<template>
  <div class="tag-panel">
    <div class="panel-header">
      <h3>Tag Groups</h3>
      <BaseButton variant="primary" @click="showCreateGroup = true">
        + New Group
      </BaseButton>
    </div>

    <div v-if="loading" class="loading-state">Loading...</div>

    <div v-else-if="error" class="error-state">Error: {{ error }}</div>

    <div v-else-if="tagGroups.length === 0" class="empty-state">
      <div class="empty-state-icon">🏷️</div>
      <div class="empty-state-title">No Tag Groups</div>
      <div class="empty-state-description">
        Create your first tag group to start organizing files
      </div>
    </div>

    <TagGroupList
      v-else
      :groups="tagGroups"
      :tags="tags"
      :usage-counts="usageCounts"
      @reorder="handleReorder"
      @edit-group="handleEditGroup"
      @delete-group="handleDeleteGroup"
      @edit-tag="handleEditTag"
      @delete-tag="handleDeleteTag"
      @add-tag="handleAddTag"
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
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useTagsStore } from '@/stores/tags'
import { BaseButton, ConfirmDialog } from '@/components/base'
import TagGroupList from './TagGroupList.vue'
import {
  CreateGroupDialog,
  EditGroupDialog,
  CreateTagDialog,
  EditTagDialog,
} from './dialogs'
import type { Tag, TagGroup } from '@/types'

const tagsStore = useTagsStore()

const tagGroups = computed(() => tagsStore.tagGroups)
const tags = computed(() => tagsStore.tags)
const loading = computed(() => tagsStore.loading)
const error = computed(() => tagsStore.error)
const usageCounts = computed(() => tagsStore.usageCounts)

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

// Group Actions
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

// Tag Actions
function handleAddTag(groupId: number) {
  targetGroupId.value = groupId
  showCreateTag.value = true
}

async function handleCreateTag(groupId: number, value: string) {
  try {
    await tagsStore.createTag(groupId, value)
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
  confirmDescription.value = ''
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
.tag-panel {
  height: 100%;
  display: flex;
  flex-direction: column;
  background: var(--surface);
}

.panel-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px 20px;
  border-bottom: 1px solid var(--border-color);
  background: var(--background);
  flex-shrink: 0;
}

.panel-header h3 {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
}

.loading-state,
.error-state,
.empty-state {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  color: var(--text-secondary);
  padding: 2rem;
}

.empty-state-icon {
  font-size: 48px;
  margin-bottom: 16px;
  opacity: 0.5;
}

.empty-state-title {
  font-size: 16px;
  font-weight: 600;
  margin-bottom: 8px;
  color: var(--text-primary);
}

.empty-state-description {
  font-size: 14px;
  text-align: center;
  max-width: 300px;
}

.error-state {
  color: #dc3545;
}
</style>
