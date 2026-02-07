# Version 2.0 UI Design Specification

This document outlines the UI/UX design for the Search/Filter and Tag Management features in Version 2.0.

---

## 1. Search/Filter UI Design

### Core Concept
Move search functionality from the left sidebar to the **top primary area**, providing more space for complex JQL queries.

### Main Components

#### A. Primary Search Bar
**Location**: Top of the window, spanning the main content area

**Components**:
- **Input Field**: Supports plain text JQL input
- **Filter Chips**: Selected conditions displayed as removable chips
- **Mode Toggle Button**:
  - `Simple Mode`: Visual selection
  - `JQL Mode`: Direct JQL syntax input
- **Advanced Filter Button**: Expands the Filter Panel

#### B. Filter Panel (Expandable/Collapsible)
**Expansion Method**: Slides down from the top when "Advanced Filter" is clicked

**Layout** (Horizontal Sections):

```
┌─────────────────────────────────────────────────────┐
│ [Tag Groups]          [File Types]    [Attributes]  │
│ ┌─────────────┐      ┌──────────┐    ┌──────────┐  │
│ │ Group: Work │      │ □ Image  │    │ Size:    │  │
│ │ ☑ Project   │      │ □ Video  │    │ [Slider] │  │
│ │ □ Meeting   │      │ □ Doc    │    │          │  │
│ │ + More...   │      └──────────┘    │ Modified:│  │
│ └─────────────┘                      │ [Date]   │  │
│                                      └──────────┘  │
└─────────────────────────────────────────────────────┘
```

**Features**:
- **Tag Groups Area**: Each group displays the top 5-10 frequently used tags, click "More" to expand full list
- **Real-time Sync**: Each selection updates the search bar chips or JQL text above
- **Quick Clear**: Provides a "Clear All Filters" button

#### C. Search Results Area
**Display**:
- Result count: `Found 127 items`
- Sort options: `Sort by: Name | Date | Size | Relevance`
- Highlight matches: Search keywords highlighted in results

### Design Principles
- Prioritize **speed** and **visibility**
- Provide visual aids for complex queries
- Maintain bidirectional sync between JQL text and GUI

---

## 2. Tag Management UI Design

### Core Concept
Independent Tag management interface, focused on "maintaining" rather than "using" tags.

### Entry Points
- "Tag Management" tab in the left sidebar
- Or accessible from Settings

### Main Layout

```
┌─────────────────────────────────────────────────────┐
│ Tag Management                    [+ New Group]     │
├─────────────────────────────────────────────────────┤
│ ┌─────────────┐  ┌─────────────────────────────┐   │
│ │ Tag Groups  │  │ Tags in "Work" Group        │   │
│ │             │  │ ┌─────────────────────────┐ │   │
│ │ ▼ Work (15) │  │ │ Tag Name    │ Usage │ ⋮ │ │   │
│ │ ▼ Personal  │  │ ├─────────────────────────┤ │   │
│ │ ▼ Location  │  │ │ Project     │  45   │ ⋮ │ │   │
│ │             │  │ │ Meeting     │  12   │ ⋮ │ │   │
│ │             │  │ │ Archive     │   3   │ ⋮ │ │   │
│ │             │  │ └─────────────────────────┘ │   │
│ │             │  │ [+ Add Tag]                 │   │
│ └─────────────┘  └─────────────────────────────┘   │
└─────────────────────────────────────────────────────┘
```

### Functional Blocks

#### A. Left Side: Tag Group List
- **Collapsible Group List**, showing the number of tags in each group
- **Drag & Drop Sorting**: Adjust the display order of groups
- **Right-click Menu**:
  - Rename Group
  - Change Color
  - Delete Group (requires confirmation)

#### B. Right Side: Tag Detail List (Table View)
**Columns**:
- Tag Name (directly editable)
- Usage Count (number of uses, click to view associated files)
- Actions (⋮ menu):
  - Edit
  - Merge with... (merge into another tag)
  - Move to Group...
  - Delete

**Batch Operations**:
- Support multi-select tags
- Batch move to other groups
- Batch delete

#### C. Tag Template Management (Separate Tab)
```
┌─────────────────────────────────────────────────────┐
│ Templates                         [+ New Template]  │
├─────────────────────────────────────────────────────┤
│ Template Name         │ Tags                │ ⋮    │
├───────────────────────┼─────────────────────┼──────┤
│ Work Project          │ [Work][Project][Q1] │ ⋮    │
│ Vacation Photos       │ [Personal][Travel]  │ ⋮    │
└─────────────────────────────────────────────────────┘
```

**Features**:
- Create commonly used tag combinations
- Quick apply to files (select template in file detail page)

