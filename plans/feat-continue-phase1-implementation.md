# Continue Constellation Phase 1 Implementation

## Overview

This plan continues Constellation Phase 1 development from the current 30% completion point to MVP delivery. The project has a solid foundation (database layer, UI framework, tag management backend) but needs the core file browsing, tagging workflow, and search functionality to become useful.

**Current State:** Phases 1.1 and 1.2 complete, Phases 1.3-1.7 need implementation
**Target:** Complete Phase 1 MVP with all essential file tagging and search features
**Estimated Scope:** 6-8 weeks of focused development

---

## Problem Statement

### Current Gaps

Based on repository analysis, here's what's missing from the Phase 1 MVP:

**Phase 1.3 (File Browser) - 5% Complete:**
- ❌ No drive enumeration or directory reading backend commands
- ❌ DirectoryTree, FileItem, ContextMenu components missing
- ❌ FileList is just a placeholder
- ❌ No file system operations (open, reveal, copy path)

**Phase 1.4 (Tag Management) - 70% Complete:**
- ✅ Backend CRUD operations complete
- ✅ Basic TagPanel UI exists
- ⚠️ Tag usage statistics not displayed
- ⚠️ Tag autocomplete missing

**Phase 1.5 (Core Tagging) - 20% Complete:**
- ✅ Individual tag add/remove commands exist
- ❌ No inline tag editing UI
- ❌ No TagCell or TagSelector components
- ❌ No bulk update_item_tags command
- ❌ Tag templates schema exists but no commands/UI

**Phase 1.6 (Search & Filter) - 0% Complete:**
- ❌ No search backend commands
- ❌ FilterPanel and AdvancedSearch components missing
- ❌ Search input in TopBar is just a placeholder

**Phase 1.7 (First-Run & Polish) - 5% Complete:**
- ❌ No SetupWizard or first-run detection
- ❌ Basic AppConfig exists but missing fields
- ❌ No config persistence logic
- ❌ No onboarding overlays or helpful empty states

### Why This Matters

Without these features, users cannot:
1. Browse files in the application
2. Tag files through the UI
3. Search/filter tagged files
4. Get started easily on first launch

The project is **not usable** until these core workflows are complete.

---

## Proposed Solution

### Implementation Strategy

Complete Phases 1.3 through 1.7 in dependency order, focusing on MVP functionality first before polish. Each phase builds on the previous one:

```
Phase 1.3 (File Browser) → Phase 1.5 (Tagging) → Phase 1.6 (Search) → Phase 1.7 (Polish)
                               ↑
                         Phase 1.4 (Polish Tag Mgmt)
```

### High-Level Approach

1. **File Browser First** - Users need to see files before they can tag them
2. **Tagging Workflow Second** - Core value proposition
3. **Search & Filter Third** - Makes tags useful
4. **Polish Last** - Onboarding and refinement

---

## Technical Approach

### Phase 1.3: File Browser Implementation (Week 1-2)

**Backend Commands to Implement:**

```rust
// src/commands/filesystem.rs (NEW FILE)

#[tauri::command]
pub async fn get_drives() -> AppResult<Vec<DriveInfo>> {
    // Use Windows API to enumerate fixed drives
    // Return drive letters, labels, total/free space
}

#[tauri::command]
pub async fn read_directory(path: String) -> AppResult<Vec<FileEntry>> {
    // Read directory contents
    // Return files and folders with metadata
    // Sort by name, filter hidden files
}

#[tauri::command]
pub async fn get_file_metadata(path: String) -> AppResult<FileMetadata> {
    // Get detailed file info: size, modified time, type
}

#[tauri::command]
pub async fn open_file_external(path: String) -> AppResult<()> {
    // Open file with default application
    // Use tauri-plugin-opener
}

#[tauri::command]
pub async fn reveal_in_explorer(path: String) -> AppResult<()> {
    // Show file in Windows Explorer
}
```

**Files:**
- `src/commands/filesystem.rs` - NEW: File system operations
- `src/lib.rs` - Register new commands

**Frontend Components to Implement:**

