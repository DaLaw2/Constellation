---
status: completed
priority: p1
issue_id: "007"
tags: [code-review, security, command-injection, critical]
dependencies: []
completed_at: 2026-01-19
---

# Command Injection in reveal_in_explorer Function

## Problem Statement

The `reveal_in_explorer` function constructs Windows shell commands using string formatting with unvalidated user input, allowing command injection attacks. An attacker could execute arbitrary commands by providing a malicious file path.

**Why this matters:**
- **CVSS Score: 7.3 (HIGH)**
- Allows executing arbitrary system commands
- Could lead to system compromise, data exfiltration, or malware installation
- Affects user trust in application security

## Findings

### Source: Security-Sentinel Agent

**Vulnerable Code** (`src/commands/filesystem.rs`, Line 343):
```rust
#[tauri::command]
pub async fn reveal_in_explorer(path: String) -> AppResult<()> {
    let path_buf = PathBuf::from(&path);

    if !path_buf.exists() {
        return Err(AppError::InvalidInput(format!("Path does not exist: {}", path)));
    }

    #[cfg(target_os = "windows")]
    {
        let explorer_path = "explorer.exe";
        let arg = format!("/select,\"{}\"", path_buf.display());  // ❌ VULNERABLE

        match std::process::Command::new(explorer_path)
            .arg(arg)  // Unsanitized user input
            .spawn()
        {
            Ok(_) => Ok(()),
            Err(e) => Err(AppError::InvalidInput(format!("Failed to open Explorer: {}", e))),
        }
    }
}
```

**Attack Example:**
```javascript
// Frontend provides malicious path
await invoke('reveal_in_explorer', {
    path: 'C:\\test" & calc.exe & "'
});

// Results in command:
// explorer.exe /select,"C:\test" & calc.exe & ""
//                             ^^^^^^^^^^^^^^^^ Injected command
```

**Attack Vector:**
1. User or attacker provides path with special characters: `" & malicious_command & "`
2. Path is inserted into command string via string formatting
3. Shell interprets `&` as command separator
4. Malicious command executes with user's privileges

### Evidence

**Current Implementation Issues:**
- ❌ Uses `format!()` to construct command line
- ❌ No escaping of special characters (`"`, `&`, `|`, `<`, `>`, `^`)
- ❌ Path passed as part of a single string argument
- ❌ No validation of path characters

**Why It's Exploitable:**
- Windows `explorer.exe` accepts `/select,path` argument
- String formatted with user input: `format!("/select,\"{}\"")`
- If path contains `"`, it can break out of quotes
- Characters like `&`, `|`, `;` allow command chaining

## Proposed Solutions

### Solution 1: Use Separate Arguments (RECOMMENDED)

**Approach:** Pass path as a separate argument instead of formatting into single string.

**Implementation:**
```rust
#[tauri::command]
pub async fn reveal_in_explorer(path: String) -> AppResult<()> {
    let path_buf = PathBuf::from(&path);

    if !path_buf.exists() {
        return Err(AppError::InvalidInput(format!("Path does not exist: {}", path)));
    }

    #[cfg(target_os = "windows")]
    {
        // Canonicalize path to prevent injection
        let canonical_path = path_buf.canonicalize()
            .map_err(|e| AppError::InvalidInput(format!("Invalid path: {}", e)))?;

        match std::process::Command::new("explorer.exe")
            .arg("/select,")
            .arg(canonical_path)  // Separate argument - no injection possible
            .spawn()
        {
            Ok(_) => Ok(()),
            Err(e) => Err(AppError::InvalidInput(format!("Failed to open Explorer: {}", e))),
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        Err(AppError::InvalidInput("This feature is only supported on Windows".to_string()))
    }
}
```

**Pros:**
- ✅ Complete protection against command injection
- ✅ Rust's `Command` API handles escaping automatically
- ✅ Simple and clean solution
- ✅ Canonicalization prevents path traversal as bonus

**Cons:**
- ⚠️ Need to verify Explorer accepts split arguments (testing required)

**Effort:** 1 hour (implement + test)
**Risk:** Low (standard Rust practice)

### Solution 2: Escape Special Characters

**Approach:** Manually escape all shell special characters before formatting.

**Implementation:**
```rust
fn escape_path_for_shell(path: &str) -> String {
    path.replace('"', "\"\"")  // Escape quotes by doubling
        .replace('&', "^&")     // Escape ampersand
        .replace('|', "^|")     // Escape pipe
        .replace('<', "^<")     // Escape less-than
        .replace('>', "^>")     // Escape greater-than
        .replace('^', "^^")     // Escape caret
}

#[tauri::command]
pub async fn reveal_in_explorer(path: String) -> AppResult<()> {
    let path_buf = PathBuf::from(&path);

    if !path_buf.exists() {
        return Err(AppError::InvalidInput(format!("Path does not exist: {}", path)));
    }

    #[cfg(target_os = "windows")]
    {
        let safe_path = escape_path_for_shell(&path_buf.display().to_string());
        let arg = format!("/select,\"{}\"", safe_path);

        match std::process::Command::new("explorer.exe")
            .arg(arg)
            .spawn()
        {
            Ok(_) => Ok(()),
            Err(e) => Err(AppError::InvalidInput(format!("Failed to open Explorer: {}", e))),
        }
    }
}
```

