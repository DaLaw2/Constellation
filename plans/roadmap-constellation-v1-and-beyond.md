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
| **Soft Delete** | Marking an item as deleted without physically removing it. |
| **Restore** | Restoring a soft-deleted item. |

### 0.2 Business Rules

#### Item Rules
- [ ] Item `path` must be unique.
- [ ] Item can be a file or a directory.
- [ ] Already soft-deleted Items cannot be soft-deleted again.
- [ ] Item can have zero or multiple Tags.

#### Tag Rules
- [ ] Tag must belong to one Tag Group.
- [ ] Tag values within the same Tag Group must be unique (case-insensitive).
- [ ] Tag value cannot be empty or whitespace only.
- [ ] When a Tag is deleted, its association with all Items must be cleared.

#### Tag Group Rules
- [ ] Tag Group name must be unique.
- [ ] Deleting a Tag Group cascades to delete all its Tags.
- [ ] Tag Group has a `display_order` for UI sorting.

#### Tag Template Rules
- [ ] Template can include Tags from different Groups.
- [ ] Applying a Template adds all its Tags to the Item.

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

### 0.5 Discussion Questions

Please confirm the following:

1. **Item & Tag Relationship**
   - Current: Many-to-Many via `item_tags` junction table.
   - Question: Do we need to record "who" tagged it and "when"?

2. **Tag Uniqueness Scope**
   - Current: Unique within the same Group.
   - Question: Do we allow Tags with the same name in different Groups?

3. **Soft Delete Strategy**
   - Current: Item supports soft delete.
   - Question: Do Tag/TagGroup also need soft delete?

4. **File Move/Rename Handling**
   - Question: How to handle files being moved in the file system?
   - Option A: Re-associate via path.
   - Option B: Track via file hash (Future).

---

## Phase 1.x: Complete UI for v1.0 Release

### 1.1 Tag Management Polish (Partial)

**Current Status**: Backend ✅ | Frontend ⚠️ Partial

**Remaining Tasks**:

| Task | File | Description |
|------|------|-------------|
| 1.1.1 | `TagPanel.vue` | Display usage count as a badge. |
| 1.1.2 | `TagPanel.vue` | Hover tooltip: "Used by X files". |
| 1.1.3 | `TagPanel.vue` | Autocomplete with 300ms debounce. |
| 1.1.4 | `TagPanel.vue` | Global tag search (cross-group). |
| 1.1.5 | `TagPanel.vue` | Keyboard arrow navigation for suggestions. |

### 1.2 Search UI Enhancement

**Current Status**: Basic implementation exists

**Tasks**:

| Task | File | Description |
|------|------|-------------|
| 1.2.1 | `FilterPanel.vue` | Improve UI layout and visual hierarchy. |
| 1.2.2 | `FilterPanel.vue` | Highlight matching text in search results. |
| 1.2.3 | `FilterPanel.vue` | Recent search history. |
| 1.2.4 | `FilterPanel.vue` | "Clear all filters" button. |

### 1.3 Picture View (New Page)

**Status**: Not started

**Goal**: Browse image files in a grid/gallery mode.

**Tasks**:

| Task | File | Description |
|------|------|-------------|
| 1.3.1 | `components/PictureView/PictureGrid.vue` | Create image grid component. |
| 1.3.2 | `components/PictureView/PictureCard.vue` | Single image card component. |
| 1.3.3 | `stores/pictureView.ts` | Picture View state management. |
| 1.3.4 | `LeftPanel.vue` | Add Picture View Tab. |
| 1.3.5 | `PictureGrid.vue` | Virtual scrolling support. |
| 1.3.6 | `PictureCard.vue` | Lazy loading for images. |
| 1.3.7 | `PictureCard.vue` | Click to show large image (Lightbox). |

**Technical Notes**:
- Use `vue-virtual-scroller` for large numbers of images.
- Use `<img loading="lazy">` for thumbnails.
- Use modal overlay for large image preview.

### 1.4 v1.0 Release Checklist

- [ ] Phase 1.1 Complete
- [ ] Phase 1.2 Complete
- [ ] Phase 1.3 Complete
- [ ] Full Manual Testing
- [ ] Fix all P0/P1 bugs
- [ ] Update README.md
- [ ] Create Release Notes
- [ ] `cargo build --release`
- [ ] Create GitHub Release

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
| 0 | P0 | 🟡 In Progress | DDD Domain Modeling |
| 1.1 | P1 | 🟡 Partial | Tag Management Polish |
| 1.2 | P1 | 🟡 Partial | Search UI Enhancement |
| 1.3 | P1 | 🔴 Not Started | Picture View |
| 1.4 | P1 | 🔴 Blocked | v1.0 Release |
| 2.1 | P2 | 🔴 Future | JQL-like Query |
| 2.2 | P2 | 🔴 Future | Thumbnails |
| 2.3 | P2 | 🔴 Future | USN Monitoring |
| 2.4 | P2 | 🔴 Future | System Tray |

---

## Next Steps

1. **Confirm Domain Model (Phase 0)**
   - Is the Ubiquitous Language accurate?
   - Are Business Rules complete?
   - Are Aggregate Boundaries reasonable?

2. **Start Phase 1.1 → 1.2 → 1.3 upon confirmation**

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
