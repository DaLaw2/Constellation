# Work Plan: Phase 1.7 Polish & Bug Fixes

## Overview
- **Phase**: 1.7 - Polish & Bug Fixes
- **Branch**: `feat/phase1.7-polish`
- **Goal**: Fix critical bugs identified during testing, improve UX polish.
- **Status**: 🟢 Nearly Complete (4/6 tasks completed, only polish tasks remaining)

## Constitution Reference (AGENTS_CONSTITUTION.md)
- ❌ NO `unwrap()`/`expect()` without justification
- ❌ NO `anyhow::Result` - use `thiserror`
- ✅ Return `Result<T, AppError>` for all public functions
- ✅ Run `cargo fmt` before commit
- ✅ Keep Rust models and TypeScript interfaces synchronized
- ✅ HALT if requirements unclear - ask for clarification

---

## Critical Bugs

### ✅ Bug 1: Foreign Keys Not Enforced (CASCADE Deletion Fails) (COMPLETED)
**Priority**: P0 (Critical)
**Status**: ✅ **COMPLETED**
**Symptom**: Deleting tag groups or tags doesn't cascade properly; intermittent deletion failures.

**Root Cause**: `PRAGMA foreign_keys=ON` is only set during schema initialization in `src-tauri/src/db/schema.rs:117`. SQLite's PRAGMA settings are **per-connection**, not per-database. When using `deadpool_sqlite` connection pool, new connections from the pool don't inherit this setting.

**Fix Location**: `src-tauri/src/db/mod.rs`

**Solution**: Configure the connection pool to set `PRAGMA foreign_keys=ON` on each new connection using a connection manager hook or modify how connections are obtained.

**Reference Code**:
```rust
// In src-tauri/src/db/mod.rs
// Option 1: Create a custom hook that runs on each connection
// Option 2: Ensure PRAGMA is set before each operation that needs it
```

**SQL to understand the issue**:
```sql
-- Foreign keys must be enabled per-connection
PRAGMA foreign_keys;  -- Returns 0 if not set
PRAGMA foreign_keys=ON;  -- Must be called for each connection
```

- [x] 1.1 Add connection initialization hook to set `PRAGMA foreign_keys=ON` on every connection from the pool
- [x] 1.2 Verify cascade delete works by testing tag group deletion
- [x] 1.3 Verify cascade delete works by testing tag deletion (item_tags cleaned up)

---

### ✅ Bug 2: Self-Duplicate Error When Renaming Tags (COMPLETED)
**Priority**: P1 (High)
**Status**: ✅ **COMPLETED**
**Symptom**: When editing a tag and keeping the same name (clicking edit, then save without changes), the UI shows "Tag already exists in this group" error.

**Location**: `src/components/TagManagement/TagPanel.vue` lines 503-515

**Current Code**:
```javascript
const isDuplicateEditTag = computed(() => {
  if (!editingTag.value || !editTagValue.value.trim()) return false
  const normalized = editTagValue.value.trim().toLowerCase()
  const groupId = editingTag.value.group_id

  const groupTags = getTagsByGroup(groupId)

  return groupTags.some(t => {
    if (t.id === editingTag.value?.id) return false  // Should skip self
    return t.value.toLowerCase() === normalized
  })
})
```

**Hypothesis**: The logic appears correct (it skips the current tag by ID comparison). Possible causes:
1. Reactivity issue where `groupTags` returns fresh data but `editingTag.value` reference is stale
2. Type coercion issue with ID comparison (unlikely since both should be numbers)
3. Issue with how Vue's computed property tracks dependencies

**Investigation Steps**:
- [x] 2.1 Add console.log debugging to trace the exact values being compared
- [x] 2.2 Verify `editingTag.value.id` and `t.id` are both numbers (not strings)
- [x] 2.3 Check if `loadTags()` is being called between opening the dialog and validation
- [x] 2.4 Apply fix after root cause is identified

---

### ✅ Bug 3: Tag Group Reordering Not Persisting/Reflecting (COMPLETED)
**Priority**: P1 (High)
**Status**: ✅ **COMPLETED**
**Symptom**: Drag-and-drop reordering of tag groups doesn't update the UI immediately; order may not persist.

**Location**: `src/components/TagManagement/TagPanel.vue` lines 614-631

**Root Cause**: The `handleDrop` function doesn't await `reorderTagGroups` and doesn't optimistically update the UI:

```javascript
function handleDrop(dropIndex: number, event: DragEvent) {
  // ...
  const groups = [...tagGroups.value]  // Local copy
  const [draggedItem] = groups.splice(dragIndex, 1)
  groups.splice(dropIndex, 0, draggedItem)

  const orderedIds = groups.map(g => g.id)
  tagsStore.reorderTagGroups(orderedIds)  // NOT AWAITED!
}
```