```typescript
// frontend/components/FileExplorer/DirectoryTree.vue
- Displays hierarchical folder structure
- Lazy loads children on expand
- Handles folder selection
- Shows drive roots at top level

// frontend/components/FileExplorer/FileItem.vue
- Displays individual file/folder in list
- Shows icon, name, size, date, tags
- Handles selection and context menu

// frontend/components/FileExplorer/FileList.vue (REPLACE)
- Uses RecycleScroller for virtual scrolling
- Renders FileItem components
- Handles sorting (name, date, size, tags)
- Shows empty state when folder empty

// frontend/components/FileExplorer/ContextMenu.vue
- Right-click menu for files
- Actions: Open, Reveal, Copy Path
- Uses @imengyu/vue3-context-menu library
```

**Files:**
- `frontend/components/FileExplorer/DirectoryTree.vue` - NEW
- `frontend/components/FileExplorer/FileItem.vue` - NEW
- `frontend/components/FileExplorer/FileList.vue` - REPLACE placeholder
- `frontend/components/FileExplorer/ContextMenu.vue` - NEW
- `frontend/stores/fileExplorer.ts` - NEW: State for current path, files, selection

**Key Libraries:**
- `@imengyu/vue3-context-menu` for context menus
- `vue-virtual-scroller` (already installed) for file list

**References:**
- Similar implementation: Phase 1 plan lines 2067-2114
- Tauri fs plugin docs: https://v2.tauri.app/plugin/file-system/
- Windows drive enumeration: Windows API GetLogicalDrives

---

### Phase 1.4: Polish Tag Management (Week 1-2, Parallel with 1.3)

**Backend Commands to Implement:**

```rust
// src/commands/tags.rs (ADD)

#[tauri::command]
pub async fn get_tag_usage_counts(state: State<'_, AppState>) -> AppResult<HashMap<i64, i64>> {
    // Return map of tag_id → count of items tagged
    // JOIN item_tags GROUP BY tag_id
}

#[tauri::command]
pub async fn search_tags(
    query: String,
    group_id: Option<i64>,
    state: State<'_, AppState>
) -> AppResult<Vec<Tag>> {
    // Autocomplete search for tags by value
    // Filter by group_id if provided
    // LIMIT 10 for dropdown
}
```

**Files:**
- `src/commands/tags.rs` - ADD new commands

**Frontend Components to Update:**

```vue
<!-- frontend/components/TagManagement/TagPanel.vue (UPDATE) -->
- Add tag usage counts next to each tag
- Implement autocomplete in tag creation input
- Show warning when deleting tags with items
```

**Files:**
- `frontend/components/TagManagement/TagPanel.vue` - UPDATE

**References:**
- Phase 1 plan lines 2117-2185
- Autocomplete pattern: debounce 300ms

---

### Phase 1.5: Core Tagging Workflow (Week 3-4)

**Backend Commands to Implement:**

```rust
// src/commands/items.rs (ADD)

#[tauri::command]
pub async fn update_item_tags(
    item_id: i64,
    tag_ids: Vec<i64>,
    state: State<'_, AppState>
) -> AppResult<()> {
    // Replace all tags for an item atomically
    // Transaction: DELETE existing + INSERT new
}

// src/commands/tag_templates.rs (NEW FILE)

#[tauri::command]
pub async fn create_tag_template(
    name: String,
    tag_ids: Vec<i64>,
    state: State<'_, AppState>
) -> AppResult<i64> {
    // Save tag combination as template
}

#[tauri::command]
pub async fn get_tag_templates(state: State<'_, AppState>) -> AppResult<Vec<TagTemplate>> {
    // Return all saved templates
}

#[tauri::command]
pub async fn apply_tag_template(
    item_id: i64,
    template_id: i64,
    state: State<'_, AppState>
) -> AppResult<()> {
    // Apply template's tags to item
}
```

**Files:**
- `src/commands/items.rs` - ADD update_item_tags
- `src/commands/tag_templates.rs` - NEW FILE
- `src/lib.rs` - Register new commands

**Frontend Components to Implement:**

