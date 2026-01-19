# Constellation

A Windows desktop application for tagging and organizing files using a flexible tag system.

## Features

- **File Browser** - Navigate drives and directories with lazy-loaded tree view
- **Tag System** - Organize files with hierarchical tag groups and tags
- **Inline Tagging** - Click on any file to add/remove tags directly
- **Tag Templates** - Save and apply common tag combinations
- **Virtual Scrolling** - Handle directories with thousands of files smoothly

## Tech Stack

- **Backend**: Rust + Tauri 2.0
- **Frontend**: Vue 3 + TypeScript + Pinia
- **Database**: SQLite (via rusqlite + deadpool)
- **Build**: Vite

## Prerequisites

- [Rust](https://rustup.rs/) (latest stable)
- [Node.js](https://nodejs.org/) (v18+)
- [pnpm](https://pnpm.io/) or npm

## Development

```bash
# Install frontend dependencies
cd frontend
npm install

# Run in development mode (from project root)
cargo tauri dev
```

## Build

```bash
# Build for production
cargo tauri build
```

The installer will be in `target/release/bundle/`.

## Project Structure

```
constellation/
├── src/                    # Rust backend
│   ├── commands/           # Tauri IPC commands
│   │   ├── filesystem.rs   # Drive/directory operations
│   │   ├── items.rs        # File item CRUD + tagging
│   │   ├── tags.rs         # Tag CRUD
│   │   ├── tag_groups.rs   # Tag group CRUD
│   │   └── tag_templates.rs# Template management
│   ├── db/                 # Database layer
│   │   ├── schema.rs       # SQLite schema
│   │   └── models.rs       # Data models
│   ├── error.rs            # Error types
│   └── lib.rs              # App initialization
├── frontend/               # Vue frontend
│   ├── components/
│   │   ├── FileExplorer/   # File browser components
│   │   ├── TagManagement/  # Tag editing components
│   │   └── Layout/         # App layout
│   ├── stores/             # Pinia stores
│   └── composables/        # Vue composables
└── tauri.conf.json         # Tauri configuration
```

## Database Schema

- **tag_groups** - Categories for tags (e.g., "Language", "Status")
- **tags** - Individual tags belonging to groups
- **items** - Tracked files/folders with metadata
- **item_tags** - Many-to-many relationship between items and tags
- **tag_templates** - Saved tag combinations for quick application

## Roadmap

### Phase 1 (Current)
- [x] Database layer with SQLite
- [x] Tag group and tag management
- [x] File browser with drive enumeration
- [x] Inline tag editing
- [x] Tag templates
- [ ] Search and filter by tags
- [ ] First-run setup wizard

### Phase 2 (Planned)
- [ ] Batch operations
- [ ] Import/export tags
- [ ] Keyboard shortcuts
- [ ] Custom tag colors

## License

MIT
