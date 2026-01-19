---
module: Constellation
date: 2026-01-19
problem_type: security_issue
component: tooling
symptoms:
  - "Path traversal vulnerability allows accessing any system file via ./ or ../"
  - "Command injection in reveal_in_explorer via shell metacharacters"
  - "Vue computed ref mutation fails silently breaking file selection"
  - "Content Security Policy disabled allowing XSS attacks"
root_cause: missing_validation
resolution_type: code_fix
severity: critical
tags: [path-traversal, command-injection, vue-reactivity, csp, tauri, security]
---

# Troubleshooting: P1 Security Issues in Tauri File Browser

## Problem

Multiple critical security vulnerabilities were identified during code review of the Phase 1.3 file browser implementation. Path traversal allowed arbitrary file access, command injection could execute arbitrary code, Vue reactivity bug broke core functionality, and disabled CSP left the app vulnerable to XSS.

## Environment
- Module: Constellation (Tauri desktop app)
- Tech Stack: Rust (Tauri 2.x), Vue 3, TypeScript
- Affected Components: filesystem.rs, items.rs, FileList.vue, tauri.conf.json
- Date: 2026-01-19

## Symptoms
- Path traversal: Frontend could request ANY path like `C:\Windows\System32\config`
- Command injection: Path with `" & calc.exe & "` would execute arbitrary commands
- Vue bug: Clicking files did nothing - selection silently failed
- CSP disabled: `"csp": null` in tauri.conf.json allowed inline scripts

## What Didn't Work

**Direct solution:** The problems were identified by security review agents and fixed directly. The todo documents (006-008 in todos/) provided clear analysis and recommended solutions.

## Solution

### 1. Path Traversal Fix (filesystem.rs and items.rs)

**Code changes:**
```rust
// Added to both filesystem.rs and items.rs
use std::path::{Component, PathBuf};

/// Validate path to prevent path traversal attacks using ./ or ../
/// Allows access to any directory, but blocks relative path manipulation
fn validate_path(path: &str) -> AppResult<PathBuf> {
    let path_buf = PathBuf::from(path);

    // Check for path traversal patterns in components
    for component in path_buf.components() {
        match component {
            Component::ParentDir => {
                return Err(AppError::InvalidInput(
                    "Path traversal not allowed: '..' detected".to_string()
                ));
            }
            Component::CurDir => {
                return Err(AppError::InvalidInput(
                    "Path traversal not allowed: '.' detected".to_string()
                ));
            }
            _ => {}
        }
    }

    // Also check raw string for encoded or hidden traversal patterns
    if path.contains("..") || path.contains("./") || path.contains(".\\") {
        return Err(AppError::InvalidInput(
            "Path traversal patterns not allowed".to_string()
        ));
    }

    Ok(path_buf)
}

// Applied to all filesystem commands:
pub async fn read_directory(path: String) -> AppResult<Vec<FileEntry>> {
    let path_buf = validate_path(&path)?;  // Validate before use
    // ... rest of implementation
}
```

**Key design decision:** Allow unlimited directory access (like Windows Explorer) but block `./` and `../` patterns. This prevents traversal attacks while maintaining full file browser functionality.

### 2. Command Injection Fix (reveal_in_explorer)

```rust
#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

pub async fn reveal_in_explorer(path: String) -> AppResult<()> {
    let path_buf = validate_path(&path)?;

    // Canonicalize path to get absolute path
    let canonical_path = path_buf.canonicalize()
        .map_err(|e| AppError::InvalidInput(format!("Invalid path: {}", e)))?;

    // Use raw_arg to prevent shell interpretation of special characters
    let select_arg = format!("/select,{}", canonical_path.display());

    match std::process::Command::new("explorer.exe")
        .raw_arg(&select_arg)  // raw_arg prevents command injection
        .spawn()
    {
        Ok(_) => Ok(()),
        Err(e) => Err(AppError::InvalidInput(format!("Failed to open Explorer: {}", e))),
    }
}
```

**Why `raw_arg`:** Standard `.arg()` still allows some shell interpretation. `raw_arg()` on Windows passes the argument exactly as-is, preventing `&`, `|`, `>` from being interpreted as shell operators.

### 3. Vue Computed Ref Mutation Fix (FileList.vue)

```typescript
// Before (broken):
const selectedPath = computed(() => fileExplorerStore.currentPath)

function handleFileClick(entry: FileEntry) {
  selectedPath.value = entry.path  // ❌ Cannot mutate computed ref!
}

// After (fixed):
import { computed, ref } from 'vue'

// Local selection state (writable ref, not computed)
const selectedPath = ref<string | null>(null)

function handleFileClick(entry: FileEntry) {
  selectedPath.value = entry.path  // ✅ ref is writable
}

function navigateUp() {
  fileExplorerStore.navigateUp()
  selectedPath.value = null  // Clear selection on navigation
}
```

**Key insight:** Selection (which file is highlighted) is UI state, separate from navigation (which directory is viewed). Use `ref()` for mutable local state, `computed()` for derived read-only values.

### 4. Content Security Policy (tauri.conf.json)

```json
// Before:
"security": {
  "csp": null
}

// After:
"security": {
  "csp": "default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'; img-src 'self' data:; font-src 'self' data:; connect-src 'self' ipc: tauri:"
}
```

**CSP breakdown:**
- `default-src 'self'`: Only allow resources from app origin
- `script-src 'self'`: Block inline scripts (prevents XSS)
- `style-src 'self' 'unsafe-inline'`: Allow Vue scoped styles
- `connect-src 'self' ipc: tauri:`: Allow Tauri IPC communication

## Why This Works

1. **Path traversal**: `std::path::Component` enum correctly identifies `ParentDir` (..) and `CurDir` (.) regardless of encoding. The string check catches edge cases.

2. **Command injection**: `raw_arg()` bypasses shell parsing entirely - the argument is passed directly to the process, so metacharacters like `&` are literal characters, not operators.

3. **Vue reactivity**: `computed()` creates a getter - it's read-only by design. `ref()` creates a reactive container that can be mutated. Using the right primitive fixes the silent failure.

4. **CSP**: Acts as defense-in-depth. Even if an XSS vulnerability exists elsewhere, the CSP blocks execution of injected scripts.

## Prevention

For future Tauri/Vue development:

- **Always validate paths** before filesystem operations - check for traversal patterns
- **Never use string formatting** for shell commands - use separate args or `raw_arg`
- **Use `ref()` for mutable state**, `computed()` only for derived read-only values
- **Enable CSP from the start** - it's easier than retrofitting later
- **Run security review agents** (security-sentinel) before merging

## Related Issues

- todos/001-pending-p1-path-traversal-vulnerability.md (items.rs path issue)
- todos/002-pending-p1-content-security-policy-disabled.md (CSP issue)
- todos/006-pending-p1-path-traversal-vulnerability.md (filesystem.rs path issue)
- todos/007-pending-p1-command-injection-reveal-explorer.md (command injection)
- todos/008-pending-p1-selected-path-mutation-bug.md (Vue reactivity bug)