```vue
<!-- frontend/components/TagManagement/TagCell.vue (NEW) -->
<template>
  <!-- Display mode: Show first 3 tags + "...+N" -->
  <div v-if="!editing" @click="startEdit" class="tag-cell">
    <span v-for="tag in displayTags" class="tag-badge">{{ tag.value }}</span>
    <span v-if="remainingCount > 0">...+{{ remainingCount }}</span>
  </div>

  <!-- Edit mode: Dropdowns grouped by tag group -->
  <div v-else class="tag-editor">
    <div v-for="group in tagGroups" :key="group.id">
      <label>{{ group.name }}</label>
      <select v-model="selectedTags[group.id]" multiple>
        <option v-for="tag in group.tags" :value="tag.id">{{ tag.value }}</option>
        <option value="__create__">+ Create New...</option>
      </select>
    </div>
    <button @click="save">Save</button>
    <button @click="cancel">Cancel</button>
  </div>
</template>

<!-- frontend/components/TagManagement/TagSelector.vue (NEW) -->
<!-- Reusable component for selecting tags from dropdowns -->

<!-- frontend/components/TagManagement/TemplateManager.vue (NEW) -->
<!-- UI for creating, viewing, applying tag templates -->
```

**Files:**
- `frontend/components/TagManagement/TagCell.vue` - NEW: Inline tag editor
- `frontend/components/TagManagement/TagSelector.vue` - NEW: Reusable selector
- `frontend/components/TagManagement/TemplateManager.vue` - NEW: Template management
- `frontend/components/FileExplorer/FileItem.vue` - UPDATE: Use TagCell component

**References:**
- Phase 1 plan lines 2188-2261
- Current items.ts store for API patterns

---

### Phase 1.6: Search & Filter (Week 5-6)

**Backend Commands to Implement:**

```rust
// src/commands/search.rs (NEW FILE)

#[tauri::command]
pub async fn search_items_and(
    tag_ids: Vec<i64>,
    state: State<'_, AppState>
) -> AppResult<Vec<Item>> {
    // Find items with ALL specified tags (AND logic)
    // Use INTERSECT for efficiency
    // Filter is_deleted = 0
}

#[tauri::command]
pub async fn search_items_or(
    tag_ids: Vec<i64>,
    state: State<'_, AppState>
) -> AppResult<Vec<Item>> {
    // Find items with ANY specified tags (OR logic)
    // Use UNION or IN clause
}

#[tauri::command]
pub async fn search_items_complex(
    query: SearchQuery,  // Nested AND/OR/NOT structure
    state: State<'_, AppState>
) -> AppResult<Vec<Item>> {
    // Build dynamic WHERE clause from query tree
    // Consider using SeaQuery for safety
}

#[tauri::command]
pub async fn search_filenames(
    query: String,
    state: State<'_, AppState>
) -> AppResult<Vec<Item>> {
    // Search by filename/path substring
    // Consider FTS5 for performance
}
```

**Files:**
- `src/commands/search.rs` - NEW FILE
- `src/lib.rs` - Register search commands

**Frontend Components to Implement:**

```vue
<!-- frontend/components/Search/FilterPanel.vue (NEW) -->
<template>
  <div class="filter-panel">
    <div class="mode-switch">
      <button @click="mode = 'simple'" :class="{active: mode === 'simple'}">
        Simple Filter
      </button>
      <button @click="mode = 'advanced'" :class="{active: mode === 'advanced'}">
        Advanced Search
      </button>
    </div>

    <!-- Simple Mode: Tag checkboxes with AND logic -->
    <div v-if="mode === 'simple'" class="simple-filter">
      <div v-for="group in tagGroups" :key="group.id" class="tag-group">
        <h3>{{ group.name }}</h3>
        <label v-for="tag in group.tags" :key="tag.id">
          <input type="checkbox" v-model="selectedTags" :value="tag.id" />
          {{ tag.value }} ({{ tag.count }})
        </label>
      </div>
      <button @click="clearFilters">Clear All</button>
    </div>

    <!-- Advanced Mode: Visual query builder -->
    <AdvancedSearch v-else @search="handleSearch" />
  </div>
</template>

<!-- frontend/components/Search/AdvancedSearch.vue (NEW) -->
<!-- Visual query builder with AND/OR/NOT logic -->
<!-- Recursive component for nested groups -->

<!-- frontend/stores/search.ts (NEW) -->
// State management for search results and filters
```

