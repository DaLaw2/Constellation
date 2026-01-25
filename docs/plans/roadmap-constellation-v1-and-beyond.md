# Constellation Roadmap

## Overview

This document outlines the future development roadmap for the Constellation project following the **v1.0.0 Release**.
The current focus is on extending functionality with advanced search, system integration, and performance optimizations.

---

## Version 2.0 Features

### 1. Search & Filter UI Redesign

**Goal**: Redesign the interface to support advanced filtering and query construction, replacing the insufficient left panel sidebar.

**Requirements**:
- **Dedicated Search Interface**: A more capable interface (e.g., top bar with expanded mode, or a dedicated search view).
- **Interactive Query Builder**: Visual tools to build JQL queries without typing code.
- **Advanced Filter Modal**: UI for selecting specific attributes (size, date, tags) to generate queries.

### 2. Settings Page Design

**Goal**: Create a central settings management page.

**Features**:
- **General**: Theme selection (Light/Dark), Language.
- **Indexing**: Manage watched folders, manual re-index trigger.
- **Appearance**: Adjust grid density, thumbnail size.
- **About**: Version info, check for updates.

### 3. Advanced Tag Search (Backend Logic)

**Goal**: Implement the custom query syntax engine (JQL-like).

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

### 4. Image & Video Preview (Windows API)

**Goal**: Display file thumbnail previews.

**Technical Approach**:
- Use `windows` crate to call `IShellItemImageFactory::GetImage`.
- Extract thumbnails in background threads.
- Cache thumbnails in SQLite BLOB or disk.

### 5. File System Monitoring (NTFS USN Journal)

**Goal**: Monitor file system changes and auto-update index.

**Technical Approach**:
- Use `usn-journal-rs` to read NTFS USN Journal.
- Requires Administrator privileges.
- Polling or subscription mode to detect changes.

### 6. System Tray & Background Mode

**Goal**: Minimize to system tray, run in background.

**Features**:
- Tray icon click to show/hide main window.
- Context menu: Show, Settings, Quit.
- Single Instance (prevent duplicate processing).
- Optional: Start on boot.

---

## References

### Internal Documentation
- [Coding Standards](../AGENTS_CONSTITUTION.md)

### External Resources
- [Tauri 2.x System Tray](https://v2.tauri.app/learn/system-tray/)
- [pest.rs Parser](https://pest.rs/)
- [SQLite FTS5](https://sqlite.org/fts5.html)
- [usn-journal-rs](https://crates.io/crates/usn-journal-rs)
- [windows-rs IShellItemImageFactory](https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/UI/Shell/struct.IShellItemImageFactory.html)
