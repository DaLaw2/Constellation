# Work Plan: Gemini - Phase 1.7 Polish & Enhancements

## Overview
- **Agent**: Gemini
- **Phase**: 1.7 - Polish & Bug Fixes
- **Branch**: `feat/phase1.7-polish`
- **Goal**: Address known bugs, improve UX, and implement requested UI refinements.

---

## Todo List

### 🐛 Critical Bugs
- [ ] **Fix Tag Group / Tag Deletion**: Investigate and fix intermittent deletion failures.
- [ ] **Fix Self-Duplicate Error**: Tag renaming fails if the new name is the same as the old name (validation logic error).
- [ ] **Fix Tag Group Reordering**: Drag and drop reordering is not persisting or reflecting correctly.

### 🎨 UI/UX Polish
- [ ] **Remove Focus Outlines**: fix Tag Group toggle button (chevron) showing black border on click.
- [ ] **Redesign "Click to add tags"**: Overhaul the visual design of the tag adder and tag display in the file view.
- [ ] **Disable Text Selection**: Global CSS to prevent text selection (except in inputs).
- [ ] **Restrict Global Context Menu**: Disable default right-click menu, allow only specific custom menus (Group/Tag).
- [ ] **Icons**: Support for adding custom icons (impl details TBD).

### 🚀 Missing Features / Verification
- [ ] **Verify Search**: Confirm search functionality (Phase 1.6) is working or implement if missing.
- [ ] **Pic View Mode**: Fix or implement Picture View mode.
- [ ] **Tag Group Expansion Black Border**: Double check any remaining borders.

---

## Notes
- **Tag Validation**: The `isDuplicate` check needs to exclude the current ID being edited.
- **Search**: Phase 1.6 marked search as complete, but user reports it missing/functional. precise verification needed.
