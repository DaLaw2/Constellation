# Constellation Roadmap: v1.0 Release & Future Vision

## Overview

This document outlines the development roadmap for the Constellation project, including:
1. **Phase 0**: Domain-Driven Design (DDD) Modeling (Primary Task)
2. **Phase 1.x**: Complete remaining UI and release v1.0
3. **Phase 2.x+**: Future features

---

## Current Architecture Summary

The project adopts **Domain-Driven Design (DDD)** architecture:

```
src-tauri/src/
├── domain/                    # Domain Layer
│   ├── entities/              # Aggregates & Entities
│   │   ├── item.rs            # Item (File/Directory)
│   │   ├── tag.rs             # Tag
│   │   ├── tag_group.rs       # TagGroup
│   │   └── tag_template.rs    # TagTemplate
│   ├── value_objects/         # Value Objects
│   │   ├── color.rs           # Color
│   │   ├── file_path.rs       # FilePath
│   │   └── tag_value.rs       # TagValue
│   ├── repositories/          # Repository Traits (Ports)
│   └── errors.rs              # Domain Errors
├── application/               # Application Layer
│   ├── dto.rs                 # Data Transfer Objects
│   └── services/              # Application Services
├── infrastructure/            # Infrastructure Layer
│   └── persistence/           # SQLite Repository Implementation
└── commands/                  # Tauri Command Handlers
```

---

## Phase 0: DDD Domain Modeling (Primary Task)

### 0.1 Ubiquitous Language

Establish a common language through discussion. Below is the preliminary proposal:

#### Core Domain Terms

| Term | Definition |
|------|------------|
| **Item** | A file or directory indexed in the file system; the carrier of tags. |
| **Tag** | A user-defined marker used to categorize Items. |
| **Tag Group** | A logical grouping of Tags, possessing color and sort order. |
| **Tag Template** | A pre-defined combination of tags for quick application. |
| **Tagging** | The action of attaching a tag to an item. |
| **Collection** | (Future) A user-curated collection of items. |

#### Operations

| Term | Definition |
|------|------------|
| **Index** | Tracking a file in the system. |
| **Apply Tag** | Attaching a tag to an item. |
| **Remove Tag** | Removing a tag from an item. |
| **Delete** | Permanently removing an item from the index. | |

### 0.2 Business Rules

#### Item Rules
- [x] Item `path` must be unique.
- [x] Item can be a file or a directory.
- [x] Item can have zero or multiple Tags.
- [x] When a file is moved/renamed in the filesystem, its Item becomes orphaned (future: tracking mechanism will auto-update).

#### Tag Rules
- [x] Tag must belong to one Tag Group.
- [x] Tag values within the same Tag Group must be unique (case-insensitive).
- [x] Different Tag Groups can have Tags with the same name.
- [x] Tag value cannot be empty or whitespace only.
- [x] When a Tag is deleted, its association with all Items must be cleared (CASCADE).

#### Tag Group Rules
- [x] Tag Group name must be unique.
- [x] Deleting a Tag Group cascades to delete all its Tags.
- [x] Tag Group has a `display_order` for UI sorting.

#### Tag Template Rules
- [x] Template can include Tags from different Groups.
- [x] Applying a Template adds all its Tags to the Item.

### 0.3 Aggregate Boundaries

```
┌─────────────────────────────────────────┐
│           TagGroup Aggregate            │
│  ┌─────────────┐                        │
│  │  TagGroup   │ (Aggregate Root)       │
│  │  - id       │                        │
│  │  - name     │                        │
│  │  - color    │ ← Value Object         │
│  │  - order    │                        │
│  └─────────────┘                        │
│         │                               │
│         │ contains                      │
│         ▼                               │
│  ┌─────────────┐                        │
│  │    Tag      │ (Entity)               │
│  │  - id       │                        │
│  │  - value    │ ← Value Object         │
│  └─────────────┘                        │
│         │                               │
│         ▼                               │
│  ┌─────────────┐                        │
│  │ TagTemplate │ (Aggregate Root)       │
│  │  - id       │                        │
│  │  - name     │                        │
│  │  - tag_ids[]│ ← References           │
│  └─────────────┘                        │
└─────────────────────────────────────────┘

┌─────────────────────────────────────────┐
│            Item Aggregate               │
│  ┌─────────────┐                        │
│  │    Item     │ (Aggregate Root)       │
│  │  - id       │                        │
│  │  - path     │ ← Value Object         │
│  │  - metadata │                        │
│  │  - tags[]   │ ← References (IDs)     │
│  └─────────────┘                        │
└─────────────────────────────────────────┘
```

