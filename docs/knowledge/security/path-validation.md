# Path Validation & Security

## Path Traversal Prevention

```rust
use std::path::{Component, PathBuf};

/// Validate path to prevent path traversal attacks
fn validate_path(path: &str) -> Result<PathBuf, DomainError> {
    let path_buf = PathBuf::from(path);

    // Check for traversal patterns in components
    for component in path_buf.components() {
        match component {
            Component::ParentDir => {
                return Err(DomainError::InvalidFilePath(
                    "Path traversal not allowed: '..' detected".to_string()
                ));
            }
            Component::CurDir => {
                return Err(DomainError::InvalidFilePath(
                    "Path traversal not allowed: '.' detected".to_string()
                ));
            }
            _ => {}
        }
    }

    // Also check raw string for encoded patterns
    if path.contains("..") || path.contains("./") || path.contains(".\\") {
        return Err(DomainError::InvalidFilePath(
            "Path traversal patterns not allowed".to_string()
        ));
    }

    Ok(path_buf)
}
```

## Command Injection Prevention

```rust
#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

pub async fn reveal_in_explorer(path: String) -> AppResult<()> {
    let path_buf = validate_path(&path)?;
    let canonical_path = path_buf.canonicalize()?;

    // Use raw_arg to prevent shell interpretation
    let select_arg = format!("/select,{}", canonical_path.display());

    std::process::Command::new("explorer.exe")
        .raw_arg(&select_arg)  // raw_arg prevents injection
        .spawn()?;

    Ok(())
}
```

**Why `raw_arg`**: Standard `.arg()` allows some shell interpretation. `raw_arg()` passes argument exactly as-is, preventing `&`, `|`, `>` from being interpreted.

## Content Security Policy (Tauri)

```json
// tauri.conf.json
{
  "security": {
    "csp": "default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'; img-src 'self' data: asset: http://asset.localhost; connect-src 'self' ipc: tauri: asset: http://asset.localhost"
  }
}
```

| Directive | Value | Purpose |
|-----------|-------|---------|
| `default-src` | `'self'` | Only app origin by default |
| `script-src` | `'self'` | Block inline scripts (XSS prevention) |
| `style-src` | `'self' 'unsafe-inline'` | Allow Vue scoped styles |
| `connect-src` | `ipc: tauri:` | Allow Tauri IPC |

## Value Object Pattern

Encapsulate validation in domain layer:

```rust
// domain/value_objects/file_path.rs
pub struct FilePath(String);

impl FilePath {
    pub fn new(path: &str) -> Result<Self, DomainError> {
        // Validation logic here
        validate_path(path)?;
        Ok(Self(path.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
```

## References

- [OWASP Path Traversal](https://owasp.org/www-community/attacks/Path_Traversal)
- [Tauri Security](https://v2.tauri.app/security/)
