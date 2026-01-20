# Constellation Phase 1 Implementation Plan (Remaining)

## Current Status
- **Phase 1.1 - 1.3 (File Browser)**: ✅ Complete
- **Phase 1.5 (Core Tagging)**: ✅ Complete
- **Phase 1.4 (Tag Management)**: ⚠️ Partial
- **Phase 1.6 (Search & Filter)**: ❌ Pending
- **Phase 1.7 (Polish)**: ❌ Pending

---

## Remaining Phases

### Phase 1.4: Tag Management Polish
**Goal**: Add usage stats and autocomplete.

**Backend (`src/commands/tags.rs`)**:
- `get_tag_usage_counts()`: Return map of `tag_id -> count`.
- `search_tags(query, group_id)`: Autocomplete search.

**Frontend (`components/TagManagement/TagPanel.vue`)**:
- Show usage counts.
- Add autocomplete to tag creation input.

### Phase 1.6: Search & Filter
**Goal**: Find files by tags (AND/OR) and filename.

**Backend (`src/commands/search.rs`)**:
- `search_items_and(tag_ids)`: Items with ALL tags.
- `search_items_or(tag_ids)`: Items with ANY tags.
- `search_filenames(query)`: LIKE query on paths.

**Frontend**:
- `components/Search/FilterPanel.vue`: Toggle AND/OR mode, tag checkboxes.
- `stores/search.ts`: Manage search state.
- `components/Layout/LeftPanel.vue`: Add Search tab.

### Phase 1.7: First-Run & Polish
**Goal**: Onboarding and config persistence.

**Backend (`src/commands/config.rs`)**:
- `get_config()`, `save_config()`, `is_first_run()`.

**Frontend**:
- `components/Setup/SetupWizard.vue`: Database location, monitored drives.
- `App.vue`: Check first-run on mount.
- `components/Shared/EmptyState.vue`: Helper messages.

---

## Implementation Order
1.  **Phase 1.4**: Finish Tag Management.
2.  **Phase 1.6**: Implement Search.
3.  **Phase 1.7**: Setup Wizard & Polish.