### Design Principles
- Prioritize **organization** and **maintenance**
- Provide batch operation capabilities
- Clearly display tag usage (avoid accidental deletion)

---

## 3. Settings Page Design

### Core Concept
A centralized settings interface with **extensible tab-based navigation**. Initial implementation focuses on the **About** section, with the architecture designed to easily accommodate future settings categories.

### Main Layout

```
┌─────────────────────────────────────────────────────┐
│ Settings                                      [✕]   │
├───────────┬─────────────────────────────────────────┤
│           │                                         │
│ About     │         [Content Area]                  │
│           │                                         │
│ ─────     │                                         │
│ (Future)  │                                         │
│ General   │                                         │
│ Indexing  │                                         │
│ Appearance│                                         │
│           │                                         │
└───────────┴─────────────────────────────────────────┘
```

### Architecture Design

#### A. Left Sidebar (Tab Navigation)
**Structure**:
- **Active Tabs**: Currently implemented settings categories (initially only "About")
- **Separator Line**: Visual divider
- **Future Tabs**: Grayed-out/disabled tabs showing planned categories

**Extensibility**:
```typescript
interface SettingsTab {
  id: string;           // 'about', 'general', 'indexing', etc.
  label: string;        // Display name
  icon?: string;        // Optional icon
  enabled: boolean;     // Whether the tab is implemented
  component: Component; // Vue component to render
}
```

**Benefits**:
- Users can see the roadmap of future settings
- Easy to add new tabs by registering new `SettingsTab` objects
- No layout changes needed when adding features

#### B. Content Area (Right Panel)
**Behavior**:
- Dynamically renders the component associated with the selected tab
- Scrollable if content exceeds viewport height
- Consistent padding and spacing

### Initial Implementation: About Tab

```
┌─────────────────────────────────────────────────────┐
│ Settings                                      [✕]   │
├───────────┬─────────────────────────────────────────┤
│           │ About Constellation                     │
│ ● About   │                                         │
│           │ ┌─────────────────────────────────┐     │
│ ─────     │ │      [App Icon/Logo]            │     │
│           │ │                                 │     │
│ General   │ │   Constellation                 │     │
│ Indexing  │ │   Version 2.0.0                 │     │
│ Appearance│ │   Build: 2026-01-30             │     │
│ Performance│ └─────────────────────────────────┘     │
│ Advanced  │                                         │
│           │ Technology Stack                        │
│           │ • Tauri 2.x                             │
│           │ • Vue 3 + TypeScript                    │
│           │ • Rust 1.75+                            │
│           │                                         │
│           │ Links                                   │
│           │ [GitHub Repository]                     │
│           │ [Report an Issue]                       │
│           │ [Documentation]                         │
│           │                                         │
│           │ Updates                                 │
│           │ ☐ Automatically check for updates      │
│           │ [Check for Updates Now]                 │
│           │                                         │
│           │ License                                 │
│           │ MIT License © 2024-2026                 │
└───────────┴─────────────────────────────────────────┘
```

### About Tab Content Specification

**Version Information**:
- App name: "Constellation"
- Version number: Read from `package.json` or Tauri config
- Build date: Embedded at compile time
- Optional: Git commit hash (for dev builds)

**Technology Stack**:
- List major dependencies with versions
- Tauri version
- Vue version
- Rust version (used to compile)

**Links** (All open in external browser):
- GitHub Repository: `https://github.com/DaLaw2/Constellation`
- Report an Issue: `https://github.com/DaLaw2/Constellation/issues/new`
- Documentation: Link to docs (if available)

**Updates**:
- Checkbox: "Automatically check for updates" (stored in settings)
- Button: "Check for Updates Now" (triggers update check via Tauri updater)
- Display current status: "Up to date" or "Update available: v2.1.0"

**License**:
- Display license type and copyright year

### Future Settings Categories (Planned)

#### General
- Language selection
- Theme (Light/Dark/Auto)
- Startup behavior

#### Indexing
- Watched folders management
- Auto-index settings
- Database maintenance tools

#### Appearance
- Picture view settings (thumbnail size, grid columns)
- File list settings (row height, date format)

#### Performance
- Cache settings
- Search performance options

#### Advanced
- Data export/import
- Developer options
- Database location

---

## Implementation Notes

### Search/Filter
- The Filter Panel should be implemented as a collapsible component
- JQL parser should validate syntax in real-time
- Filter chips should be interactive (click to edit, X to remove)

### Tag Management
- Table view should support inline editing
- Drag & drop should provide visual feedback
- Usage count should be clickable to show related items
- Deletion should require confirmation if usage > 0

---

## Next Steps

1. Create wireframe mockups for both interfaces
2. Define component hierarchy for Vue implementation
3. Specify state management requirements
4. Design API contracts for new backend queries
