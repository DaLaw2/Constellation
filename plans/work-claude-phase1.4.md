# Work Plan: Claude - Phase 1.4 Tag Management Polish

## Overview
- **Agent**: Claude
- **Phase**: 1.4 - Tag Management Polish
- **Branch**: `feat/phase1.4-tag-polish`
- **Parallel Work**: Gemini is working on Phase 1.6 (Search) - no file conflicts

## Constitution Reference (AGENTS_CONSTITUTION.md)
- ❌ NO `unwrap()`/`expect()` without justification
- ✅ USE `thiserror` for errors, return `Result<T, AppError>`
- ✅ Run `cargo fmt` before commit
- ✅ Keep Rust models and TypeScript interfaces synchronized
- ✅ HALT if requirements unclear - ask for clarification

---

## Goal
Enhance tag management UI with better usage stats display and autocomplete UX.

## Current State
- **Backend**: ✅ Complete (`get_tag_usage_counts`, `search_tags` in `src/commands/tags.rs`)
- **Frontend**: ⚠️ Basic integration exists, needs UX polish

---

## Tasks

### Task 1: Enhance Usage Count Display
**File**: `frontend/components/TagManagement/TagPanel.vue`

- [ ] 1.1 Style usage count as a badge (not just parentheses)
- [ ] 1.2 Show "0" for unused tags with muted style
- [ ] 1.3 Add hover tooltip: "Used by X files"

**Current code** (line 44):
```vue
<span v-if="usageCounts[tag.id]" class="tag-count">({{ usageCounts[tag.id] }})</span>
```

### Task 2: Improve Autocomplete
**File**: `frontend/components/TagManagement/TagPanel.vue`

- [ ] 2.1 Add 300ms debounce to `handleTagSearch()`
- [ ] 2.2 Search globally (remove group filter) to catch duplicates
- [ ] 2.3 Show group name in suggestions
- [ ] 2.4 Click suggestion → auto-fill input
- [ ] 2.5 Arrow key navigation for suggestions

**Current code** (lines 163-178): `handleTagSearch()` function

### Task 3: Store Updates (if needed)
**File**: `frontend/stores/tags.ts`

- [ ] 3.1 Verify `searchTags` can work without groupId (it can - line 63-70)
- [ ] 3.2 Ensure usageCounts reactivity is correct

### Task 4: Quality Checks
- [ ] 4.1 Run `pnpm build` - no TypeScript errors
- [ ] 4.2 Run `cargo fmt && cargo clippy`
- [ ] 4.3 Test manually with `pnpm tauri dev`

---

## Files to Modify

| File | Changes |
|------|---------|
| `frontend/components/TagManagement/TagPanel.vue` | UI enhancements |
| `frontend/stores/tags.ts` | Minor (if needed) |

**No backend changes required** - APIs already exist.

---

## Commit Message
```
feat(tags): polish tag management with usage badges and autocomplete

- Style usage counts as badges with hover tooltips
- Add debounced global search for autocomplete
- Enable keyboard navigation in suggestions

Co-Authored-By: Claude <noreply@anthropic.com>
```
