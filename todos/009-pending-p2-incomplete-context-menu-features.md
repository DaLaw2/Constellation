---
status: pending
priority: p2
issue_id: "009"
tags: [code-review, feature, agent-native, integration]
dependencies: []
---

# Incomplete Context Menu Features - Workflow Gap

## Problem Statement

The file browser context menu contains three menu items ("Add to Library", "Tag...", "Properties") that only log to console with TODO comments. More critically, this creates a **workflow integration gap** - users and agents cannot complete the primary application workflow: browse files → add to library → tag files.

**Why this matters:**
- **Breaks core user workflow** - cannot tag discovered files
- **Agent-native gap** - agents can browse but not organize
- **User confusion** - clickable menu items that don't work
- **Incomplete feature** - Phase 1.3 claims file browser is done, but it's disconnected from tagging system

## Findings

### Source: Agent-Native-Reviewer + Simplicity-Reviewer

**Affected Code** (`frontend/composables/useFileContextMenu.ts`):

**Issue 1: "Add to Library"** (Lines 49-55):
```typescript
{
  label: 'Add to Library',
  icon: '➕',
  onClick: () => {
    // TODO: Implement add to library
    console.log('Add to library:', entry.path)
  },
},
```

**Issue 2: "Tag..."** (Lines 56-63):
```typescript
{
  label: 'Tag...',
  icon: '🏷️',
  onClick: () => {
    // TODO: Implement tagging
    console.log('Tag file:', entry.path)
  },
},
```

**Issue 3: "Properties"** (Lines 68-79):
```typescript
{
  label: 'Properties',
  icon: 'ℹ️',
  onClick: async () => {
    try {
      const metadata = await fileExplorerStore.getFileMetadata(entry.path)
      console.log('File metadata:', metadata)
      // TODO: Show properties dialog
    } catch (e) {
      console.error('Failed to get metadata:', e)
    }
  },
},
```

### Evidence from Agent-Native Reviewer

**The "Write to Location" Test FAILED:**
- ✅ Agent can enumerate drives with `get_drives`
- ✅ Agent can read directories with `read_directory`
- ✅ Agent can filter file types
- ❌ **Agent CANNOT tag files** - no workflow to connect browsing to tagging
- ❌ **Agent CANNOT add files to library** - menu item is stub

**Critical Workflow Gap:**
```
User/Agent wants to: "Tag all PDFs in D:\Documents as work-related"

Current state:
1. ✅ get_drives → find D:\ drive
2. ✅ read_directory('D:\Documents') → get file list
3. ✅ Filter PDFs → entry.name.endsWith('.pdf')
4. ❌ ADD TO LIBRARY → no implementation
   - Need: create_item(path, is_directory, size, modified_time)
   - Command exists but not wired up
5. ❌ TAG FILES → no workflow
   - Need multi-step: get_item_by_path → create_item (if null) → add_tag_to_item
   - Commands exist but no integration
```

### Evidence from Simplicity-Reviewer

These TODO items are **YAGNI violations** and **technical debt**:
- Features that don't exist shouldn't be in the UI
- Creates false expectations for users
- Agents are confused about available capabilities
- Better to hide menu items until implemented

## Proposed Solutions

### Solution 1: Implement Core Workflow Integration (RECOMMENDED)

**Approach:** Wire up the TODO items to existing backend commands and create necessary UI components.

**Part A: Add to Library** (Highest Priority)

**Implementation:**
```typescript
// frontend/composables/useFileContextMenu.ts

{
  label: 'Add to Library',
  icon: '➕',
  onClick: async () => {
    try {
      // Check if already in library
      const existingItem = await invoke('get_item_by_path', { path: entry.path })

      if (existingItem) {
        // Show toast: "Already in library"
        console.info('Item already in library:', entry.path)
        return
      }

      // Create new item
      const itemId = await invoke('create_item', {
        path: entry.path,
        isDirectory: entry.is_directory,
        size: entry.size,
        modifiedTime: entry.modified_time,
      })

      // Show success toast
      console.info('Added to library:', entry.path, 'ID:', itemId)

      // TODO: Refresh library view if visible
    } catch (e) {
      console.error('Failed to add to library:', e)
      // Show error toast
    }
  },
},
```

**Part B: Tag Files** (High Priority)

