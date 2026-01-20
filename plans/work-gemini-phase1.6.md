# Work Plan: Gemini - Phase 1.6 Search & Filter

## Overview
- **Agent**: Gemini
- **Phase**: 1.6 - Search & Filter
- **Branch**: `feat/phase1.6-search`
- **Parallel Work**: Claude is working on Phase 1.4 (Tag Polish) - no file conflicts

## Constitution Reference (AGENTS_CONSTITUTION.md)
- ❌ NO `unwrap()`/`expect()` without justification
- ❌ NO `anyhow::Result` - use `thiserror`
- ✅ Return `Result<T, AppError>` for all public functions
- ✅ Run `cargo fmt` before commit
- ✅ Keep Rust models and TypeScript interfaces synchronized
- ✅ HALT if requirements unclear - ask for clarification

---

## Goal
Implement search functionality to find files by tags (AND/OR logic) and filename.

---

## Tasks

### Task 1: Create Backend Search Module
**New File**: `src/commands/search.rs`

- [x] 1.1 Create `search_items_by_tags_and(tag_ids: Vec<i64>)` → Returns items with ALL tags
- [x] 1.2 Create `search_items_by_tags_or(tag_ids: Vec<i64>)` → Returns items with ANY tags
- [x] 1.3 Create `search_items_by_filename(query: String)` → LIKE query on path
- [x] 1.4 Create combined `search_items(tag_ids, mode, filename_query)` command

**Reference Pattern**: `src/commands/items.rs` for query structure
**Reference Pattern**: `src/commands/tags.rs:211-274` for LIKE search

**SQL for AND logic**:
```sql
SELECT i.* FROM items i
INNER JOIN item_tags it ON i.id = it.item_id
WHERE it.tag_id IN (?, ?, ?) AND i.is_deleted = 0
GROUP BY i.id
HAVING COUNT(DISTINCT it.tag_id) = ?  -- must match all tags
```

**SQL for OR logic**:
```sql
SELECT DISTINCT i.* FROM items i
INNER JOIN item_tags it ON i.id = it.item_id
WHERE it.tag_id IN (?, ?, ?) AND i.is_deleted = 0
```

### Task 2: Register Commands
**File**: `src/commands/mod.rs`

- [x] 2.1 Add `pub mod search;`

**File**: `src/lib.rs`

- [x] 2.2 Register search commands in `invoke_handler`

### Task 3: Create Search Store
**New File**: `frontend/stores/search.ts`

- [x] 3.1 Define `SearchState` interface
- [x] 3.2 Create `useSearchStore` with Pinia
- [x] 3.3 Implement `searchByTags(tagIds, mode)`
- [x] 3.4 Implement `searchByFilename(query)`
- [x] 3.5 Implement `combinedSearch()`

**Reference Pattern**: `frontend/stores/tags.ts` for store structure

### Task 4: Create FilterPanel Component
**New File**: `frontend/components/Search/FilterPanel.vue`

- [x] 4.1 Create component with `<script setup lang="ts">`
- [x] 4.2 Add AND/OR mode toggle
- [x] 4.3 Add tag selection checkboxes (grouped by tag group)
- [x] 4.4 Add filename search input
- [x] 4.5 Add "Search" button and results display
- [x] 4.6 Style following existing patterns (`TagPanel.vue`)

### Task 5: Integrate into LeftPanel
**File**: `frontend/components/Layout/LeftPanel.vue`

- [x] 5.1 Add "Search" tab button
- [x] 5.2 Import `FilterPanel` component
- [x] 5.3 Add conditional render for search mode
- [x] 5.4 Update `ViewMode` type in app store if needed

**Current tabs** (line 4-15):
```vue
<button :class="['tab-btn', { active: currentMode === 'file-browser' }]">
<button :class="['tab-btn', { active: currentMode === 'tag-management' }]">
```

### Task 6: Quality Checks
- [x] 6.1 Run `cargo fmt && cargo clippy` - no warnings
- [x] 6.2 Run `pnpm build` - no TypeScript errors
- [ ] 6.3 Test manually with `pnpm tauri dev`

---

## Files Summary

| File | Action |
|------|--------|
| `src/commands/search.rs` | **NEW** |
| `src/commands/mod.rs` | Add module |
| `src/lib.rs` | Register commands |
| `frontend/stores/search.ts` | **NEW** |
| `frontend/components/Search/FilterPanel.vue` | **NEW** |
| `frontend/components/Layout/LeftPanel.vue` | Add tab |
| `frontend/stores/app.ts` | Add 'search' to ViewMode (if needed) |

---

## Type Definitions

### Rust (in search.rs)
```rust
use crate::db::models::Item;
use crate::error::AppResult;
// ... follow existing patterns
```

### TypeScript (in search.ts)
```typescript
// Reuse Item type from items store or define
import type { Item } from './items' // or inline

interface SearchState {
  results: Item[]
  loading: boolean
  error: string | null
  mode: 'and' | 'or'
  selectedTagIds: number[]
  filenameQuery: string
}
```

---

## Commit Message
```
feat(search): implement tag-based and filename search

- Add search commands with AND/OR tag logic
- Create FilterPanel component with mode toggle
- Add Search tab to LeftPanel
- Create search store for state management

Co-Authored-By: Gemini <noreply@google.com>
```
