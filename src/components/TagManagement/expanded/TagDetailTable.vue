<template>
  <div class="tag-detail-table">
    <!-- Empty state: no group selected -->
    <div v-if="!groupId" class="empty-state">
      <div class="empty-icon">🏷️</div>
      <div class="empty-title">Select a Tag Group</div>
      <div class="empty-description">Choose a group from the left panel to view and manage its tags</div>
    </div>

    <!-- Table content -->
    <template v-else>
      <!-- Header -->
      <div class="table-header-bar">
        <h3 class="table-title">
          Tags in "{{ selectedGroupName }}"
        </h3>
        <BaseButton variant="primary" size="small" @click="emit('addTag', groupId!)">
          + Add Tag
        </BaseButton>
      </div>

      <!-- Batch action bar -->
      <div v-if="selectedTagIds.size > 0" class="batch-bar">
        <span class="batch-count">{{ selectedTagIds.size }} selected</span>
        <BaseButton variant="text" size="small" @click="emit('batchMove', [...selectedTagIds])">
          Move to Group
        </BaseButton>
        <BaseButton variant="danger" size="small" @click="emit('batchDelete', [...selectedTagIds])">
          Delete Selected
        </BaseButton>
        <BaseButton variant="text" size="small" @click="clearSelection">
          Cancel
        </BaseButton>
      </div>

      <!-- Table -->
      <div class="table-container">
        <table class="tag-table">
          <thead>
            <tr>
              <th class="col-checkbox">
                <input
                  type="checkbox"
                  :checked="isAllSelected"
                  :indeterminate="isPartiallySelected"
                  @change="toggleSelectAll"
                />
              </th>
              <th class="col-name">Tag Name</th>
              <th class="col-usage">Usage</th>
              <th class="col-actions">Actions</th>
            </tr>
          </thead>
          <tbody>
            <tr v-if="filteredTags.length === 0">
              <td colspan="4" class="no-tags">
                No tags in this group. Click "+ Add Tag" to create one.
              </td>
            </tr>
            <tr
              v-for="tag in filteredTags"
              :key="tag.id"
              :class="{ 'row-selected': selectedTagIds.has(tag.id) }"
            >
              <td class="col-checkbox">
                <input
                  type="checkbox"
                  :checked="selectedTagIds.has(tag.id)"
                  @change="toggleTag(tag.id)"
                />
              </td>
              <td class="col-name">
                <span class="tag-value">{{ tag.value }}</span>
              </td>
              <td class="col-usage">
                <span
                  class="usage-badge"
                  :class="{ 'usage-zero': !usageCounts[tag.id] }"
                >
                  {{ usageCounts[tag.id] || 0 }}
                </span>
              </td>
              <td class="col-actions">
                <button
                  class="btn-actions"
                  @click.stop="openActionMenu($event, tag)"
                  title="Actions"
                >
                  ⋮
                </button>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </template>

    <!-- Action context menu -->
    <ContextMenu
      :visible="actionMenu.visible"
      :x="actionMenu.x"
      :y="actionMenu.y"
      :items="actionMenuItems"
      @update:visible="actionMenu.visible = $event"
      @select="actionMenu.visible = false"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { BaseButton, ContextMenu } from '@/components/base'
import type { ContextMenuItem } from '@/components/base'
import type { Tag, TagGroup } from '@/types'

interface Props {
  groupId: number | null
  tags: Tag[]
  usageCounts: Record<number, number>
  groups: TagGroup[]
}

const props = defineProps<Props>()

const emit = defineEmits<{
  addTag: [groupId: number]
  editTag: [tag: Tag]
  deleteTag: [tag: Tag]
  moveTag: [tag: Tag]
  mergeTag: [tag: Tag]
  batchMove: [tagIds: number[]]
  batchDelete: [tagIds: number[]]
}>()

const selectedTagIds = ref<Set<number>>(new Set())

const actionMenu = ref({
  visible: false,
  x: 0,
  y: 0,
  target: null as Tag | null,
})

const filteredTags = computed(() => {
  if (!props.groupId) return []
  return props.tags.filter(t => t.group_id === props.groupId)
})

const selectedGroupName = computed(() => {
  const group = props.groups.find(g => g.id === props.groupId)
  return group?.name || ''
})

const isAllSelected = computed(() => {
  return filteredTags.value.length > 0 && filteredTags.value.every(t => selectedTagIds.value.has(t.id))
})

