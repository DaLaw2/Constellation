---
status: completed
priority: p1
issue_id: "008"
tags: [code-review, bug, frontend, vue]
dependencies: []
completed_at: 2026-01-19
---

# Computed Ref Mutation Bug in FileList Selection

## Problem Statement

`FileList.vue` attempts to mutate a computed ref, which is **read-only** in Vue 3. This causes file selection to silently fail - users click on files but nothing gets selected. This breaks the entire file selection workflow.

**Why this matters:**
- Users cannot select files (broken core functionality)
- Silent failure - no error message, just doesn't work
- Prevents implementing features like "tag selected files"
- Bad user experience - appears to be broken UI

## Findings

### Source: Architecture-Strategist Agent

**Vulnerable Code** (`frontend/components/FileExplorer/FileList.vue`):

```typescript
// Line 79: selectedPath is a computed ref (READ-ONLY)
const selectedPath = computed(() => fileExplorerStore.currentPath)

// Lines 97-100: Attempting to mutate computed ref (WILL FAIL)
function handleFileClick(entry: FileEntry) {
  // Single click - just select
  selectedPath.value = entry.path  // ❌ Cannot mutate computed ref!
}
```

**Why It's Broken:**
- `computed()` creates a **read-only** reactive reference
- Derived from `fileExplorerStore.currentPath`
- Attempting to assign `selectedPath.value =` does nothing (or throws error in strict mode)
- Selection state is never actually updated

**Observed Behavior:**
- User clicks file → `handleFileClick` fires
- `selectedPath.value =` fails silently
- File doesn't appear selected in UI
- User confused, clicks again, nothing happens

### Evidence from Pattern-Recognition Agent

This is a **common anti-pattern** in Vue 3:
- Confusing `computed()` (read-only) with `ref()` (writable)
- Pattern-Recognition-Specialist flagged this as critical bug

## Proposed Solutions

### Solution 1: Use Local Ref for Selection State (RECOMMENDED)

**Approach:** Separate selection from navigation. Track selected file in local component state.

**Implementation:**
```typescript
// frontend/components/FileExplorer/FileList.vue

<script setup lang="ts">
import { computed, ref } from 'vue'
import { RecycleScroller } from 'vue-virtual-scroller'
import { useFileExplorerStore } from '../../stores/fileExplorer'
import { useFileContextMenu } from '../../composables/useFileContextMenu'
import FileItem from './FileItem.vue'
import type { FileEntry } from '../../stores/fileExplorer'
import 'vue-virtual-scroller/dist/vue-virtual-scroller.css'

const fileExplorerStore = useFileExplorerStore()
const { showFileContextMenu } = useFileContextMenu()

const currentPath = computed(() => fileExplorerStore.currentPath)
const files = computed(() => fileExplorerStore.currentFiles)
const loading = computed(() => fileExplorerStore.loading)
const error = computed(() => fileExplorerStore.error)

// NEW: Local selection state (writable ref)
const selectedPath = ref<string | null>(null)

const canNavigateUp = computed(() => {
  if (!currentPath.value) return false
  const pathParts = currentPath.value.split('\\').filter(Boolean)
  return pathParts.length > 1
})

function navigateUp() {
  fileExplorerStore.navigateUp()
  selectedPath.value = null  // Clear selection on navigation
}

function refresh() {
  if (currentPath.value) {
    fileExplorerStore.readDirectory(currentPath.value)
    selectedPath.value = null  // Clear selection on refresh
  }
}

function handleFileClick(entry: FileEntry) {
  // Single click - update local selection state
  selectedPath.value = entry.path
}

function handleFileDoubleClick(entry: FileEntry) {
  if (entry.is_directory) {
    // Navigate into directory
    fileExplorerStore.navigateTo(entry.path)
    selectedPath.value = null  // Clear selection on navigation
  } else {
    // Open file with default application
    fileExplorerStore.openFileExternal(entry.path)
  }
}

function handleFileContextMenu(entry: FileEntry, event: MouseEvent) {
  // Select file before showing context menu
  selectedPath.value = entry.path

  showFileContextMenu({
    entry,
    x: event.x,
    y: event.y,
  })
}
</script>

<template>
  <!-- ... -->
  <RecycleScroller
    v-else
    class="file-scroller"
    :items="files"
    :item-size="60"
    key-field="path"
    v-slot="{ item }"
  >
    <FileItem
      :entry="item"
      :selected="selectedPath === item.path"
      @click="handleFileClick"
      @double-click="handleFileDoubleClick"
      @context-menu="handleFileContextMenu"
    />
  </RecycleScroller>
  <!-- ... -->
</template>
```

**Pros:**
- ✅ Fixes the bug completely
- ✅ Selection is independent from navigation
- ✅ Clear semantics: `ref()` is writable, `computed()` is read-only
- ✅ Follows Vue 3 best practices

**Cons:**
- ⚠️ Selection state not persisted (cleared on navigation)
- ⚠️ Need to clear selection on directory change