**Design Principles**:
- Tag and TagGroup form an aggregate because Tag lifecycle depends on TagGroup.
- Item is an independent aggregate, referencing Tag via ID (eventual consistency).
- TagTemplate is an independent aggregate, holding a list of Tag IDs.

### 0.4 Domain Events (Future)

The project has not yet implemented Domain Events, but the design is reserved for future expansion:

```rust
// domain/events/mod.rs (Future Implementation)

pub trait DomainEvent: Send + Sync {
    fn event_type(&self) -> &'static str;
    fn occurred_at(&self) -> i64;
}

// Example Events
pub struct ItemIndexed { pub item_id: i64, pub path: String }
pub struct TagApplied { pub item_id: i64, pub tag_id: i64 }
pub struct TagRemoved { pub item_id: i64, pub tag_id: i64 }
pub struct ItemSoftDeleted { pub item_id: i64 }
```

**Usage**:
- Trigger re-indexing when file monitoring detects changes.
- Statistics on tag usage.
- Future implementation of Undo/Redo features.

### 0.5 Discussion Questions (Resolved)

All questions have been confirmed:

1. **Item & Tag Relationship** ✅
   - Decision: Many-to-Many via `item_tags` junction table.
   - No need to record "who" or "when" (single user, UI-based tagging).

2. **Tag Uniqueness Scope** ✅
   - Decision: Unique within the same Group.
   - Different Groups CAN have Tags with the same name (e.g., "Japan" in "Location" and "Cuisine").

3. **Soft Delete Strategy** ✅
   - Decision: No soft delete. Items are permanently deleted.
   - Removed: `is_deleted`, `deleted_at` fields from Item entity.

4. **File Move/Rename Handling** ✅
   - Decision: Currently, moved/renamed files become orphaned (Item path invalid).
   - Future: Tracking mechanism (file hash) will auto-update paths.

---

## Phase 1.x: Complete UI for v1.0 Release

### 1.1 Tag Management Polish

**Status**: ✅ Complete

| Task | Description | Status |
|------|-------------|--------|
| 1.1.1 | Display usage count as a badge | ✅ |
| 1.1.2 | Hover tooltip: "Used by X files" | ✅ |
| 1.1.3 | Autocomplete with 300ms debounce | ✅ |
| 1.1.4 | Global tag search (cross-group) | ✅ |
| 1.1.5 | Keyboard arrow navigation for suggestions | ✅ |

### 1.2 Search UI Enhancement

**Status**: ✅ Complete

| Task | File | Description | Status |
|------|------|-------------|--------|
| 1.2.1 | `FilterPanel.vue` | Improve UI layout and visual hierarchy | ✅ |
| 1.2.2 | `FilterPanel.vue` | Highlight matching text in search results | ✅ |
| 1.2.3 | `FilterPanel.vue` | Recent search history (requires detailed mode) | ✅ |
| 1.2.4 | `FilterPanel.vue` | "Clear all filters" button | ✅ |
| 1.2.5 | `FilterPanel.vue` | **BUG**: Click result should navigate to file's folder | ✅ Fixed |

### 1.3 Picture View (New Page)

**Status**: ✅ Complete

**Goal**: Browse image files in a grid/gallery mode.

**Tasks**:

| Task | File | Description | Status |
|------|------|-------------|--------|
| 1.3.1 | `components/PictureView/PictureGrid.vue` | Create image grid component. | ✅ |
| 1.3.2 | `components/PictureView/PictureCard.vue` | Single image card component. | ✅ |
| 1.3.3 | `stores/pictureView.ts` | Picture View state management. | ✅ |
| 1.3.4 | `LeftPanel.vue` | Add Picture View Tab. | ✅ |
| 1.3.5 | `PictureGrid.vue` | Virtual scrolling support. | ✅ |
| 1.3.6 | `PictureCard.vue` | Lazy loading for images. | ✅ |
| 1.3.7 | `PictureCard.vue` | Click to show large image (Lightbox). | ✅ |

