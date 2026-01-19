---
status: pending
priority: p1
issue_id: "003"
tags: [code-review, database, data-integrity, critical]
dependencies: []
---

# Missing Transaction Boundaries in Multi-Statement Operations

## Problem Statement

No explicit transactions are used anywhere in the codebase. All database operations execute as individual autocommit statements. Multi-statement operations (like `update_tag_group` which can execute up to 3 separate UPDATEs) are not atomic, leading to potential data corruption if operations fail mid-execution.

**Why it matters**: Partial updates create inconsistent database state, violate ACID properties, and can cause data loss or corruption during failures.

## Findings

**Data Integrity Guardian Report:**
- **Severity**: CRITICAL
- **Issue #13, #14**: Multiple UPDATEs not transactional

**Architecture Strategist Report:**
- Identified as critical scalability issue for data consistency

**Affected Operations**:

1. **update_tag_group** (`src/commands/tag_groups.rs:91-115`)
   - 3 separate UPDATE statements for name, color, display_order
   - `updated_at` set multiple times (last one wins)
   - No atomicity guarantee

2. **update_item** (`src/commands/items.rs:126-149`)
   - Multiple UPDATE statements for path, size, modified_time
   - Same atomicity problem

3. **update_tag** (`src/commands/tags.rs:130-139`)
   - Multiple UPDATEs not wrapped in transaction

**Example Vulnerable Code**:
```rust
if let Some(name) = name {
    conn.execute("UPDATE tag_groups SET name = ?1, updated_at = unixepoch() WHERE id = ?2", ...)?;
}
if let Some(color) = color {
    conn.execute("UPDATE tag_groups SET color = ?1, updated_at = unixepoch() WHERE id = ?2", ...)?;
}
if let Some(display_order) = display_order {
    conn.execute("UPDATE tag_groups SET display_order = ?1, updated_at = unixepoch() WHERE id = ?2", ...)?;
}
```

**Data Corruption Scenario**:
```
update_tag_group(1, Some("New Name"), Some("#ff0000"), Some(5))

Timeline:
1. name UPDATE succeeds
2. POWER FAILURE / APP CRASH
3. color UPDATE never happens
4. display_order UPDATE never happens

Result: Partial update, inconsistent state, updated_at shows wrong time
```

## Proposed Solutions

### Solution 1: Explicit BEGIN/COMMIT Transactions (Recommended)
**Effort**: Medium | **Risk**: Low | **Impact**: Critical

Wrap all multi-statement operations in explicit transactions:

```rust
conn.interact(move |conn: &mut Connection| {
    conn.execute("BEGIN IMMEDIATE", [])?;

    let result = (|| {
        // Check if exists
        let exists: bool = conn.query_row(
            "SELECT COUNT(*) FROM tag_groups WHERE id = ?1",
            [id],
            |row| row.get::<_, i64>(0).map(|count| count > 0),
        )?;

        if !exists {
            return Err(rusqlite::Error::QueryReturnedNoRows);
        }

        // All updates here
        if let Some(name) = name {
            conn.execute(
                "UPDATE tag_groups SET name = ?1, updated_at = unixepoch() WHERE id = ?2",
                (name, id),
            )?;
        }

        // ... other updates

        Ok::<(), rusqlite::Error>(())
    })();

    // Commit or rollback
    match result {
        Ok(_) => {
            conn.execute("COMMIT", [])?;
            Ok(())
        }
        Err(e) => {
            conn.execute("ROLLBACK", [])?;
            Err(e)
        }
    }
})
```

**Pros**:
- Guarantees atomicity
- Proper ACID compliance
- Explicit error handling with rollback

**Cons**: Slightly more code

### Solution 2: Single Dynamic UPDATE Statement
**Effort**: High | **Risk**: Low | **Impact**: High

Build a single UPDATE with only provided fields:

```rust
let mut updates = vec![];
let mut params: Vec<Box<dyn rusqlite::ToSql>> = vec![];

if let Some(name) = name {
    updates.push("name = ?");
    params.push(Box::new(name));
}
if let Some(color) = color {
    updates.push("color = ?");
    params.push(Box::new(color));
}
// ... collect all updates

let sql = format!(
    "UPDATE tag_groups SET {}, updated_at = unixepoch() WHERE id = ?",
    updates.join(", ")
);
conn.execute(&sql, params.as_slice())?;
```

**Pros**: Single query, inherently atomic
**Cons**: More complex, requires dynamic SQL building

## Recommended Action

**Implement Solution 1** (Explicit Transactions) for all multi-statement operations.

This is the industry-standard approach that provides clear transaction boundaries and proper error handling.

## Technical Details

**Files to Modify**:
1. `src/commands/tag_groups.rs` - update_tag_group (lines 70-122)
2. `src/commands/items.rs` - update_item (lines 105-156)
3. `src/commands/tags.rs` - update_tag (lines 111-146)

**Pattern to Apply**:
- BEGIN IMMEDIATE at start of interact closure
- Nested closure for operations that can fail
- COMMIT on success, ROLLBACK on error
- Propagate errors properly

**Additional Operations That Should Use Transactions**:
- Any future multi-step operations
- Cascading deletes (though CASCADE handles this at DB level)
- Batch operations when added

## Acceptance Criteria

- [ ] All multi-statement update operations wrapped in explicit transactions
- [ ] Proper rollback on any error
- [ ] `updated_at` timestamp set only once per transaction
- [ ] No partial updates possible
- [ ] Integration tests verify atomicity (simulate failures mid-transaction)
- [ ] Performance testing shows negligible overhead

## Work Log

### 2026-01-19
- **Discovered**: Data Integrity Guardian identified during database audit
- **Confirmed by**: Architecture Strategist agent
- **Status**: Awaiting triage and implementation
- **Priority**: CRITICAL - Data corruption risk

## Resources

- Data Integrity Guardian Report: Full database audit in agent output
- SQLite Transaction Docs: https://www.sqlite.org/lang_transaction.html
- Rusqlite Transaction API: https://docs.rs/rusqlite/latest/rusqlite/struct.Transaction.html
- Related Findings: #5 (Race Conditions), #15 (No Rollback Handling)