**Effort:** 30 minutes (straightforward fix)
**Risk:** Low (standard Vue pattern)

### Solution 2: Add setSelectedPath to Store

**Approach:** Track selection in the Pinia store with a dedicated setter method.

**Implementation:**
```typescript
// frontend/stores/fileExplorer.ts
export const useFileExplorerStore = defineStore('fileExplorer', () => {
  const currentPath = ref<string>('')
  const currentFiles = ref<FileEntry[]>([])
  const selectedPath = ref<string | null>(null)  // NEW
  const drives = ref<DriveInfo[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)

  async function readDirectory(path: string) {
    try {
      loading.value = true
      error.value = null
      currentFiles.value = await invoke<FileEntry[]>('read_directory', { path })
      currentPath.value = path
      selectedPath.value = null  // Clear selection on directory change
      return currentFiles.value
    } catch (e) {
      error.value = e as string
      console.error('Failed to read directory:', e)
      throw e
    } finally {
      loading.value = false
    }
  }

  function setSelectedPath(path: string | null) {
    selectedPath.value = path
  }

  return {
    // ... existing exports
    selectedPath,
    setSelectedPath,
  }
})

// frontend/components/FileExplorer/FileList.vue
const selectedPath = computed(() => fileExplorerStore.selectedPath)

function handleFileClick(entry: FileEntry) {
  fileExplorerStore.setSelectedPath(entry.path)  // Use store method
}
```

**Pros:**
- ✅ Centralized selection state in store
- ✅ Selection can be accessed by other components
- ✅ Easier to persist selection history

**Cons:**
- ❌ More complex than local state
- ❌ Selection is global (may not be desired)
- ⚠️ Still need to use `computed()` correctly in component

**Effort:** 1 hour (store + component changes)
**Risk:** Low (but more complex)

### Solution 3: Use v-model with Custom Setter

**Approach:** Create a writable computed with custom getter/setter.

**Implementation:**
```typescript
const selectedPath = computed({
  get() {
    return fileExplorerStore.currentPath
  },
  set(value: string) {
    // Custom logic for setting selection
    // Could dispatch to store or update local ref
  }
})
```

**Pros:**
- ✅ Keeps computed syntax
- ✅ Allows custom logic

**Cons:**
- ❌ More complex than simple ref
- ❌ Unclear semantics (selection vs navigation)
- ❌ Not the intended use case

**Effort:** 45 minutes
**Risk:** Medium (confusing pattern)

## Recommended Action

**Use Solution 1: Local ref for selection state**

**Rationale:**
- Simplest and clearest solution
- Follows Vue 3 best practices
- Selection is a UI concern (belongs in component)
- Navigation and selection are separate concepts

**Implementation Steps:**
1. Change `const selectedPath = computed(() => ...)` to `const selectedPath = ref<string | null>(null)`
2. Update `handleFileClick` to `selectedPath.value = entry.path`
3. Add `selectedPath.value = null` in navigation functions (navigateUp, handleFileDoubleClick)
4. Test file selection works correctly
5. Test selection clears on directory navigation
6. Test context menu still works (selects file before showing menu)

## Technical Details

**Affected Files:**
- `frontend/components/FileExplorer/FileList.vue` (Lines 79, 97-100)

**Vue 3 Computed Refs:**
- `computed()` creates a **read-only** reactive value
- Derived from other reactive sources
- Automatically updates when dependencies change
- **Cannot be assigned to** (will fail or throw error)

**Correct Usage:**
```typescript
// ✅ Read-only computed
const doubled = computed(() => count.value * 2)

// ✅ Writable ref
const count = ref(0)
count.value = 42  // Works

// ❌ Wrong: mutating computed
doubled.value = 84  // ERROR or silent failure
```

**Selection vs Navigation:**
- **Selection:** Which file is highlighted (UI state)
- **Navigation:** Which directory is being viewed (data state)
- These are **different concerns** and should be tracked separately

## Acceptance Criteria

- [ ] File selection works - clicking a file highlights it
- [ ] Visual feedback shows selected file (border, background color)
- [ ] Selection clears when navigating to a new directory
- [ ] Selection clears when clicking "Up" button
- [ ] Context menu correctly selects file before showing menu
- [ ] Double-clicking file doesn't leave orphaned selection
- [ ] No console errors related to readonly refs

## Work Log

### 2026-01-19 - Issue Discovered
- Architecture-Strategist agent identified computed ref mutation
- Pattern-Recognition-Specialist flagged as anti-pattern
- **Severity:** CRITICAL (breaks core functionality)
- **Impact:** File selection doesn't work at all
- Recommended fix: Use local ref instead of computed

## Resources

- **Vue 3 Docs:** [Computed Properties](https://vuejs.org/guide/essentials/computed.html)
- **Vue 3 Docs:** [Refs and Reactivity](https://vuejs.org/guide/essentials/reactivity-fundamentals.html)
- **Pattern:** Selection state is typically local to list components
- **Similar Issue:** None in codebase (first occurrence)