**Implementation requires new UI component:**
```vue
<!-- frontend/components/TagManagement/TagSelectorDialog.vue (NEW) -->
<template>
  <dialog ref="dialogRef" class="tag-selector-dialog">
    <h2>Select Tags for: {{ fileName }}</h2>

    <div class="tag-list">
      <div v-for="group in tagGroups" :key="group.id" class="tag-group">
        <h3>{{ group.name }}</h3>
        <div class="tags">
          <label v-for="tag in getTagsByGroup(group.id)" :key="tag.id">
            <input
              type="checkbox"
              :value="tag.id"
              v-model="selectedTagIds"
            />
            {{ tag.value }}
          </label>
        </div>
      </div>
    </div>

    <div class="dialog-actions">
      <button @click="cancel">Cancel</button>
      <button @click="apply" class="primary">Apply Tags</button>
    </div>
  </dialog>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useTagsStore } from '../../stores/tags'
import { useTagGroupsStore } from '../../stores/tagGroups'

const emit = defineEmits<{
  apply: [tagIds: number[]]
  cancel: []
}>()

const props = defineProps<{
  fileName: string
  existingTagIds: number[]
}>()

const tagStore = useTagsStore()
const tagGroupStore = useTagGroupsStore()

const dialogRef = ref<HTMLDialogElement>()
const selectedTagIds = ref<number[]>([...props.existingTagIds])

const tagGroups = computed(() => tagGroupStore.tagGroups)

function getTagsByGroup(groupId: number) {
  return tagStore.tags.filter(t => t.group_id === groupId)
}

function apply() {
  emit('apply', selectedTagIds.value)
  dialogRef.value?.close()
}

function cancel() {
  emit('cancel')
  dialogRef.value?.close()
}

function open() {
  dialogRef.value?.showModal()
}

defineExpose({ open })
</script>
```

Then wire it up:
```typescript
// frontend/composables/useFileContextMenu.ts

import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

const tagSelectorDialog = ref<any>(null)  // Reference to dialog component

{
  label: 'Tag...',
  icon: '🏷️',
  onClick: async () => {
    try {
      // 1. Check if in library, create if not
      let item = await invoke('get_item_by_path', { path: entry.path })

      if (!item) {
        const itemId = await invoke('create_item', {
          path: entry.path,
          isDirectory: entry.is_directory,
          size: entry.size,
          modifiedTime: entry.modified_time,
        })
        item = { id: itemId, path: entry.path }
      }

      // 2. Get existing tags
      const existingTags = await invoke('get_tags_for_item', { itemId: item.id })

      // 3. Show tag selector dialog
      tagSelectorDialog.value?.open(entry.name, existingTags.map(t => t.id))

      // 4. On dialog.apply, update tags
      // (handled by dialog component emitting 'apply' event)
    } catch (e) {
      console.error('Failed to open tag selector:', e)
    }
  },
},
```

**Part C: Properties Dialog** (Lower Priority)

Create simple properties dialog:
```vue
<!-- frontend/components/FileExplorer/FilePropertiesDialog.vue (NEW) -->
<template>
  <dialog ref="dialogRef" class="properties-dialog">
    <h2>Properties: {{ metadata.path }}</h2>

    <table>
      <tr>
        <td>Type:</td>
        <td>{{ metadata.is_directory ? 'Folder' : 'File' }}</td>
      </tr>
      <tr v-if="metadata.size">
        <td>Size:</td>
        <td>{{ formatBytes(metadata.size) }}</td>
      </tr>
      <tr v-if="metadata.modified_time">
        <td>Modified:</td>
        <td>{{ new Date(metadata.modified_time * 1000).toLocaleString() }}</td>
      </tr>
      <tr v-if="metadata.created_time">
        <td>Created:</td>
        <td>{{ new Date(metadata.created_time * 1000).toLocaleString() }}</td>
      </tr>
      <tr>
        <td>Read-only:</td>
        <td>{{ metadata.is_readonly ? 'Yes' : 'No' }}</td>
      </tr>
    </table>

    <button @click="close">Close</button>
  </dialog>
</template>
```

**Pros:**
- ✅ Completes the user workflow
- ✅ Enables agent-native tagging
- ✅ Removes TODO comments (technical debt)
- ✅ Fulfills Phase 1.3 goals

**Cons:**
- ❌ Requires significant frontend work (2-3 new components)
- ❌ Tag selector dialog is complex (groups, checkboxes, state)
- ⚠️ May duplicate logic from Phase 1.5 (tagging UI)

**Effort:** 8-12 hours (3 components + integration + testing)
**Risk:** Medium (UI complexity, state management)

### Solution 2: Remove Unimplemented Menu Items (QUICK FIX)

**Approach:** Hide or disable menu items until they're actually implemented.

**Implementation:**
```typescript
const menuItems = [
  {
    label: 'Open',
    icon: '📂',
    onClick: () => { ... },
  },
  {
    label: 'Show in Explorer',
    icon: '📁',
    onClick: () => { ... },
  },
  // Remove these items entirely:
  // - Add to Library
  // - Tag...
  // - Properties (or show metadata in a simple way)
]
```