**Technical Notes**:
- Use `vue-virtual-scroller` for large numbers of images.
- Use `<img loading="lazy">` for thumbnails.
- Use modal overlay for large image preview.

### 1.4 v1.0 Release Checklist

- [x] Phase 1.1 Complete
- [x] Phase 1.2 Complete
- [x] Phase 1.3 Complete
- [x] Full Manual Testing
- [x] Fix all P0/P1 bugs
- [x] Update README.md
- [X] Create Release Notes
- [X] `cargo build --release`
- [X] Create GitHub Release

---

## Phase 2+: Future Features

### 2.1 Advanced Tag Search (JQL-like Query)

**Goal**: Custom query syntax similar to Jira JQL.

**Example Queries**:
```
tag = "vacation" AND tag = "2024"
name ~ "*.jpg" OR name ~ "*.png"
size > 10MB AND modified > "2024-01-01"
tag IN ("work", "project") AND NOT tag = "archived"
```

**Implementation Approach**:
1. **Parser**: Use `pest` crate to define grammar.
2. **AST**: Build Query Abstract Syntax Tree.
3. **Executor**: Convert to SQLite query or FTS5 search.

### 2.2 Image & Video Preview (Windows API)

**Goal**: Display file thumbnail previews.

**Technical Approach**:
- Use `windows` crate to call `IShellItemImageFactory::GetImage`.
- Extract thumbnails in background threads.
- Cache thumbnails in SQLite BLOB or disk.

### 2.3 File System Monitoring (NTFS USN Journal)

**Goal**: Monitor file system changes and auto-update index.

**Technical Approach**:
- Use `usn-journal-rs` to read NTFS USN Journal.
- Requires Administrator privileges.
- Polling or subscription mode to detect changes.

### 2.4 System Tray & Background Mode

**Goal**: Minimize to system tray, run in background.

**Features**:
- Tray icon click to show/hide main window.
- Context menu: Show, Settings, Quit.
- Single Instance (prevent duplicate processing).
- Optional: Start on boot.

---

## Priority Summary

| Phase | Priority | Status | Description |
|-------|----------|--------|-------------|
| 0 | P0 | 🟢 Complete | DDD Domain Modeling |
| 1.1 | P1 | 🟡 Partial | Tag Management Polish |
| 1.2 | P1 | � Complete | Search UI Enhancement |
| 1.3 | P1 | 🟢 Complete | Picture View |
| 1.4 | P1 | 🔴 Blocked | v1.0 Release |
| 2.1 | P2 | 🔴 Future | JQL-like Query |
| 2.2 | P2 | 🔴 Future | Thumbnails |
| 2.3 | P2 | 🔴 Future | USN Monitoring |
| 2.4 | P2 | 🔴 Future | System Tray |

---

## Next Steps

1. ~~**Confirm Domain Model (Phase 0)**~~ ✅ Complete

2. **Start Phase 1.1 → 1.2 → 1.3**
   - Phase 1.1: Tag Management Polish
   - ~~Phase 1.2: Search UI Enhancement~~
   - ~~Phase 1.3: Picture View~~

3. **v1.0 Release after Phase 1 completion**

---

## References

### Internal Documentation
- `docs/AGENTS_CONSTITUTION.md` - Coding Standards
- `docs/TECH_STACK_DOCUMENTATION.md` - Tech Stack Reference

### External Resources
- [Tauri 2.x System Tray](https://v2.tauri.app/learn/system-tray/)
- [pest.rs Parser](https://pest.rs/)
- [SQLite FTS5](https://sqlite.org/fts5.html)
- [usn-journal-rs](https://crates.io/crates/usn-journal-rs)
- [windows-rs IShellItemImageFactory](https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/UI/Shell/struct.IShellItemImageFactory.html)
