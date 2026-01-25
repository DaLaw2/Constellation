# Search History Architecture (DDD)

## 1. Domain Analysis

The "Search History" feature requires persisting the context of a user's search intent for future recall.

### Ubiquitous Language

*   **Search Criteria**: The definition of *what* is being searched. It consists of:
    *   **Text Query** (Optional): The keyword string.
    *   **Tags** (List): The set of tags filtered.
    *   **Mode** (Enum): The logical operator for tags (AND/OR).
*   **Search History**: A historical record of a unique `Search Criteria` that was executed.
*   **Recent Searches**: A time-ordered list of unique `Search History` entries.

### Domain Model

#### Aggregate Root: `SearchHistory`

`SearchHistory` is an independent Aggregate Root. It does not belong to `Item` or `Tag` aggregates, although it references them.

```rust
pub struct SearchHistory {
    pub id: SearchHistoryId,      // Unique Identity
    pub criteria: SearchCriteria, // Value Object defining the search
    pub last_used_at: i64,        // Unix timestamp for LRU/Sorting
}
```

#### Value Object: `SearchCriteria`

This Value Object encapsulates the specific parameters of a search. It must implement structural equality to ensure that searching for "Foo" + "Tag A" today is recognized as the same criteria as searching for "Foo" + "Tag A" yesterday.

```rust
pub struct SearchCriteria {
    pub text_query: Option<String>,
    pub tag_ids: Vec<i64>,        // MUST be sorted for consistent equality comparison
    pub mode: SearchMode,         // AND / OR
}
```

## 2. Infrastructure (SQLite Schema)

We use a normalized schema to ensure data integrity and leverage SQL relations for cascading deletes.

### `search_histories` Table
Stores the core history record and the scalar parts of the criteria.

```sql
CREATE TABLE search_histories (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    text_query TEXT,                 -- Nullable
    search_mode TEXT NOT NULL,       -- 'AND' | 'OR'
    last_used_at INTEGER NOT NULL,   -- Unix timestamp
    created_at INTEGER NOT NULL DEFAULT (unixepoch())
);

-- Optimization for retrieving the "Recent" list
CREATE INDEX idx_search_histories_last_used ON search_histories(last_used_at DESC);
```

### `search_history_tags` Table
A junction table handling the Many-to-Many relationship between History and Tags.

```sql
CREATE TABLE search_history_tags (
    search_history_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    PRIMARY KEY (search_history_id, tag_id),
    
    -- Cascade Delete: If history entry is removed, links are removed.
    FOREIGN KEY (search_history_id) REFERENCES search_histories(id) ON DELETE CASCADE,
    
    -- Cascade Delete: If a Tag is deleted, it is removed from all histories.
    -- This maintains referential integrity without application-side logic.
    FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
);
```

## 3. Repository Pattern

The repository handles the logic of "Upserting" history (updating `last_used_at` if exists, inserting if new).

```rust
pub trait SearchHistoryRepository {
    async fn save(&self, criteria: SearchCriteria) -> Result<(), DomainError>;
    async fn get_recent(&self, limit: usize) -> Result<Vec<SearchHistory>, DomainError>;
    async fn delete(&self, id: i64) -> Result<(), DomainError>;
    async fn clear_all(&self) -> Result<(), DomainError>;
}
```