**Or disable them:**
```typescript
{
  label: 'Add to Library',
  icon: '➕',
  disabled: true,  // Grayed out
  onClick: () => {},
},
{
  label: 'Tag...',
  icon: '🏷️',
  disabled: true,
  onClick: () => {},
},
```

**Pros:**
- ✅ Quick fix (15 minutes)
- ✅ Honest about capabilities
- ✅ No user confusion

**Cons:**
- ❌ Doesn't solve the workflow gap
- ❌ Agents still can't tag files
- ❌ Core feature still missing

**Effort:** 15 minutes
**Risk:** Low (but doesn't fix root issue)

### Solution 3: Implement Add to Library Only, Defer Tagging

**Approach:** Quick win - wire up "Add to Library" (simple), defer complex tagging UI to Phase 1.5.

**Implementation:**
Same as Solution 1 Part A, but remove/disable "Tag..." and "Properties".

**Pros:**
- ✅ Partial workflow completion (browse → add to library)
- ✅ Agents can programmatically add files
- ✅ Reasonable scope for quick fix
- ✅ Tag UI can be properly designed in Phase 1.5

**Cons:**
- ⚠️ Still missing tagging workflow (main value proposition)
- ⚠️ Incomplete feature set

**Effort:** 2-3 hours
**Risk:** Low

## Recommended Action

**Use Solution 3 as immediate fix, plan Solution 1 for Phase 1.5**

**Rationale:**
- "Add to Library" is simple and high-value
- Tagging UI is complex and better designed holistically in Phase 1.5
- Phase 1.5 plan already includes TagCell and TagSelector components
- Better to have one working feature than three broken features

**Immediate Steps (Phase 1.3 completion):**
1. Implement "Add to Library" workflow (Solution 3)
2. Remove or disable "Tag..." menu item with tooltip: "Coming in Phase 1.5"
3. Remove "Properties" menu item (or defer)
4. Update Phase 1.5 plan to include context menu tagging integration

**Phase 1.5 Steps:**
1. Implement TagSelectorDialog component
2. Wire up "Tag..." context menu action
3. Implement Properties dialog if needed
4. Test full workflow: browse → add to library → tag → search

## Technical Details

**Affected Files:**
- `frontend/composables/useFileContextMenu.ts` (Lines 49-79)
- NEW: `frontend/components/TagManagement/TagSelectorDialog.vue`
- NEW: `frontend/components/FileExplorer/FilePropertiesDialog.vue`

**Existing Commands (already available):**
- `create_item(path, is_directory, size, modified_time)` → Returns item ID
- `get_item_by_path(path)` → Returns item or null
- `add_tag_to_item(item_id, tag_id)` → Associates tag with item
- `get_tags_for_item(item_id)` → Returns Tag[]
- `get_file_metadata(path)` → Returns FileMetadata

**Integration Flow:**
```
Context Menu "Add to Library" Click
    ↓
Check if item exists: get_item_by_path(entry.path)
    ↓
If null: create_item(...) → item_id
    ↓
Success toast: "Added to library"
    ↓
(Future) Enable "Tag..." menu item for this file
```

## Acceptance Criteria

**Phase 1.3 (Immediate):**
- [ ] "Add to Library" context menu item works
- [ ] Successfully creates item in database via `create_item`
- [ ] Shows success/error toast messages
- [ ] Duplicate detection (checks `get_item_by_path` first)
- [ ] "Tag..." menu item removed or disabled with tooltip
- [ ] "Properties" menu item removed or deferred
- [ ] Agent can programmatically add files to library

**Phase 1.5 (Future):**
- [ ] TagSelectorDialog component implemented
- [ ] "Tag..." context menu item enabled
- [ ] Tagging workflow: browse → add to library → tag → verify
- [ ] Properties dialog implemented (if desired)

## Work Log

### 2026-01-19 - Issue Discovered
- Agent-Native-Reviewer identified workflow integration gap
- Simplicity-Reviewer flagged TODO comments as YAGNI violations
- **Impact:** Users and agents cannot complete core workflows
- Recommended phased approach: Quick fix for Phase 1.3, full solution in Phase 1.5

## Resources

- **Phase 1.5 Plan:** `plans/feat-continue-phase1-implementation.md` (Lines 215-256)
- **Existing Commands:** `src/commands/items.rs` (create_item, get_item_by_path, add_tag_to_item)
- **Similar Pattern:** Tag creation inline in TagPanel.vue
- **UI Framework:** Native dialog element (no dependency)
