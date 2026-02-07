<template>
  <div class="group-list-panel">
    <draggable
      v-model="localGroups"
      item-key="id"
      handle=".drag-handle"
      @start="dragging = true"
      @end="handleReorder"
      :animation="200"
      ghost-class="ghost"
      chosen-class="chosen"
      drag-class="dragging"
    >
      <template #item="{ element: group }">
        <div class="group-item" :class="{ selected: selectedGroupId === group.id }">
          <div
            class="group-item-header"
            @click="emit('selectGroup', group.id)"
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
            <span class="group-tag-count">({{ getTagCount(group.id) }})</span>
          </div>
        </div>
      </template>
    </draggable>

    <div v-if="groups.length === 0" class="empty-state">
      No tag groups yet
    </div>

    <ContextMenu
      :visible="contextMenu.visible"
      :x="contextMenu.x"
      :y="contextMenu.y"
      :items="contextMenuItems"
      @update:visible="contextMenu.visible = $event"
      @select="contextMenu.visible = false"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import draggable from 'vuedraggable'
import { ContextMenu } from '@/components/base'
import type { ContextMenuItem } from '@/components/base'
import type { Tag, TagGroup } from '@/types'

interface Props {
  groups: TagGroup[]
  tags: Tag[]
  selectedGroupId: number | null
}

const props = defineProps<Props>()

const emit = defineEmits<{
  selectGroup: [groupId: number]
  reorder: [groupIds: number[]]
  editGroup: [group: TagGroup]
  deleteGroup: [group: TagGroup]
}>()

const localGroups = ref([...props.groups])
const dragging = ref(false)

watch(() => props.groups, (newGroups) => {
  localGroups.value = [...newGroups]
}, { deep: true })

const contextMenu = ref({
  visible: false,
  x: 0,
  y: 0,
  target: null as TagGroup | null,
})

const contextMenuItems = computed((): ContextMenuItem[] => [
  { label: 'Edit Group', icon: '✏️', action: () => {
    if (contextMenu.value.target) emit('editGroup', contextMenu.value.target)
  }},
  { divider: true },
  { label: 'Delete Group', icon: '🗑️', danger: true, action: () => {
    if (contextMenu.value.target) emit('deleteGroup', contextMenu.value.target)
  }},
])

function getTagCount(groupId: number): number {
  return props.tags.filter(t => t.group_id === groupId).length
}

function handleReorder() {
  dragging.value = false
  const orderedIds = localGroups.value.map(g => g.id)
  emit('reorder', orderedIds)
}

function handleContextMenu(event: MouseEvent, group: TagGroup) {
  contextMenu.value = {
    visible: true,
    x: event.clientX,
    y: event.clientY,
    target: group,
  }
}
</script>

<style scoped>
.group-list-panel {
  flex: 1;
  min-height: 0;
  overflow-y: auto;
  padding: 12px;
}

.group-item {
  margin-bottom: 4px;
  border-radius: 6px;
  overflow: hidden;
}

.group-item.selected {
  background: rgba(25, 118, 210, 0.08);
  border-left: 3px solid var(--primary-color);
}

.group-item-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 10px 12px;
  cursor: pointer;
  transition: var(--transition-fast);
  user-select: none;
}

.group-item-header:hover {
  background: rgba(0, 0, 0, 0.04);
}

.group-info {
  display: flex;
  align-items: center;
  gap: 10px;
  flex: 1;
  min-width: 0;
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

.group-color-badge {
  width: 16px;
  height: 16px;
  border-radius: 3px;
  flex-shrink: 0;
}

.group-name {
  font-size: 14px;
  font-weight: 500;
  color: var(--text-primary);
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.group-tag-count {
  font-size: 12px;
  color: var(--text-secondary);
  flex-shrink: 0;
}

.empty-state {
  padding: 24px;
  text-align: center;
  font-size: 13px;
  color: var(--text-secondary);
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
