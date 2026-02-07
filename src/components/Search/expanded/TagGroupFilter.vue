<template>
  <div class="tag-group-filter">
    <h4 class="section-title">Tag Groups</h4>

    <div v-if="tagGroups.length === 0" class="empty-hint">
      No tag groups available
    </div>

    <div v-else class="groups-container">
      <div v-for="group in tagGroups" :key="group.id" class="filter-group">
        <div class="group-header">
          <span
            class="group-color"
            :style="{ backgroundColor: group.color || '#9e9e9e' }"
          ></span>
          <span class="group-name">{{ group.name }}</span>
        </div>

        <div class="tag-chips">
          <label
            v-for="tag in getVisibleTags(group.id)"
            :key="tag.id"
            class="tag-chip"
            :class="{ checked: isSelected(tag.id) }"
          >
            <input
              type="checkbox"
              :checked="isSelected(tag.id)"
              @change="emit('toggleTag', tag.id)"
            />
            <span class="chip-label">{{ tag.value }}</span>
          </label>

          <button
            v-if="hasMoreTags(group.id)"
            class="more-btn"
            @click="toggleExpand(group.id)"
          >
            {{ isExpanded(group.id)
              ? 'Show less'
              : `+${getRemainingCount(group.id)} more`
            }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import type { Tag, TagGroup } from '@/types'

interface Props {
  tagGroups: TagGroup[]
  tags: Tag[]
  selectedTagIds: number[]
  usageCounts: Record<number, number>
}

const props = defineProps<Props>()

const emit = defineEmits<{
  toggleTag: [tagId: number]
}>()

const MAX_VISIBLE = 5
const expandedGroups = ref<Set<number>>(new Set())

function getTagsByGroup(groupId: number): Tag[] {
  return props.tags
    .filter(t => t.group_id === groupId)
    .sort((a, b) => {
      const countA = props.usageCounts[a.id] || 0
      const countB = props.usageCounts[b.id] || 0
      return countB - countA
    })
}

function getVisibleTags(groupId: number): Tag[] {
  const tags = getTagsByGroup(groupId)
  if (expandedGroups.value.has(groupId)) return tags
  return tags.slice(0, MAX_VISIBLE)
}

function hasMoreTags(groupId: number): boolean {
  return getTagsByGroup(groupId).length > MAX_VISIBLE
}

function getRemainingCount(groupId: number): number {
  return getTagsByGroup(groupId).length - MAX_VISIBLE
}

function isExpanded(groupId: number): boolean {
  return expandedGroups.value.has(groupId)
}

function toggleExpand(groupId: number) {
  if (expandedGroups.value.has(groupId)) {
    expandedGroups.value.delete(groupId)
  } else {
    expandedGroups.value.add(groupId)
  }
}

function isSelected(tagId: number): boolean {
  return props.selectedTagIds.includes(tagId)
}
</script>

<style scoped>
.tag-group-filter {
  flex: 1;
  min-width: 0;
}

.section-title {
  margin: 0 0 12px 0;
  font-size: 13px;
  font-weight: 600;
  color: var(--text-primary);
}

.empty-hint {
  font-size: 12px;
  color: var(--text-secondary);
  font-style: italic;
}

.groups-container {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.filter-group {
  /* Individual group */
}

.group-header {
  display: flex;
  align-items: center;
  gap: 6px;
  margin-bottom: 6px;
}

.group-color {
  width: 10px;
  height: 10px;
  border-radius: 2px;
}

.group-name {
  font-size: 12px;
  font-weight: 500;
  color: var(--text-secondary);
}

.tag-chips {
  display: flex;
  flex-wrap: wrap;
  gap: 4px;
}

.tag-chip {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 4px 10px;
  border-radius: 4px;
  font-size: 12px;
  cursor: pointer;
  background: var(--background);
  border: 1px solid var(--border-color);
  transition: var(--transition-fast);
}

.tag-chip:hover {
  border-color: var(--primary-color);
}

.tag-chip.checked {
  background: rgba(25, 118, 210, 0.1);
  border-color: var(--primary-color);
  color: var(--primary-color);
}

.tag-chip input {
  display: none;
}

.chip-label {
  white-space: nowrap;
}

.more-btn {
  padding: 4px 10px;
  border: none;
  background: transparent;
  font-size: 12px;
  color: var(--primary-color);
  cursor: pointer;
  border-radius: 4px;
}

.more-btn:hover {
  background: rgba(25, 118, 210, 0.06);
}
</style>