**Files:**
- `frontend/components/Search/FilterPanel.vue` - NEW
- `frontend/components/Search/AdvancedSearch.vue` - NEW
- `frontend/stores/search.ts` - NEW
- `frontend/components/Layout/LeftPanel.vue` - UPDATE: Integrate FilterPanel

**Key Libraries:**
- Consider `vue-query-builder` or build custom recursive component

**References:**
- Phase 1 plan lines 2264-2358
- Best practices research on complex query building
- SQLite INTERSECT pattern for AND logic

---

### Phase 1.7: First-Run Setup & Polish (Week 7-8)

**Backend Commands to Implement:**

```rust
// src/commands/config.rs (NEW FILE)

#[tauri::command]
pub async fn get_config(state: State<'_, AppState>) -> AppResult<AppConfig> {
    // Load config from file or defaults
}

#[tauri::command]
pub async fn save_config(
    config: AppConfig,
    state: State<'_, AppState>
) -> AppResult<()> {
    // Save config to JSON file
    // Update in-memory state
}

#[tauri::command]
pub async fn is_first_run(state: State<'_, AppState>) -> AppResult<bool> {
    // Check if config file exists
}

#[tauri::command]
pub async fn complete_setup(
    config: AppConfig,
    state: State<'_, AppState>
) -> AppResult<()> {
    // Save initial config
    // Create default tag groups if requested
}
```

**Files:**
- `src/commands/config.rs` - NEW
- `src/state.rs` - UPDATE: Expand AppConfig with monitored_drives, created_at
- `src/lib.rs` - Register config commands

**Frontend Components to Implement:**

```vue
<!-- frontend/components/Setup/SetupWizard.vue (NEW) -->
<template>
  <div class="setup-wizard" v-if="showSetup">
    <div class="step" v-if="currentStep === 1">
      <h2>Welcome to Constellation</h2>
      <p>Let's get you set up to start organizing your files.</p>
      <button @click="nextStep">Get Started</button>
    </div>

    <div class="step" v-if="currentStep === 2">
      <h2>Choose Database Location</h2>
      <input type="text" v-model="config.dbPath" readonly />
      <button @click="selectDbPath">Change Location</button>
      <button @click="nextStep">Next</button>
    </div>

    <div class="step" v-if="currentStep === 3">
      <h2>Select Drives to Monitor</h2>
      <label v-for="drive in availableDrives" :key="drive.letter">
        <input type="checkbox" v-model="config.monitoredDrives" :value="drive.letter" />
        {{ drive.letter }}: ({{ drive.label }})
      </label>
      <button @click="nextStep">Next</button>
    </div>

    <div class="step" v-if="currentStep === 4">
      <h2>Create Default Tag Groups?</h2>
      <label>
        <input type="checkbox" v-model="createDefaults" />
        Create sample tag groups (Language, Author, Property)
      </label>
      <button @click="finishSetup">Finish Setup</button>
    </div>
  </div>
</template>

<!-- frontend/components/Shared/EmptyState.vue (NEW) -->
<!-- Reusable empty state component with types -->

<!-- frontend/components/Shared/LoadingSkeleton.vue (NEW) -->
<!-- Skeleton loaders for file list -->
```

**Files:**
- `frontend/components/Setup/SetupWizard.vue` - NEW
- `frontend/components/Shared/EmptyState.vue` - NEW
- `frontend/components/Shared/LoadingSkeleton.vue` - NEW
- `frontend/App.vue` - UPDATE: Check first run on mount

**Onboarding Library:**
- Install `v-onboarding` for tutorial overlays
- Create onboarding steps for key features
- Store "onboarding completed" flag in localStorage

**References:**
- Phase 1 plan lines 2359-2483
- v-onboarding docs: https://v-onboarding.fatihsolhan.com/
- Empty state best practices research

---

## Implementation Phases