**Pros:**
- ✅ Protects against known attack vectors
- ✅ Preserves existing command structure

**Cons:**
- ❌ Easy to miss an escape character
- ❌ Windows shell escaping is complex and error-prone
- ❌ May break with Windows updates
- ❌ False sense of security (incomplete escaping)

**Effort:** 2 hours (implement + comprehensive testing)
**Risk:** Medium (easy to get wrong)

### Solution 3: Validate Path Characters

**Approach:** Reject paths containing shell special characters.

**Implementation:**
```rust
fn validate_path_characters(path: &str) -> AppResult<()> {
    let forbidden_chars = ['&', '|', '<', '>', '^', ';', '\n', '\r'];

    if path.chars().any(|c| forbidden_chars.contains(&c)) {
        return Err(AppError::InvalidInput(
            "Path contains invalid characters".to_string()
        ));
    }

    Ok(())
}

#[tauri::command]
pub async fn reveal_in_explorer(path: String) -> AppResult<()> {
    validate_path_characters(&path)?;

    let path_buf = PathBuf::from(&path);
    // ... rest of implementation
}
```

**Pros:**
- ✅ Simple to implement
- ✅ Explicit validation

**Cons:**
- ❌ May reject legitimate paths (rare but possible)
- ❌ Whitelist approach is better than blacklist
- ❌ Doesn't fix root cause (string concatenation)

**Effort:** 30 minutes
**Risk:** Medium (false positives, incomplete protection)

## Recommended Action

**Use Solution 1: Separate arguments**

**Rationale:**
- Leverages Rust's secure `Command` API
- No manual escaping required (less error-prone)
- Canonical paths prevent both injection and traversal
- Industry best practice for executing commands

**Implementation Steps:**
1. Update `reveal_in_explorer` to use separate `.arg()` calls
2. Add canonicalization step: `path_buf.canonicalize()`
3. Test with normal paths: `C:\Users\Username\Documents\file.txt`
4. Test with malicious inputs:
   - `C:\test" & calc & "`
   - `C:\test | dir C:\Windows\System32`
   - `C:\test > output.txt`
5. Verify Explorer opens correctly and no commands execute
6. Add unit tests with injection payloads

## Technical Details

**Affected Files:**
- `src/commands/filesystem.rs` (Lines 331-358)

**Command Injection Vectors:**
- `&` - Command separator (execute multiple commands)
- `|` - Pipe operator (redirect output to another command)
- `;` - Command separator (Unix-style, may work in some contexts)
- `<` `>` - Redirection operators
- `\n` - Newline (may allow multi-line injection)
- `^` - Escape character (Windows)

**Windows Shell Escaping Rules:**
- Doubling quotes: `"` becomes `""`
- Caret prefix: `&` becomes `^&`
- **Complex and error-prone** - avoid manual escaping

**Testing Payloads:**
```javascript
// Test these paths - none should execute commands
await invoke('reveal_in_explorer', { path: 'C:\\test" & calc.exe & "' })
await invoke('reveal_in_explorer', { path: 'C:\\test | dir C:\\Windows\\System32' })
await invoke('reveal_in_explorer', { path: 'C:\\test > output.txt & type output.txt' })
await invoke('reveal_in_explorer', { path: 'C:\\test; whoami' })
```

**Expected Behavior After Fix:**
- Explorer opens OR error returned
- **No commands execute**
- Path canonicalization may reject malformed paths (good)

## Acceptance Criteria

- [ ] `reveal_in_explorer` uses separate `.arg()` calls instead of format!()
- [ ] Path is canonicalized before passing to Command
- [ ] Injection payloads (see Testing Payloads) do NOT execute commands
- [ ] Normal paths still open Explorer correctly
- [ ] Unit tests verify injection protection
- [ ] Code review confirms no other string formatting vulnerabilities

## Work Log

### 2026-01-19 - Issue Discovered
- Security-Sentinel agent identified command injection vulnerability
- CVSS Score: 7.3 (HIGH)
- Attack vector: Path containing shell metacharacters
- **Blocker:** Must be fixed before PR merge

## Resources

- **OWASP Reference:** [A03:2021 – Injection](https://owasp.org/Top10/A03_2021-Injection/)
- **CWE-78:** OS Command Injection
- **Rust Security:** [Command API Documentation](https://doc.rust-lang.org/std/process/struct.Command.html)
- **Similar Fix:** Tag injection in `create_tag` (none - different vector)
