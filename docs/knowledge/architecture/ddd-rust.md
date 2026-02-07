# DDD Patterns in Rust & Constellation Architecture

This document details the Domain-Driven Design (DDD) architecture used in the Constellation project, along with general Rust DDD patterns.

## 1. Constellation Domain Model

### Directory Structure

The project follows a strict layered architecture:

```
src-tauri/src/
├── domain/                    # Domain Layer
│   ├── entities/              # Aggregates & Entities
│   │   ├── item.rs            # Item (File/Directory)
│   │   ├── tag.rs             # Tag
│   │   ├── tag_group.rs       # TagGroup
│   │   └── tag_template.rs    # TagTemplate
│   ├── value_objects/         # Value Objects
│   │   ├── color.rs           # Color
│   │   ├── file_path.rs       # FilePath
│   │   └── tag_value.rs       # TagValue
│   ├── repositories/          # Repository Traits (Ports)
│   └── errors.rs              # Domain Errors
├── application/               # Application Layer
│   ├── dto.rs                 # Data Transfer Objects
│   └── services/              # Application Services
├── infrastructure/            # Infrastructure Layer
│   └── persistence/           # SQLite Repository Implementation
└── commands/                  # Tauri Command Handlers
```

### Core Domain Terms

| Term | Definition |
|------|------------|
| **Item** | A file or directory indexed in the file system; the carrier of tags. |
| **Tag** | A user-defined marker used to categorize Items. |
| **Tag Group** | A logical grouping of Tags, possessing color and sort order. |
| **Tag Template** | A pre-defined combination of tags for quick application. |
| **Tagging** | The action of attaching a tag to an item. |

### Aggregate Boundaries

The following diagrams illustrate the aggregate roots and their relationships:

```
┌─────────────────────────────────────────┐
│           TagGroup Aggregate            │
│  ┌─────────────┐                        │
│  │  TagGroup   │ (Aggregate Root)       │
│  │  - id       │                        │
│  │  - name     │                        │
│  │  - color    │ ← Value Object         │
│  │  - order    │                        │
│  └─────────────┘                        │
│         │                               │
│         │ contains                      │
│         ▼                               │
│  ┌─────────────┐                        │
│  │    Tag      │ (Entity)               │
│  │  - id       │                        │
│  │  - value    │ ← Value Object         │
│  └─────────────┘                        │
│         │                               │
│         ▼                               │
│  ┌─────────────┐                        │
│  │ TagTemplate │ (Aggregate Root)       │
│  │  - id       │                        │
│  │  - name     │                        │
│  │  - tag_ids[]│ ← References           │
│  └─────────────┘                        │
└─────────────────────────────────────────┘

┌─────────────────────────────────────────┐
│            Item Aggregate               │
│  ┌─────────────┐                        │
│  │    Item     │ (Aggregate Root)       │
│  │  - id       │                        │
│  │  - path     │ ← Value Object         │
│  │  - metadata │                        │
│  │  - tags[]   │ ← References (IDs)     │
│  └─────────────┘                        │
└─────────────────────────────────────────┘
```

**Design Principles**:
- **Tag & TagGroup**: Form a single aggregate because the Tag lifecycle is strictly bound to the TagGroup.
- **Item**: An independent aggregate. It references Tags via ID (eventual consistency), ensuring no hard database constraints lock the systems tightly.
- **TagTemplate**: An independent aggregate holding a list of references.

---

## 2. General DDD Patterns (Reference)

### Aggregate Design Theory

Aggregates are consistency boundaries. The aggregate root manages lifecycle and enforces business rules.

**Example (Generic Order System):**
```rust
// Aggregate Root Example
pub struct Order {
    id: OrderId,
    customer_id: CustomerId,
    items: Vec<OrderItem>,      // Entities within aggregate
    status: OrderStatus,
}

impl Order {
    // Factory method - enforces invariants at creation
    pub fn create(customer_id: CustomerId, items: Vec<OrderItem>) -> Result<Self, DomainError> {
        if items.is_empty() {
            return Err(DomainError::EmptyOrder);
        }
        Ok(Self { /* ... */ })
    }

    // Domain behavior with business rules
    pub fn add_item(&mut self, item: OrderItem) -> Result<(), DomainError> {
        if self.status != OrderStatus::Pending {
            return Err(DomainError::CannotModifyConfirmedOrder);
        }
        self.items.push(item);
        Ok(())
    }
}
```

### Repository Pattern

The repository pattern abstracts data access behind a trait (Port), allowing different implementations (Adapters).

```rust
// Repository Trait (Port)
#[async_trait]
pub trait ItemRepository: Send + Sync {
    async fn find_by_id(&self, id: &ItemId) -> Result<Option<Item>, RepositoryError>;
    async fn save(&self, item: &Item) -> Result<(), RepositoryError>;
    async fn delete(&self, id: &ItemId) -> Result<(), RepositoryError>;
}
```

### Domain Events (Planned)

Future implementation to decouple side effects (like re-indexing or analytics).

```rust
pub trait DomainEvent: Send + Sync {
    fn event_type(&self) -> &'static str;
    fn occurred_at(&self) -> i64;
}

// Example events
pub struct ItemIndexed { pub item_id: i64, pub path: String }
pub struct TagApplied { pub item_id: i64, pub tag_id: i64 }
```

## References

- [rust-ddd GitHub](https://github.com/thalerjonathan/rust-ddd)
- [domain-driven-hexagon](https://github.com/sairyss/domain-driven-hexagon)
- [Rust CQRS Documentation](https://doc.rust-cqrs.org/theory_ddd.html)