### Week 1-2: File Browser Foundation
- **Focus**: Users can navigate drives and folders, see file lists
- **Tasks**:
  - [x] Implement backend file system commands (get_drives, read_directory)
  - [x] Create DirectoryTree component with lazy loading
  - [x] Create FileItem and FileList components with virtual scrolling
  - [x] Add context menu integration
  - [x] Connect to fileExplorer Pinia store
- **Success Criteria**: Can browse files, no tagging functionality yet

### Week 3-4: Core Tagging Workflow
- **Focus**: Users can tag files through inline editing
- **Tasks**:
  - [x] Implement update_item_tags bulk command
  - [x] Create TagCell inline editor component
  - [x] Create TagSelector reusable component
  - [x] Integrate TagCell into FileItem
  - [x] Implement "Create New Tag" inline flow
  - [x] Add tag templates backend and UI
- **Success Criteria**: Can click file, edit tags, save changes

### Week 5-6: Search & Filter
- **Focus**: Users can find tagged files with AND/OR/NOT logic
- **Tasks**:
  - [ ] Implement search backend commands (AND, OR, complex)
  - [ ] Create FilterPanel with simple checkbox mode
  - [ ] Create AdvancedSearch query builder
  - [ ] Add real-time search results
  - [ ] Implement filename quick search
  - [ ] Connect to search Pinia store
- **Success Criteria**: Can filter by tags, search by filename, see results

### Week 7-8: Setup & Polish
- **Focus**: First-run experience and usability improvements
- **Tasks**:
  - [ ] Expand AppConfig with all fields
  - [ ] Implement config persistence commands
  - [ ] Create SetupWizard multi-step flow
  - [ ] Add v-onboarding tutorial overlays
  - [ ] Create EmptyState and LoadingSkeleton components
  - [ ] Integrate empty states throughout app
  - [ ] Polish tag management UI (usage counts, autocomplete)
- **Success Criteria**: New users can set up easily, UI feels complete

---

## Alternative Approaches Considered

### 1. Build Search First (Rejected)
**Reasoning**: Users need files to search before search is useful. File browser is foundation.

### 2. Skip First-Run Setup (Rejected)
**Reasoning**: Poor onboarding = abandoned app. Setup wizard critical for retention.

### 3. Use External File Manager (Rejected)
**Reasoning**: Defeats purpose of integrated tagging experience. Constellation needs built-in browser.

### 4. Implement Phase 2 Features (Rejected)
**Reasoning**: Phase 1 is not usable yet. Finish MVP before enhancements.

---

## Success Metrics

**Functional Completeness:**
- [ ] All Phase 1.3-1.7 acceptance criteria met (from original plan)
- [ ] Can complete full workflow: setup → browse → tag → search
- [ ] No critical bugs or blocking issues

**Performance:**
- [ ] File list with 1000+ files scrolls at 60 FPS
- [ ] Search results return in < 100ms for 10K items
- [ ] Directory tree expansion feels instant (< 50ms)

**Usability:**
- [ ] First-run setup completes in < 2 minutes
- [ ] Tagging a file takes < 5 clicks
- [ ] No empty state without helpful message

---

## Dependencies & Prerequisites

### Backend
- ✅ Tauri 2.x project structure (complete)
- ✅ SQLite database with schema (complete)
- ✅ Database pool with async commands (complete)
- ✅ Tag and Item models defined (complete)
- ⚠️ File system operations need Windows API integration

### Frontend
- ✅ Vue 3 + TypeScript setup (complete)
- ✅ Pinia stores for tags and items (complete)
- ✅ vue-virtual-scroller installed (complete)
- ⚠️ Need to install: @imengyu/vue3-context-menu, v-onboarding

### External
- ✅ Windows 10/11 development environment
- ⚠️ May need tauri-plugin-opener for external file opening

---

## Risk Analysis & Mitigation

### Risk: USN Journal Integration Complexity
**Impact**: High - Path tracking is key feature
**Likelihood**: Medium
**Mitigation**:
- Implement fallback file watcher first
- USN Journal as Phase 2 enhancement
- Focus on manual refresh for Phase 1 MVP

