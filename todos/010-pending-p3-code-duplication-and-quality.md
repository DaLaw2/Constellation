---
status: pending
priority: p3
issue_id: "010"
tags: [code-review, quality, refactoring, duplication]
dependencies: []
---

# Code Duplication and Quality Improvements

## Problem Statement

The Phase 1.3 implementation contains ~50 lines of duplicated code across multiple components, including utility functions, CSS animations, and styling. While not critical, this creates maintenance burden and violates DRY (Don't Repeat Yourself) principles.

**Why this matters:**
- Maintenance burden - changes must be made in multiple places
- Inconsistency risk - duplicates can drift out of sync
- Code bloat - unnecessary lines of code
- Developer experience - harder to find and modify shared logic

## Findings

### Source: Pattern-Recognition-Specialist + Simplicity-Reviewer

**Duplication Summary:**

| Code Block | Locations | Lines | Priority |
|------------|-----------|-------|----------|
| `formatBytes()` | DirectoryTree.vue:111-117, FileItem.vue:128-134 | 7 × 2 | Medium |
| `formatDate()` | FileItem.vue:136-151 | 16 × 1 | Low (single use) |
| Loading spinner CSS | DirectoryTree.vue, FileList.vue, DirectoryNode.vue | 12 × 3 | Low |
| Hover color `rgba(0,0,0,0.04)` | 5 components | 1 × 5 | Low |
| Magic numbers | Various locations | N/A | Low |

## Detailed Findings

### 1. formatBytes Function Duplication (MEDIUM PRIORITY)

**Locations:**
- `D:\Code\Rust\Constellation\frontend\components\FileExplorer\DirectoryTree.vue` (Lines 111-117)
- `D:\Code\Rust\Constellation\frontend\components\FileExplorer\FileItem.vue` (Lines 128-134)

**Code:**
```typescript
function formatBytes(bytes: number): string {
  if (bytes === 0) return '0 B'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return Math.round(bytes / Math.pow(k, i) * 100) / 100 + ' ' + sizes[i]
}
```

**Impact:** Medium - used in 2 places, likely to be used more in future (file metadata, search results, etc.)

### 2. formatDate Function (LOW PRIORITY - Single Use)

**Location:** `FileItem.vue` (Lines 136-151)

**Code:**
```typescript
function formatDate(timestamp: number): string {
  const date = new Date(timestamp * 1000)
  const now = new Date()
  const diff = now.getTime() - date.getTime()
  const days = Math.floor(diff / (1000 * 60 * 60 * 24))

  if (days === 0) {
    return 'Today ' + date.toLocaleTimeString('en-US', { hour: '2-digit', minute: '2-digit' })
  } else if (days === 1) {
    return 'Yesterday'
  } else if (days < 7) {
    return `${days} days ago`
  } else {
    return date.toLocaleDateString('en-US', { month: 'short', day: 'numeric', year: 'numeric' })
  }
}
```

**Impact:** Low - only used in FileItem.vue currently

### 3. Loading Spinner CSS Duplication (LOW PRIORITY)

**Locations:**
- `DirectoryTree.vue` (Lines 140-151)
- `FileList.vue` (Lines 190-201)
- `DirectoryNode.vue` (Lines 148-160)

**Code:**
```css
.loading-spinner {
  width: 32px;
  height: 32px;
  border: 3px solid var(--border-color);
  border-top-color: var(--primary-color);
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}
```

**Impact:** Low - CSS duplication, minimal maintenance burden

### 4. Hover Color Inconsistency (LOW PRIORITY)

**Locations:**
- DirectoryTree.vue:198 - `rgba(0, 0, 0, 0.04)`
- DirectoryNode.vue:103 - `rgba(0, 0, 0, 0.04)`
- **FileItem.vue:167 - `rgba(0, 0, 0, 0.02)`** ⚠️ Different!
- FileList.vue:152 - `rgba(0, 0, 0, 0.04)`
- LeftPanel.vue:74 - `rgba(0, 0, 0, 0.04)`

**Impact:** Low - visual inconsistency

### 5. Magic Numbers (LOW PRIORITY)

**Locations:**
- `filesystem.rs:48` - `for i in 0..26` (number of drive letters A-Z)
- `filesystem.rs:106` - `vec![0u16; 256]` (volume name buffer size)
- `FileList.vue:43` - `:item-size="60"` (virtual scroller item height)

**Impact:** Low - minor readability issue

## Proposed Solutions

### Solution 1: Extract Shared Utilities (RECOMMENDED)

**Approach:** Create utility module for shared functions.

**Implementation:**
```typescript
// frontend/utils/formatters.ts (NEW)

/**
 * Format bytes to human-readable string (KB, MB, GB, etc.)
 */
export function formatBytes(bytes: number): string {
  if (bytes === 0) return '0 B'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return Math.round(bytes / Math.pow(k, i) * 100) / 100 + ' ' + sizes[i]
}

/**
 * Format Unix timestamp to relative or absolute date string
 */
export function formatDate(timestamp: number): string {
  const date = new Date(timestamp * 1000)
  const now = new Date()
  const diff = now.getTime() - date.getTime()
  const days = Math.floor(diff / (1000 * 60 * 60 * 24))

  if (days === 0) {
    return 'Today ' + date.toLocaleTimeString('en-US', { hour: '2-digit', minute: '2-digit' })
  } else if (days === 1) {
    return 'Yesterday'
  } else if (days < 7) {
    return `${days} days ago`
  } else {
    return date.toLocaleDateString('en-US', { month: 'short', day: 'numeric', year: 'numeric' })
  }
}

/**
 * Format file path for display (truncate, highlight filename)
 */
export function formatPath(path: string, maxLength: number = 50): string {
  if (path.length <= maxLength) return path
  // Truncate middle: "C:\...\folder\file.txt"
  const parts = path.split('\\')
  const filename = parts[parts.length - 1]
  const drive = parts[0]
  return `${drive}\\...\\${filename}`
}
```

Then update components:
```typescript
// DirectoryTree.vue, FileItem.vue
import { formatBytes, formatDate } from '../../utils/formatters'

// Remove local implementations, use imported functions
```

**Pros:**
- ✅ Single source of truth
- ✅ Reusable across entire app
- ✅ Easy to test in isolation
- ✅ Can add more utilities (formatPath, formatFileType, etc.)

**Cons:**
- ⚠️ Need to update import statements in components

**Effort:** 1 hour
**Risk:** Low

### Solution 2: Create Shared Components

**Approach:** Extract loading spinner to reusable component.

**Implementation:**
```vue
<!-- frontend/components/common/LoadingSpinner.vue (NEW) -->
<template>
  <div :class="['loading-spinner', size]"></div>
</template>

<script setup lang="ts">
defineProps<{
  size?: 'small' | 'medium' | 'large'
}>()
</script>

<style scoped>
.loading-spinner {
  border: 3px solid var(--border-color);
  border-top-color: var(--primary-color);
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

.loading-spinner.small {
  width: 16px;
  height: 16px;
  border-width: 2px;
}

.loading-spinner.medium {
  width: 32px;
  height: 32px;
  border-width: 3px;
}

.loading-spinner.large {
  width: 48px;
  height: 48px;
  border-width: 4px;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}
</style>
```

Usage:
```vue
<LoadingSpinner size="medium" />
```

**Pros:**
- ✅ Consistent spinner across app
- ✅ Easy to customize (size, color)
- ✅ Removes CSS duplication

**Cons:**
- ⚠️ Overhead of component import
- ⚠️ May be overkill for simple spinner

**Effort:** 1 hour
**Risk:** Low

### Solution 3: Standardize CSS Variables

**Approach:** Define hover colors in global CSS variables.

**Implementation:**
```css
/* frontend/assets/styles/main.css */
:root {
  --hover-bg: rgba(0, 0, 0, 0.04);
  --hover-bg-light: rgba(0, 0, 0, 0.02);
  --hover-bg-dark: rgba(0, 0, 0, 0.08);
}
```

Then in components:
```css
.tree-node:hover {
  background: var(--hover-bg);
}

.file-item:hover {
  background: var(--hover-bg);  /* Changed from 0.02 to 0.04 */
}
```

**Pros:**
- ✅ Consistent colors
- ✅ Easy to theme
- ✅ Single place to change

**Cons:**
- ⚠️ Minimal impact (low priority)

**Effort:** 30 minutes
**Risk:** Low

### Solution 4: Extract Magic Numbers to Constants

**Approach:** Define constants for magic numbers.

**Implementation:**
```rust
// src/commands/filesystem.rs

const MAX_DRIVE_LETTERS: usize = 26;  // A-Z
const VOLUME_NAME_BUFFER_SIZE: usize = 256;  // Windows API standard

for i in 0..MAX_DRIVE_LETTERS {
    // ...
}

let mut volume_name_buffer = vec![0u16; VOLUME_NAME_BUFFER_SIZE];
```

```vue
<!-- FileList.vue -->
<script setup lang="ts">
const VIRTUAL_SCROLLER_ITEM_HEIGHT = 60
</script>

<template>
  <RecycleScroller :item-size="VIRTUAL_SCROLLER_ITEM_HEIGHT" />
</template>
```

**Pros:**
- ✅ Self-documenting code
- ✅ Easy to change

**Cons:**
- ⚠️ Minimal impact

**Effort:** 15 minutes
**Risk:** Low

## Recommended Action

**Prioritize Solution 1 (utility module), defer others**

**Rationale:**
- `formatBytes` and `formatDate` will be reused extensively
- Utility module is standard best practice
- Other duplications are low-impact and can wait

**Implementation Steps:**
1. Create `frontend/utils/formatters.ts`
2. Move `formatBytes()` and `formatDate()` into it
3. Export with JSDoc comments
4. Update DirectoryTree.vue and FileItem.vue to import
5. Add unit tests for formatters (optional but recommended)

**Defer:**
- LoadingSpinner component (low ROI)
- CSS variable standardization (minimal impact)
- Magic number extraction (nice-to-have)

## Technical Details

**Files to Create:**
- `frontend/utils/formatters.ts` (NEW)

**Files to Modify:**
- `frontend/components/FileExplorer/DirectoryTree.vue` (remove formatBytes)
- `frontend/components/FileExplorer/FileItem.vue` (remove formatBytes, optionally formatDate)

**Testing:**
```typescript
// frontend/utils/__tests__/formatters.spec.ts (NEW)
import { formatBytes, formatDate } from '../formatters'

describe('formatBytes', () => {
  it('should format 0 bytes', () => {
    expect(formatBytes(0)).toBe('0 B')
  })

  it('should format kilobytes', () => {
    expect(formatBytes(1024)).toBe('1 KB')
  })

  it('should format megabytes', () => {
    expect(formatBytes(1048576)).toBe('1 MB')
  })
})

describe('formatDate', () => {
  it('should format today', () => {
    const now = Math.floor(Date.now() / 1000)
    expect(formatDate(now)).toContain('Today')
  })
})
```

## Acceptance Criteria

**For Solution 1 (Utility Module):**
- [ ] `frontend/utils/formatters.ts` created with exported functions
- [ ] `formatBytes()` removed from DirectoryTree.vue and FileItem.vue
- [ ] Both components import from utility module
- [ ] Application builds without errors
- [ ] File size display works correctly in both tree and list views
- [ ] (Optional) Unit tests added for formatters

**For Deferred Solutions:**
- [ ] LoadingSpinner component tracked as separate enhancement
- [ ] CSS variables tracked as design system improvement
- [ ] Magic numbers tracked as code quality task

## Work Log

### 2026-01-19 - Issue Discovered
- Pattern-Recognition-Specialist identified ~50 lines of duplication
- Simplicity-Reviewer recommended extraction to utilities
- Priority: P3 (Nice-to-have, not blocking)
- Recommended phased approach: formatters first, rest later

## Resources

- **Similar Pattern:** None currently - this would be first shared utility module
- **Testing Framework:** Vitest (if project has it) or Jest
- **Code Style:** Follow existing TypeScript conventions
- **Future Utilities:** Could add formatPath, formatFileType, formatPermissions, etc.
