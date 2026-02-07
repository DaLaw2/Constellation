<template>
  <div class="tag-groups-list">
    <draggable
      v-model="localGroups"
      item-key="id"
      handle=".drag-handle"
      @start="emit('dragStart')"
      @end="handleReorder"
      :animation="200"
      ghost-class="ghost"
      chosen-class="chosen"
      drag-class="dragging"
    >
      <template #item="{ element: group }">
        <div class="tag-group-item">
          <!-- Group Header -->
          <div
            class="tag-group-header"
            @click="toggleGroup(group.id)"
            @contextmenu.prevent="handleContextMenu($event, group)"
          >
            <div class="group-info">
              <span class="drag-handle" title="Drag to reorder" @click.stop>
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  width="16"
                  height="16"
                  viewBox="0 0 24 24"
                  fill="none"
                  stroke="currentColor"
                  stroke-width="2"
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  class="icon-drag"
                >
                  <circle cx="9" cy="12" r="1" />
                  <circle cx="9" cy="5" r="1" />
                  <circle cx="9" cy="19" r="1" />
                  <circle cx="15" cy="12" r="1" />
                  <circle cx="15" cy="5" r="1" />
                  <circle cx="15" cy="19" r="1" />
                </svg>
              </span>
              <span
                class="group-color-badge"
                :style="{ backgroundColor: group.color || '#9e9e9e' }"
              ></span>
              <span class="group-name">{{ group.name }}</span>
            </div>
            <div class="group-actions">
              <button class="btn-icon toggle-btn">
                <svg
                  v-if="expandedGroups.has(group.id)"
                  xmlns="http://www.w3.org/2000/svg"
                  width="16"
                  height="16"
                  viewBox="0 0 24 24"
                  fill="none"
                  stroke="currentColor"
                  stroke-width="2"
                  stroke-linecap="round"
                  stroke-linejoin="round"
                >
                  <polyline points="6 9 12 15 18 9" />
                </svg>
                <svg
                  v-else
                  xmlns="http://www.w3.org/2000/svg"
                  width="16"
                  height="16"
                  viewBox="0 0 24 24"
                  fill="none"
                  stroke="currentColor"
                  stroke-width="2"
                  stroke-linecap="round"
                  stroke-linejoin="round"
                >
                  <polyline points="9 18 15 12 9 6" />
                </svg>
              </button>
            </div>
          </div>

          <!-- Tag List (expanded) -->
          <div v-if="expandedGroups.has(group.id)" class="tag-list">
            <div
              v-for="tag in getTagsByGroup(group.id)"
              :key="tag.id"
              class="tag-item"
              @contextmenu.prevent="handleTagContextMenu($event, tag)"
            >
              <span class="tag-value">{{ tag.value }}</span>
              <div class="tag-actions">
                <span
                  class="tag-count-badge"
                  :class="{ 'tag-count-zero': !usageCounts[tag.id] }"
                  :title="getUsageTooltip(tag.id)"
                >
                  {{ usageCounts[tag.id] || 0 }}
                </span>
              </div>
            </div>
            <button class="btn-add-tag" @click="emit('addTag', group.id)">
              + Add Tag
            </button>
          </div>
        </div>
      </template>
    </draggable>

    <!-- Context Menu -->
    <ContextMenu
      :visible="contextMenu.visible"
      :x="contextMenu.x"
      :y="contextMenu.y"
      :items="contextMenuItems"
      @update:visible="contextMenu.visible = $event"
      @select="handleMenuSelect"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import draggable from 'vuedraggable'
import { ContextMenu } from '@/components/base'
import type { ContextMenuItem } from '@/components/base'
import type { Tag, TagGroup } from '@/types'

interface TagGroupListProps {
  groups: TagGroup[]
  tags: Tag[]
  usageCounts: Record<number, number>
}

const props = defineProps<TagGroupListProps>()

const emit = defineEmits<{
  reorder: [groupIds: number[]]
  dragStart: []
  editGroup: [group: TagGroup]
  deleteGroup: [group: TagGroup]
  editTag: [tag: Tag]
  deleteTag: [tag: Tag]
  addTag: [groupId: number]
}>()

const localGroups = ref([...props.groups])
const expandedGroups = ref<Set<number>>(new Set())

watch(() => props.groups, (newGroups) => {
  localGroups.value = [...newGroups]
}, { deep: true })

const contextMenu = ref<{
  visible: boolean
  x: number
  y: number
  target: TagGroup | Tag | null
  type: 'group' | 'tag'
}>({
  visible: false,
  x: 0,
  y: 0,
  target: null,
  type: 'group',
})

const contextMenuItems = computed((): ContextMenuItem[] => {
  if (contextMenu.value.type === 'group') {
    return [
      { label: 'Edit Group', icon: '✏️', action: handleContextEditGroup },
      { divider: true },
      {
        label: 'Delete Group',
        icon: '🗑️',
        danger: true,
        action: handleContextDeleteGroup,
      },
    ]
  } else {
    return [
      { label: 'Edit Tag', icon: '✏️', action: handleContextEditTag },
      { divider: true },
      {
        label: 'Delete Tag',
        icon: '🗑️',
        danger: true,
        action: handleContextDeleteTag,
      },
    ]
  }
})