### Risk: Virtual Scrolling Performance
**Impact**: Medium - Poor UX with large file lists
**Likelihood**: Low
**Mitigation**:
- vue-virtual-scroller already proven library
- Test with 10K+ file directory early
- Fixed item height (48px) for best performance

### Risk: Scope Creep
**Impact**: High - Delays MVP
**Likelihood**: Medium
**Mitigation**:
- Strict adherence to Phase 1 acceptance criteria
- No Phase 2 features before MVP complete
- Regular progress reviews against plan

### Risk: Complex Query Builder UI
**Impact**: Medium - Advanced search unusable
**Likelihood**: Medium
**Mitigation**:
- Start with simple checkbox filter (80% use case)
- Query builder as progressive enhancement
- Thorough testing of edge cases

---

## References & Research

### Internal References
- **Original Phase 1 Plan**: `docs/plans/feat-constellation-mvp-phase1-foundation.md`
- **Database Schema**: `src/db/schema.rs`
- **Existing Models**: `src/db/models.rs`
- **Tag Commands**: `src/commands/tags.rs:1-182`
- **Item Commands**: `src/commands/items.rs:1-372`

### External References

**Tauri Documentation:**
- File System Plugin: [https://v2.tauri.app/plugin/file-system/](https://v2.tauri.app/plugin/file-system/)
- State Management: [https://v2.tauri.app/develop/state-management/](https://v2.tauri.app/develop/state-management/)
- Async Commands: [https://v2.tauri.app/develop/calling-rust/](https://v2.tauri.app/develop/calling-rust/)

**Vue 3 Libraries:**
- v-onboarding: [https://v-onboarding.fatihsolhan.com/](https://v-onboarding.fatihsolhan.com/)
- vue3-context-menu: [https://github.com/imengyu/vue3-context-menu](https://github.com/imengyu/vue3-context-menu)
- vue-virtual-scroller: [https://github.com/Akryum/vue-virtual-scroller](https://github.com/Akryum/vue-virtual-scroller)

**SQLite:**
- FTS5 Full-Text Search: [https://www.sqlite.org/fts5.html](https://www.sqlite.org/fts5.html)
- Query Optimization: [https://www.sqlite.org/queryplanner.html](https://www.sqlite.org/queryplanner.html)

**Best Practices:**
- Empty State UX: [https://www.toptal.com/designers/ux/empty-state-ux-design](https://www.toptal.com/designers/ux/empty-state-ux-design)
- Vue Debouncing: [https://cloudinary.com/guides/web-performance/vue-debounce](https://cloudinary.com/guides/web-performance/vue-debounce)

---

## Acceptance Criteria Summary

From Phase 1 original plan, all following must be met:

**Phase 1.3:**
- [x] Can browse all local drives
- [x] Directory tree navigation works
- [x] File list shows files with metadata
- [x] Virtual scrolling handles 1000+ files
- [x] Context menu provides file operations

**Phase 1.5:**
- [ ] Can click tag cell to enter edit mode
- [ ] Tag selection dropdowns grouped properly
- [ ] Can create new tags inline
- [ ] Tags auto-save on blur
- [ ] Templates can be created and applied

**Phase 1.6:**
- [ ] Simple filter panel works with AND logic
- [ ] Advanced search builder allows AND/OR/NOT
- [ ] Real-time search results
- [ ] Quick filename search functional

**Phase 1.7:**
- [ ] Setup wizard guides first-run experience
- [ ] Config persists between sessions
- [ ] Onboarding overlays help new users
- [ ] Empty states provide helpful guidance

---

## Next Steps

**Ready to Start?**

1. **Begin with Phase 1.3** (File Browser) - Foundation for everything else
2. **Parallel work on Phase 1.4** (Tag polish) - Can happen simultaneously
3. **Follow with Phase 1.5** (Tagging UI) - Requires file browser complete
4. **Then Phase 1.6** (Search) - Requires tagging to be useful
5. **Finish with Phase 1.7** (Setup & Polish) - Final touches

**Estimated Timeline:** 6-8 weeks to MVP
**Current Progress:** 30% complete
**Target:** 100% Phase 1 MVP

Use `/workflows:work plans/feat-continue-phase1-implementation.md` to begin execution of this plan.
