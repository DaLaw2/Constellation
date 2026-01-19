<template>
  <div class="tag-panel">
    <div class="panel-header">
      <h3>Tag Groups</h3>
      <button class="btn btn-primary" @click="showCreateGroupDialog = true">
        + New Group
      </button>
    </div>

    <div v-if="loading" class="loading-state">
      Loading...
    </div>

    <div v-else-if="error" class="error-state">
      Error: {{ error }}
    </div>

    <div v-else-if="tagGroups.length === 0" class="empty-state">
      <div class="empty-state-icon">🏷️</div>
      <div class="empty-state-title">No Tag Groups</div>
      <div class="empty-state-description">
        Create your first tag group to start organizing files
      </div>
    </div>

    <div v-else class="tag-groups-list">
      <div v-for="group in tagGroups" :key="group.id" class="tag-group-item">
        <div class="tag-group-header">
          <div class="group-info">
            <span
              class="group-color-badge"
              :style="{ backgroundColor: group.color || '#9e9e9e' }"
            ></span>
            <span class="group-name">{{ group.name }}</span>
          </div>
          <button class="btn-icon" @click="toggleGroup(group.id)">
            {{ expandedGroups.has(group.id) ? '▼' : '▶' }}
          </button>
        </div>

        <div v-if="expandedGroups.has(group.id)" class="tag-list">
          <div v-for="tag in getTagsByGroup(group.id)" :key="tag.id" class="tag-item">
            <span class="tag-value">{{ tag.value }}</span>
          </div>
          <button class="btn-add-tag" @click="showAddTagDialog(group.id)">
            + Add Tag
          </button>
        </div>
      </div>
    </div>

    <!-- Simple create group dialog -->
    <div v-if="showCreateGroupDialog" class="dialog-overlay" @click.self="showCreateGroupDialog = false">
      <div class="dialog">
        <h3>Create Tag Group</h3>
        <div class="form-group">
          <label>Name:</label>
          <input type="text" v-model="newGroupName" placeholder="e.g., Language" />
        </div>
        <div class="form-group">
          <label>Color:</label>
          <input type="color" v-model="newGroupColor" />
        </div>
        <div class="dialog-actions">
          <button class="btn" @click="showCreateGroupDialog = false">Cancel</button>
          <button class="btn btn-primary" @click="createGroup">Create</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useTagsStore } from '../../stores/tags'

const tagsStore = useTagsStore()

const tagGroups = computed(() => tagsStore.tagGroups)
const loading = computed(() => tagsStore.loading)
const error = computed(() => tagsStore.error)

const expandedGroups = ref<Set<number>>(new Set())
const showCreateGroupDialog = ref(false)
const newGroupName = ref('')
const newGroupColor = ref('#3B82F6')

onMounted(() => {
  tagsStore.loadTagGroups()
  tagsStore.loadTags()
})

function toggleGroup(groupId: number) {
  if (expandedGroups.value.has(groupId)) {
    expandedGroups.value.delete(groupId)
  } else {
    expandedGroups.value.add(groupId)
  }
}

function getTagsByGroup(groupId: number) {
  return tagsStore.getTagsByGroup(groupId)
}

async function createGroup() {
  if (!newGroupName.value.trim()) return

  try {
    await tagsStore.createTagGroup(newGroupName.value.trim(), newGroupColor.value)
    newGroupName.value = ''
    newGroupColor.value = '#3B82F6'
    showCreateGroupDialog.value = false
  } catch (e) {
    console.error('Failed to create group:', e)
  }
}

function showAddTagDialog(groupId: number) {
  const value = prompt('Enter tag value:')
  if (value) {
    tagsStore.createTag(groupId, value.trim())
  }
}
</script>

<style scoped>
.tag-panel {
  height: 100%;
  display: flex;
  flex-direction: column;
}

.panel-header {
  padding: 1rem;
  border-bottom: 1px solid var(--border-color);
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.panel-header h3 {
  font-size: 14px;
  font-weight: 600;
}

.loading-state,
.error-state {
  padding: 2rem;
  text-align: center;
  color: var(--text-secondary);
}

.error-state {
  color: #d32f2f;
}

.tag-groups-list {
  flex: 1;
  overflow: auto;
  padding: 0.5rem;
}

.tag-group-item {
  margin-bottom: 0.5rem;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background: var(--background);
}

.tag-group-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0.75rem;
  cursor: pointer;
}

.tag-group-header:hover {
  background: var(--surface);
}

.group-info {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.group-color-badge {
  width: 12px;
  height: 12px;
  border-radius: 2px;
}

.group-name {
  font-weight: 500;
  font-size: 13px;
}

.tag-list {
  padding: 0.5rem;
  border-top: 1px solid var(--border-color);
  background: var(--surface);
}

.tag-item {
  padding: 0.5rem;
  margin-bottom: 0.25rem;
  background: var(--background);
  border-radius: 4px;
  font-size: 12px;
}

.tag-value {
  color: var(--text-primary);
}

.btn-add-tag {
  width: 100%;
  padding: 0.5rem;
  margin-top: 0.5rem;
  border: 1px dashed var(--border-color);
  background: transparent;
  border-radius: 4px;
  font-size: 12px;
  color: var(--text-secondary);
  cursor: pointer;
  transition: var(--transition-fast);
}

.btn-add-tag:hover {
  border-color: var(--primary-color);
  color: var(--primary-color);
  background: rgba(25, 118, 210, 0.04);
}

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
  padding: 1.5rem;
  width: 400px;
  max-width: 90vw;
  box-shadow: 0 10px 30px rgba(0, 0, 0, 0.3);
}

.dialog h3 {
  margin-bottom: 1rem;
  font-size: 18px;
}

.form-group {
  margin-bottom: 1rem;
}

.form-group label {
  display: block;
  margin-bottom: 0.5rem;
  font-size: 13px;
  font-weight: 500;
}

.form-group input {
  width: 100%;
  padding: 8px 12px;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  font-size: 14px;
}

.form-group input[type="color"] {
  height: 40px;
  padding: 4px;
}

.dialog-actions {
  display: flex;
  gap: 0.5rem;
  justify-content: flex-end;
  margin-top: 1.5rem;
}
</style>