const isPartiallySelected = computed(() => {
  const count = filteredTags.value.filter(t => selectedTagIds.value.has(t.id)).length
  return count > 0 && count < filteredTags.value.length
})

const actionMenuItems = computed((): ContextMenuItem[] => [
  { label: 'Edit', icon: '✏️', action: () => {
    if (actionMenu.value.target) emit('editTag', actionMenu.value.target)
  }},
  { label: 'Move to Group', icon: '📂', action: () => {
    if (actionMenu.value.target) emit('moveTag', actionMenu.value.target)
  }},
  { label: 'Merge with...', icon: '🔗', action: () => {
    if (actionMenu.value.target) emit('mergeTag', actionMenu.value.target)
  }},
  { divider: true },
  { label: 'Delete', icon: '🗑️', danger: true, action: () => {
    if (actionMenu.value.target) emit('deleteTag', actionMenu.value.target)
  }},
])

function toggleTag(tagId: number) {
  if (selectedTagIds.value.has(tagId)) {
    selectedTagIds.value.delete(tagId)
  } else {
    selectedTagIds.value.add(tagId)
  }
  // Force reactivity
  selectedTagIds.value = new Set(selectedTagIds.value)
}

function toggleSelectAll() {
  if (isAllSelected.value) {
    selectedTagIds.value = new Set()
  } else {
    selectedTagIds.value = new Set(filteredTags.value.map(t => t.id))
  }
}

function clearSelection() {
  selectedTagIds.value = new Set()
}

function openActionMenu(event: MouseEvent, tag: Tag) {
  actionMenu.value = {
    visible: true,
    x: event.clientX,
    y: event.clientY,
    target: tag,
  }
}
</script>

<style scoped>
.tag-detail-table {
  height: 100%;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.empty-state {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  color: var(--text-secondary);
}

.empty-icon {
  font-size: 48px;
  margin-bottom: 16px;
  opacity: 0.4;
}

.empty-title {
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: 8px;
}

.empty-description {
  font-size: 13px;
  max-width: 300px;
  text-align: center;
}

.table-header-bar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px 20px;
  border-bottom: 1px solid var(--border-color);
  flex-shrink: 0;
}

.table-title {
  margin: 0;
  font-size: 15px;
  font-weight: 600;
  color: var(--text-primary);
}

.batch-bar {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 20px;
  background: rgba(25, 118, 210, 0.06);
  border-bottom: 1px solid var(--border-color);
  flex-shrink: 0;
}

.batch-count {
  font-size: 13px;
  font-weight: 500;
  color: var(--primary-color);
  margin-right: 8px;
}

.table-container {
  flex: 1;
  overflow-y: auto;
}

.tag-table {
  width: 100%;
  border-collapse: collapse;
}

.tag-table thead {
  position: sticky;
  top: 0;
  z-index: 1;
  background: var(--background);
}

.tag-table th {
  padding: 10px 16px;
  text-align: left;
  font-size: 12px;
  font-weight: 600;
  color: var(--text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.5px;
  border-bottom: 2px solid var(--border-color);
}

.tag-table td {
  padding: 10px 16px;
  font-size: 14px;
  color: var(--text-primary);
  border-bottom: 1px solid var(--border-color);
}

.tag-table tr:hover td {
  background: rgba(0, 0, 0, 0.02);
}

.row-selected td {
  background: rgba(25, 118, 210, 0.04);
}

.col-checkbox {
  width: 40px;
  text-align: center;
}

.col-checkbox input[type="checkbox"] {
  cursor: pointer;
}

.col-name {
  min-width: 150px;
}

.tag-value {
  font-weight: 500;
}

.col-usage {
  width: 80px;
  text-align: center;
}

.usage-badge {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  min-width: 28px;
  height: 22px;
  padding: 0 8px;
  background: var(--primary-color);
  color: white;
  font-size: 12px;
  font-weight: 500;
  border-radius: 11px;
}

.usage-zero {
  background: var(--border-color);
  color: var(--text-secondary);
}

.col-actions {
  width: 60px;
  text-align: center;
}

.btn-actions {
  padding: 4px 8px;
  background: transparent;
  border: none;
  border-radius: 4px;
  font-size: 18px;
  cursor: pointer;
  color: var(--text-secondary);
  transition: var(--transition-fast);
  line-height: 1;
}

.btn-actions:hover {
  background: var(--secondary-color);
  color: var(--text-primary);
}

.no-tags {
  text-align: center;
  color: var(--text-secondary);
  font-size: 13px;
  padding: 32px 16px !important;
}
</style>
