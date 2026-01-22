# SQLite Optimization

## PRAGMA Configuration

```sql
-- Enable WAL mode (concurrent reads, better performance)
PRAGMA journal_mode = WAL;

-- Normal synchronous (good balance of safety/speed)
PRAGMA synchronous = NORMAL;

-- 32MB cache
PRAGMA cache_size = -32000;

-- Enable foreign keys
PRAGMA foreign_keys = ON;
```

**Important**: PRAGMA settings are per-connection. With connection pools, set on each new connection.

## Indexing Strategy

```sql
-- Single column indexes
CREATE INDEX idx_items_path ON items(path);
CREATE INDEX idx_items_is_directory ON items(is_directory);
CREATE INDEX idx_items_is_deleted ON items(is_deleted);
CREATE INDEX idx_tags_group_id ON tags(group_id);
CREATE INDEX idx_tags_value ON tags(value);

-- Junction table indexes (critical for JOIN performance)
CREATE INDEX idx_item_tags_item_id ON item_tags(item_id);
CREATE INDEX idx_item_tags_tag_id ON item_tags(tag_id);
```

## FTS5 Full-Text Search

### Setup

```sql
-- Create FTS5 virtual table
CREATE VIRTUAL TABLE items_fts USING fts5(
    name,
    path,
    content='items',
    content_rowid='id',
    tokenize='unicode61'
);

-- Sync triggers
CREATE TRIGGER items_ai AFTER INSERT ON items BEGIN
    INSERT INTO items_fts(rowid, name, path)
    VALUES (new.id, new.name, new.path);
END;

CREATE TRIGGER items_ad AFTER DELETE ON items BEGIN
    INSERT INTO items_fts(items_fts, rowid, name, path)
    VALUES ('delete', old.id, old.name, old.path);
END;
```

### Query Examples

```sql
-- Simple search
SELECT * FROM items_fts WHERE items_fts MATCH 'document';

-- Boolean operators
SELECT * FROM items_fts WHERE items_fts MATCH 'report AND annual';
SELECT * FROM items_fts WHERE items_fts MATCH 'report OR summary';
SELECT * FROM items_fts WHERE items_fts MATCH 'report NOT draft';

-- Phrase search
SELECT * FROM items_fts WHERE items_fts MATCH '"annual report"';

-- Column filter
SELECT * FROM items_fts WHERE items_fts MATCH 'name:report';

-- Prefix search
SELECT * FROM items_fts WHERE items_fts MATCH 'doc*';

-- With BM25 ranking
SELECT f.*, bm25(items_fts) as rank
FROM items_fts
JOIN items f ON f.id = items_fts.rowid
WHERE items_fts MATCH ?
ORDER BY rank;
```

## Multi-Tag Search Queries

### AND Logic (Items with ALL tags)

```sql
SELECT i.* FROM items i
INNER JOIN item_tags it ON i.id = it.item_id
WHERE it.tag_id IN (?, ?, ?) AND i.is_deleted = 0
GROUP BY i.id
HAVING COUNT(DISTINCT it.tag_id) = ?  -- must match all tags
```

### OR Logic (Items with ANY tag)

```sql
SELECT DISTINCT i.* FROM items i
INNER JOIN item_tags it ON i.id = it.item_id
WHERE it.tag_id IN (?, ?, ?) AND i.is_deleted = 0
```

## References

- [SQLite FTS5](https://sqlite.org/fts5.html)
- [SQLite Query Planning](https://sqlite.org/queryplanner.html)