function getTagsByGroup(groupId: number): Tag[] {
  return props.tags.filter(t => t.group_id === groupId)
}

function toggleGroup(groupId: number) {
  if (expandedGroups.value.has(groupId)) {
    expandedGroups.value.delete(groupId)
  } else {
    expandedGroups.value.add(groupId)
  }
}

function handleReorder() {
  const orderedIds = localGroups.value.map(g => g.id)
  emit('reorder', orderedIds)
}

function handleContextMenu(event: MouseEvent, group: TagGroup) {
  contextMenu.value = {
    visible: true,
    x: event.clientX,
    y: event.clientY,
    target: group,
    type: 'group',
  }
}

function handleTagContextMenu(event: MouseEvent, tag: Tag) {
  contextMenu.value = {
    visible: true,
    x: event.clientX,
    y: event.clientY,
    target: tag,
    type: 'tag',
  }
}

function handleMenuSelect() {
  contextMenu.value.visible = false
}

function handleContextEditGroup() {
  if (contextMenu.value.target && contextMenu.value.type === 'group') {
    emit('editGroup', contextMenu.value.target as TagGroup)
  }
}

function handleContextDeleteGroup() {
  if (contextMenu.value.target && contextMenu.value.type === 'group') {
    emit('deleteGroup', contextMenu.value.target as TagGroup)
  }
}

function handleContextEditTag() {
  if (contextMenu.value.target && contextMenu.value.type === 'tag') {
    emit('editTag', contextMenu.value.target as Tag)
  }
}

function handleContextDeleteTag() {
  if (contextMenu.value.target && contextMenu.value.type === 'tag') {
    emit('deleteTag', contextMenu.value.target as Tag)
  }
}

function getUsageTooltip(tagId: number): string {
  const count = props.usageCounts[tagId] || 0
  return count === 1 ? '1 file' : `${count} files`
}
</script>

<style scoped>
.tag-groups-list {
  overflow-y: auto;
  padding: 12px;
}

.tag-group-item {
  margin-bottom: 8px;
  background: var(--background);
  border: 1px solid var(--border-color);
  border-radius: 6px;
  overflow: hidden;
}

.tag-group-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px;
  cursor: pointer;
  transition: var(--transition-fast);
  user-select: none;
}

.tag-group-header:hover {
  background: var(--surface);
}

.group-info {
  display: flex;
  align-items: center;
  gap: 10px;
  flex: 1;
}

.drag-handle {
  cursor: grab;
  color: var(--text-secondary);
  display: flex;
  align-items: center;
}

.drag-handle:active {
  cursor: grabbing;
}

.icon-drag {
  width: 16px;
  height: 16px;
}

.group-color-badge {
  width: 20px;
  height: 20px;
  border-radius: 4px;
  flex-shrink: 0;
}

.group-name {
  font-size: 14px;
  font-weight: 500;
  color: var(--text-primary);
}

.group-actions {
  display: flex;
  align-items: center;
  gap: 4px;
}

.btn-icon {
  padding: 4px;
  background: transparent;
  border: none;
  cursor: pointer;
  border-radius: 4px;
  color: var(--text-secondary);
  transition: var(--transition-fast);
  display: flex;
  align-items: center;
  justify-content: center;
}

.btn-icon:hover {
  background: var(--secondary-color);
  color: var(--text-primary);
}

.tag-list {
  border-top: 1px solid var(--border-color);
  padding: 8px 12px;
  background: var(--surface);
}

.tag-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 8px 10px;
  margin-bottom: 4px;
  background: var(--background);
  border-radius: 4px;
  cursor: pointer;
  transition: var(--transition-fast);
}

.tag-item:hover {
  background: var(--secondary-color);
}

.tag-value {
  font-size: 13px;
  color: var(--text-primary);
}

.tag-actions {
  display: flex;
  align-items: center;
  gap: 8px;
}

.tag-count-badge {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  min-width: 24px;
  height: 20px;
  padding: 0 6px;
  background: var(--primary-color);
  color: white;
  font-size: 11px;
  font-weight: 500;
  border-radius: 10px;
}

.tag-count-zero {
  background: var(--border-color);
  color: var(--text-secondary);
}

.btn-add-tag {
  width: 100%;
  padding: 8px;
  margin-top: 8px;
  background: transparent;
  border: 1px dashed var(--border-color);
  border-radius: 4px;
  color: var(--text-secondary);
  font-size: 13px;
  cursor: pointer;
  transition: var(--transition-fast);
}

.btn-add-tag:hover {
  background: var(--background);
  border-color: var(--primary-color);
  color: var(--primary-color);
}

/* Draggable styles */
.ghost {
  opacity: 0.4;
}

.chosen {
  opacity: 0.8;
}

.dragging {
  opacity: 0.5;
  transform: rotate(2deg);
}
</style>