**Problems**:
1. `reorderTagGroups` is not awaited, so errors aren't caught
2. Local `groups` array is reordered but never used to update UI
3. UI relies on `loadTagGroups()` inside the store to refresh, which happens asynchronously

**Fix**:
```javascript
async function handleDrop(dropIndex: number, event: DragEvent) {
  // ...
  const orderedIds = groups.map(g => g.id)
  try {
    await tagsStore.reorderTagGroups(orderedIds)
  } catch (e) {
    console.error('Failed to reorder:', e)
    // Optionally show user feedback
  }
}
```

- [x] 3.1 Make `handleDrop` async and await `reorderTagGroups`
- [x] 3.2 Add error handling/feedback for reorder failures
- [x] 3.3 Test reordering persists after page refresh

**Solution Implemented**: Used `vuedraggable` library with proper configuration:
- Added `dragDropEnabled: false` to `tauri.conf.json` to prevent native drag-drop interference
- Added CSS properties: `-webkit-user-drag: none`, `pointer-events: auto`, `user-select: none`
- Added `@click.stop` to drag handle to prevent event propagation
- Implemented proper error handling in `handleReorder` function

---

## UI/UX Polish

### ✅ Bug 4: Toggle Button Black Border on Click (COMPLETED)
**Priority**: P2 (Medium)
**Status**: ✅ **COMPLETED**
**Symptom**: Chevron toggle button shows black border when clicked.

**Location**:
- `src/components/TagManagement/TagPanel.vue` line 54 (`.btn-icon` class)
- `src/assets/styles/main.css` line 126 (`.btn-icon` definition)

**Root Cause**: The `.btn-icon` class doesn't have `border: none`, allowing browser default button styling to show on focus/active states.

**Current CSS** (main.css:126):
```css
.btn-icon {
  padding: 8px;
  background: transparent;
}
```

**Fix**: Add `border: none` to `.btn-icon`:
```css
.btn-icon {
  padding: 8px;
  background: transparent;
  border: none;
}
```

- [x] 4.1 Add `border: none` to `.btn-icon` in `src/assets/styles/main.css`
- [x] 4.2 Verify toggle buttons no longer show border on click

---

### Task 5: Disable Text Selection Globally
**Priority**: P3 (Low)
**Goal**: Prevent accidental text selection in UI (except in input fields).

**Location**: `src/assets/styles/main.css`

**Add CSS**:
```css
/* Disable text selection globally except in inputs */
body {
  user-select: none;
  -webkit-user-select: none;
}

input, textarea, [contenteditable="true"] {
  user-select: text;
  -webkit-user-select: text;
}
```

- [ ] 5.1 Add global text selection prevention to main.css
- [ ] 5.2 Verify inputs are still selectable

---

### Task 6: Restrict Global Context Menu
**Priority**: P3 (Low)
**Goal**: Disable default browser right-click menu except in specific areas.

**Location**: `src/App.vue`

**Add to App.vue**:
```javascript
onMounted(() => {
  document.addEventListener('contextmenu', (e) => {
    // Allow context menu only for specific elements
    if (!e.target.closest('.allow-context-menu')) {
      e.preventDefault()
    }
  })
})
```

- [ ] 6.1 Add global contextmenu prevention in App.vue
- [ ] 6.2 Mark areas that should allow context menu with `.allow-context-menu` class

---

## Removed Items (Already Complete or Invalid)

The following items from the original plan were removed:

1. ~~**Verify Search**~~: Search is fully implemented (FilterPanel.vue, search.rs, search.ts)
2. ~~**Pic View Mode**~~: Not a valid requirement; removed
3. ~~**Tag Group Expansion Black Border**~~: Duplicate of Bug 4

---

## Quality Checks

- [ ] Run `cargo fmt && cargo clippy` - no warnings
- [ ] Run `pnpm build` - no TypeScript errors
- [ ] Test manually with `pnpm tauri dev`:
  - [ ] Delete tag group → tags cascade delete
  - [ ] Delete tag → item_tags cascade delete
  - [ ] Rename tag to same name → no error
  - [ ] Drag reorder groups → persists after refresh
  - [ ] Click toggle buttons → no border flicker

---

## Files Summary

| File | Changes |
|------|---------|
| `src-tauri/src/db/mod.rs` | Add PRAGMA foreign_keys hook for connection pool |
| `src/components/TagManagement/TagPanel.vue` | Fix reorder await, investigate duplicate check |
| `src/assets/styles/main.css` | Add border:none to btn-icon, add user-select rules |
| `src/App.vue` | Add global contextmenu prevention |

---

## Commit Message
```
fix(tags): resolve deletion cascade and UI polish issues

- Enable foreign_keys PRAGMA on all pool connections (P0)
- Fix tag group reorder not awaiting backend call
- Remove button borders on click/focus
- Add global text selection prevention

Co-Authored-By: Claude <noreply@anthropic.com>
```
