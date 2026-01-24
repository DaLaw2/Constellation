# Constellation Manual Test Checklist

Use this checklist after each development phase to ensure product quality.

---

## How to Use

1. Reset test environment: Delete `%APPDATA%/com.constellation.app/constellation.db`
2. Run through each section in order
3. Mark items: `[x]` pass, `[-]` fail, `[?]` blocked
4. Record failures with screenshots/details

---

## 1. Tag Group Management

### Basic Operations
- [ ] Create tag group with valid name
- [ ] Create tag group with color
- [ ] Create tag group without color (should use default)
- [ ] Edit tag group name
- [ ] Edit tag group color
- [ ] Delete empty tag group
- [ ] Delete tag group with tags (should cascade delete tags)
- [ ] Reorder tag groups via drag-and-drop

### Boundary Conditions
- [ ] Create tag group with 1 character name
- [ ] Create tag group with 255 character name
- [ ] Create 50+ tag groups (performance check)

### Edge Cases & Errors
- [ ] Create tag group with empty name → Should show error
- [ ] Create tag group with whitespace-only name → Should show error
- [ ] Create duplicate tag group name → Should show error
- [ ] Create tag group with special characters: `日本語`, `émoji 🏷️`, `<script>`

### UI Check
- [ ] Tag groups display in correct order
- [ ] Color badge displays correctly
- [ ] Edit/delete buttons visible on hover
- [ ] Drag handle visible and functional

---

## 2. Tag Management

### Basic Operations
- [ ] Create tag in a group
- [ ] Edit tag value
- [ ] Delete tag (should remove from all items)
- [ ] Move tag to different group (if supported)

### Boundary Conditions
- [ ] Create tag with 1 character value
- [ ] Create tag with 255 character value
- [ ] Create 100+ tags in single group (performance)
- [ ] Create tags with same name in DIFFERENT groups → Should succeed

### Edge Cases & Errors
- [ ] Create tag with empty value → Should show error
- [ ] Create tag with whitespace-only value → Should show error
- [ ] Create duplicate tag in SAME group → Should show error
- [ ] Create tag with special characters: `C++`, `C#`, `file/path`, `a\b`

### UI Check
- [ ] Tags display under correct group
- [ ] Tag count badge shows correct number
- [ ] Autocomplete suggestions appear when typing
- [ ] Selected tags highlight correctly

---

## 3. Item (File/Directory) Management

### Basic Operations
- [ ] Add file to index via file browser
- [ ] Add directory to index
- [ ] Add tag to item (single click)
- [ ] Remove tag from item
- [ ] Delete item from index
- [ ] Open file externally (double-click or button)
- [ ] Reveal in Explorer

### Boundary Conditions
- [ ] Add file with very long path (260+ chars)
- [ ] Add file with Unicode path: `D:\測試\文件.txt`
- [ ] Add 1000+ items (performance)
- [ ] Add item with 50+ tags

### Edge Cases & Errors
- [ ] Add same file twice → Should show existing item or error
- [ ] Add non-existent path → Should show error
- [ ] Add file that gets moved/renamed → Item should become orphaned
- [ ] Add file with special characters: `file (1).txt`, `file[2].txt`

### UI Check
- [ ] File icon vs folder icon displays correctly
- [ ] File path truncates gracefully if too long
- [ ] Tags on item display as chips/badges
- [ ] Item list scrolls smoothly with 1000+ items

---

## 4. Tag Template Management

### Basic Operations
- [ ] Create template with multiple tags
- [ ] Apply template to item (adds all tags)
- [ ] Edit template (add/remove tags)
- [ ] Delete template

### Boundary Conditions
- [ ] Create template with 1 tag
- [ ] Create template with 50+ tags
- [ ] Create 20+ templates

### Edge Cases & Errors
- [ ] Create template with empty name → Should show error
- [ ] Create template with no tags → Should show error or warning
- [ ] Apply template when some tags already exist on item → Should not duplicate
- [ ] Delete tag that is part of template → Template should update

