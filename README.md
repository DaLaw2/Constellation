# Constellation

<h3 align="center">Organize your universe of files.</h3>

<p align="center">
  <img src="assets/preview_01.png" alt="Constellation Main View" width="800" />
</p>

Constellation is a Windows desktop application that breaks free from the limitations of traditional folder hierarchies. Using a flexible tag system, you can classify your files across multiple dimensions simultaneously—without being forced into a single organizational structure.

---

## 💡 The Problem with Folders

Traditional folder structures force you to choose a single classification dimension:

```
Option A: By Language           Option B: By Author
├─ Japanese/                    ├─ Author A/
├─ English/                     ├─ Author B/
└─ Chinese/                     └─ Author C/
   └─ Some Work/                   └─ Some Work/
      ❌ Can't find by author         ❌ Can't find by language
```

**You can't answer questions like:**
- "Show me all Japanese works by Author A that are full-color"
- "Find all original works in English"
- "List everything by Author B that's complete"

---

## ✨ The Constellation Solution

### Multi-Dimensional Tagging
Tag any folder or file with multiple attributes:
- **Language**: Japanese, English, Chinese
- **Author**: Creator names
- **Work**: Series or project names  
- **Attributes**: Full-color, Original, Complete, etc.

Then search using **Boolean logic** (AND/OR) to find exactly what you need:
- `Japanese AND Author A AND Full-Color`
- `Author B AND (Original OR Fan Work)`

### Non-Invasive Design
- ✅ **No filename modifications**
- ✅ **No extra files in your folders**
- ✅ **All tags stored in a separate SQLite database**
- ✅ **Your original file structure stays untouched**

---

## 📸 Screenshots

| **Tag Management** | **Advanced Search** |
|:---:|:---:|
| <img src="assets/preview_02.png" width="400" /> | <img src="assets/preview_03.png" width="400" /> |
| *Batch operations with multi-select support* | *Filter by tags, file types, size, and date* |

---

## 🎯 Key Features

### 🏷️ Flexible Tag System
- **Tag Groups**: Organize tags into categories (Language, Author, Status, etc.)
- **Color Coding**: Assign colors to tag groups for quick visual identification
- **Tag Templates**: Save frequently used tag combinations for one-click application
- **Auto-Complete**: Suggestions while typing to avoid duplicates

### 🔍 Powerful Search
- **CQL Search Engine**: Full boolean syntax with `AND`, `OR`, `NOT`, and parentheses
- **Mixed Queries**: Combine tag filters, filename patterns, size, and date conditions
- **Real-Time Filtering**: Click tags in the sidebar to instantly filter results
- **Search History**: Automatically saves recent queries for quick reuse

### 🖼️ Media Preview
- **Thumbnail Generation**: Image and video thumbnails with disk caching
- **Configurable Cache**: Adjustable cache size with automatic eviction
- **Preview Settings**: Dedicated settings tab to configure thumbnail behavior

### 📁 Automatic File Tracking
- **Path Sync**: Detects file renames, moves, and deletions automatically
- **Cross-Volume Detection**: Tracks files moved between different drives
- **NTFS Journal Integration**: One-click enable for drives without Change Journal

### 🗂️ Integrated File Browser
- **Dual-Panel Interface**:
  - Left: Toggle between file tree or tag management
  - Right: File list with tag information
- **Expanded Panels**: Full-featured tag management and advanced search views
- **Multiple View Modes**: Detail view, large icon view, and picture grid view
- **Smooth Performance**: Virtual scrolling handles thousands of files effortlessly

### ⚙️ Settings
- **File Tracking**: Configure auto-refresh and per-drive status
- **Preview Settings**: Thumbnail cache size and behavior
- **About**: Version info and license

---

## 🆚 Why Not Use...?

| Solution | Limitation |
|----------|-----------|
| **TagSpaces** | Modifies filenames or creates sidecar files that clutter your folders |
| **Eagle** | Forces you to import files, breaking your original folder structure |
| **Obsidian/Notion** | Can only create references, can't operate on actual files |
| **Directory Opus** | Expensive, complex, replaces your entire file manager |
| **macOS Finder Tags** | macOS only, tags frequently lost when moving files |

**Constellation** keeps your folders clean, works directly with files, and is built specifically for Windows.

---

## 🛠️ Tech Stack

- **Core**: [Rust](https://www.rust-lang.org/) + [Tauri 2.0](https://tauri.app/)
- **Frontend**: [Vue 3](https://vuejs.org/) + TypeScript + Pinia
- **Database**: SQLite (via rusqlite)
- **Platform**: Windows 10+ (optimized for Windows-specific features)

---

## 🚀 Getting Started

### For Users
Download the latest release from the [Releases](https://github.com/DaLaw2/Constellation/releases) page.

**System Requirements:**
- Windows 10 or later
- 150MB available disk space

### For Developers

**Prerequisites:**
- [Rust](https://rustup.rs/) (latest stable)
- [Node.js](https://nodejs.org/) (v18+)

**Installation:**

```bash
# 1. Clone the repository
git clone https://github.com/DaLaw2/Constellation.git
cd Constellation

# 2. Install dependencies
npm install

# 3. Run in development mode
cargo tauri dev
```

**Building:**

```bash
cargo tauri build
```

The executable will be in `src-tauri/target/release/`.

---

## 📝 Known Limitations

- Windows only (utilizes Windows-specific APIs like NTFS Change Journal)

---

## 📄 License

GNU GPLv3

---

## 👏 Acknowledgements

- App icons designed by [Freepik](https://www.flaticon.com/) from [Flaticon](https://www.flaticon.com/)