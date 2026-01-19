---
status: pending
priority: p1
issue_id: "004"
tags: [code-review, data-integrity, database, critical]
dependencies: []
---

# Soft Delete Implementation Incomplete - Data Loss Risk

## Problem Statement

The database schema includes soft delete fields (`is_deleted`, `deleted_at`) but the implementation is incomplete. Current `delete_item` command performs HARD DELETE, causing permanent data loss. Additionally, no queries filter by `is_deleted`, so soft-deleted items would still appear in results.

**Why it matters**: Users expect recoverability when deleting files. Hard deletes with CASCADE remove all tag associations permanently, with no undo capability. This violates user expectations and creates data loss risk.

## Findings

**Data Integrity Guardian Report:**
- **Issue #6**: Soft-Deleted Items Still Queryable (if implemented)
- **Issue #7**: Missing Soft Delete Command
- **Issue #8**: deleted_at Not Set Atomically

**Schema Evidence** (`src/db/schema.rs:41-42`):
```sql
is_deleted BOOLEAN NOT NULL DEFAULT 0,
deleted_at INTEGER
```

**Current Behavior** (`src/commands/items.rs:159-174`):
```rust
pub async fn delete_item(id: i64, ...) -> AppResult<()> {
    conn.execute("DELETE FROM items WHERE id = ?1", [id])?;
    // Hard delete with CASCADE removes all item_tags
}
```

**Data Loss Scenario**:
```
User has 1000 items tagged with "Important" (id=5)
User accidentally calls delete_item(5)
Result: HARD DELETE with CASCADE
ALL 1000 item_tags associations permanently deleted
No way to recover
User's entire tagging work is lost
```

## Proposed Solutions

### Solution 1: Complete Soft Delete Implementation (Recommended)
**Effort**: High | **Risk**: Low | **Impact**: Critical

Implement full soft delete lifecycle:

```rust
// New command for soft delete
#[tauri::command]
pub async fn soft_delete_item(id: i64, state: State<'_, AppState>) -> AppResult<()> {
    let conn = state.db_pool.get().await?;

    conn.interact(move |conn: &mut Connection| {
        conn.execute("BEGIN IMMEDIATE", [])?;

        let result = (|| {
            // Check if item exists and is not already deleted
            let item: (bool,) = conn.query_row(
                "SELECT is_deleted FROM items WHERE id = ?1",
                [id],
                |row| Ok((row.get(0)?,)),
            ).map_err(|_| rusqlite::Error::QueryReturnedNoRows)?;

            if item.0 {
                return Err(rusqlite::Error::InvalidQuery); // Already deleted
            }

            // Soft delete: Set is_deleted=1 and deleted_at atomically
            conn.execute(
                "UPDATE items SET is_deleted = 1, deleted_at = unixepoch(), updated_at = unixepoch() WHERE id = ?1",
                [id],
            )?;

            Ok::<(), rusqlite::Error>(())
        })();

        match result {
            Ok(_) => conn.execute("COMMIT", [])?,
            Err(e) => { conn.execute("ROLLBACK", [])?; return Err(e); }
        }
        Ok(())
    })
    .await??;

    Ok(())
}

// Command to restore deleted items
#[tauri::command]
pub async fn restore_item(id: i64, state: State<'_, AppState>) -> AppResult<()> {
    conn.execute(
        "UPDATE items SET is_deleted = 0, deleted_at = NULL, updated_at = unixepoch() WHERE id = ?1 AND is_deleted = 1",
        [id],
    )?;
    Ok(())
}

// Command to list deleted items
#[tauri::command]
pub async fn get_deleted_items(state: State<'_, AppState>) -> AppResult<Vec<Item>> {
    // Query with WHERE is_deleted = 1
}

// Rename existing delete to permanent_delete_item or make it use soft delete
```

**Update all query commands** to filter `is_deleted = 0`:
```rust
// src/commands/items.rs:36
"SELECT ... FROM items WHERE id = ?1 AND is_deleted = 0"

// src/commands/items.rs:68
"SELECT ... FROM items WHERE path = ?1 AND is_deleted = 0"
```

**Pros**:
- User-friendly recoverability
- Maintains all associations during soft delete
- Can implement "trash" UI feature
- Matches schema design intent

**Cons**:
- More implementation work
- Need to handle deleted items in all queries
- Database grows with soft-deleted items (need cleanup strategy)

### Solution 2: Keep Hard Delete, Remove Schema Fields
**Effort**: Low | **Risk**: High | **Impact**: Medium

Remove `is_deleted` and `deleted_at` fields from schema since they're not used.

**Pros**: Simplifies codebase
**Cons**: No recoverability, data loss risk remains

### Solution 3: Add Confirmation and Keep Current Implementation
**Effort**: Low | **Risk**: High | **Impact**: Low

Keep hard delete but add frontend confirmation dialog.

**Pros**: Easy to implement
**Cons**: Doesn't solve fundamental problem, still causes data loss

## Recommended Action

**Implement Solution 1** (Complete Soft Delete Implementation)

The schema was designed with soft delete in mind. Complete the implementation to match user expectations and prevent accidental data loss.

## Technical Details

**Files to Modify**:
1. `src/commands/items.rs`:
   - Add `soft_delete_item(id)`
   - Add `restore_item(id)`
   - Add `get_deleted_items()`
   - Rename `delete_item` to `permanently_delete_item` or make it call soft_delete
   - Update all query commands to filter `is_deleted = 0`

2. `src/lib.rs`:
   - Register new IPC commands in invoke_handler

3. `frontend/stores/items.ts`:
   - Add store methods for soft delete, restore, get trash
   - Update existing delete to use soft delete

4. `frontend/components/` (optional):
   - Add "Trash" view to show deleted items
   - Add restore button

**Migration Consideration**: Existing databases won't have issues since all items default to `is_deleted = 0`.

## Acceptance Criteria

- [ ] `soft_delete_item` command marks items as deleted without removing from database
- [ ] `deleted_at` timestamp set atomically with `is_deleted`
- [ ] `restore_item` command undeletes items
- [ ] `get_deleted_items` returns list of soft-deleted items
- [ ] All query commands filter out soft-deleted items by default
- [ ] `permanently_delete_item` exists for irreversible deletion
- [ ] item_tags associations preserved during soft delete
- [ ] Integration tests verify complete lifecycle: create → soft delete → restore → permanent delete

## Work Log

### 2026-01-19
- **Discovered**: Data Integrity Guardian identified during database audit
- **Status**: Awaiting triage and implementation
- **Priority**: CRITICAL - Data loss risk, schema-implementation mismatch

## Resources

- Data Integrity Guardian Report: Complete database audit findings
- Related Findings: #10 (Orphaned item_tags on Soft Delete), #11 (Hard Delete Violates Pattern)
- Pattern: Soft Delete Best Practices - https://www.clever-cloud.com/blog/engineering/2015/05/20/soft-deletion-with-postgresql/