### UI Check
- [ ] Template list displays all templates
- [ ] Template preview shows included tags
- [ ] Apply button clearly visible

---

## 5. Search & Filter

### Basic Operations
- [ ] Search by single tag (AND mode)
- [ ] Search by multiple tags (AND mode) - must have ALL
- [ ] Search by multiple tags (OR mode) - must have ANY
- [ ] Search by filename
- [ ] Combined: tags + filename filter
- [ ] Clear all filters

### Boundary Conditions
- [ ] Search with 10+ tags selected
- [ ] Search with very long filename query
- [ ] Search returning 1000+ results (performance)
- [ ] Search returning 0 results

### Edge Cases & Errors
- [ ] Search with no criteria → Should show all or prompt
- [ ] Search for tag that was just deleted → Should handle gracefully
- [ ] Filename search with regex characters: `*.jpg`, `file(1)`

### UI Check
- [ ] Search results update in real-time (or with debounce)
- [ ] Selected filter tags display clearly
- [ ] Result count shown
- [ ] "No results" message when empty

---

## 6. File Browser Navigation

### Basic Operations
- [ ] List all drives
- [ ] Navigate into directory
- [ ] Navigate back (parent directory)
- [ ] Navigate via breadcrumb
- [ ] Sort by name/date/size

### Boundary Conditions
- [ ] Directory with 10,000+ files (performance)
- [ ] Deeply nested path (20+ levels)
- [ ] Empty directory

### Edge Cases & Errors
- [ ] Navigate to directory without read permission → Should show error
- [ ] Navigate to network path (if supported)
- [ ] Directory deleted while viewing → Should handle gracefully

### UI Check
- [ ] File/folder icons correct
- [ ] File size formatted (KB/MB/GB)
- [ ] Modified date formatted
- [ ] Breadcrumb shows full path
- [ ] Loading indicator for slow directories

---

## 7. Cross-Feature Integration

### Workflows
- [ ] Complete workflow: Browse → Add file → Create group → Create tag → Apply tag → Search
- [ ] Bulk tagging: Select multiple files → Apply template
- [ ] Re-tagging: Remove all tags → Apply new tags

### Data Integrity
- [ ] Close and reopen app → All data persists
- [ ] Delete tag group → Items lose those tags but remain indexed
- [ ] Delete tag → Item-tag association removed

---

## 8. Performance Benchmarks

| Operation | Acceptable | Record Actual |
|-----------|------------|---------------|
| App startup | < 2s | ___s |
| Load 1000 items | < 1s | ___s |
| Search 10000 items | < 500ms | ___ms |
| Add tag to item | < 100ms | ___ms |
| Navigate directory (1000 files) | < 1s | ___s |

---

## 9. Error Handling & Recovery

- [ ] Database file locked by another process → Should show clear error
- [ ] Database file corrupted → Should show error, not crash
- [ ] Disk full when saving → Should show error
- [ ] Network disconnected (if applicable)

---

## 10. UI/UX General

### Visual
- [ ] No layout breaking on window resize
- [ ] Minimum window size enforced
- [ ] Dark/light theme (if applicable)
- [ ] All text readable (contrast, size)
- [ ] Icons consistent style

### Interaction
- [ ] Keyboard navigation works (Tab, Enter, Escape)
- [ ] Focus states visible
- [ ] Loading states shown for async operations
- [ ] Error messages clear and actionable
- [ ] Success feedback (toast/notification)

### Responsiveness
- [ ] UI remains responsive during database operations
- [ ] Cancel long operations (if applicable)
- [ ] No UI freeze > 500ms

---

## Test Session Log

**Date:** ____-__-__
**Version/Commit:** ____________
**Tester:** ____________

### Summary
- Total tests: ___
- Passed: ___
- Failed: ___
- Blocked: ___

### Failed Tests Details

| # | Section | Test | Expected | Actual | Screenshot |
|---|---------|------|----------|--------|------------|
| 1 | | | | | |
| 2 | | | | | |

### Notes
```
(Additional observations, suggestions, or issues not covered by checklist)
```
