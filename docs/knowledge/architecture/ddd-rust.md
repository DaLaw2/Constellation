# DDD Patterns in Rust

## Aggregate Design

Aggregates are consistency boundaries. The aggregate root manages lifecycle and enforces business rules.

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

## Aggregate Boundary Determination

1. **Identify Business Invariants**: True invariants define transactional boundaries
2. **Keep Aggregates Small**: Single responsibility
3. **Reference by ID Only**: Aggregates hold IDs to other aggregates, not direct references

```
Question: Does this entity have meaning independent of the aggregate root?

TagGroup (Aggregate Root)
├── Tag (Entity) - No independent lifecycle → inside aggregate

Item (Aggregate Root)
└── tags[] (References) - HAS independent lifecycle → separate aggregate
```

## Repository Pattern

```rust
// Repository Trait (Port)
#[async_trait]
pub trait ItemRepository: Send + Sync {
    async fn find_by_id(&self, id: &ItemId) -> Result<Option<Item>, RepositoryError>;
    async fn save(&self, item: &Item) -> Result<(), RepositoryError>;
    async fn delete(&self, id: &ItemId) -> Result<(), RepositoryError>;
}

// SQLite Implementation (Adapter)
pub struct SqliteItemRepository {
    pool: SqlitePool,
}

#[async_trait]
impl ItemRepository for SqliteItemRepository {
    // Implementation details...
}
```

## Domain Events (Future)

```rust
pub trait DomainEvent: Send + Sync {
    fn event_type(&self) -> &'static str;
    fn occurred_at(&self) -> i64;
}

// Example events
pub struct ItemIndexed { pub item_id: i64, pub path: String }
pub struct TagApplied { pub item_id: i64, pub tag_id: i64 }
```

**Event Bus Libraries:**
- `eventador` - Lock-free pub/sub
- `tokio::sync::broadcast` - Built-in Tokio solution

## References

- [rust-ddd GitHub](https://github.com/thalerjonathan/rust-ddd)
- [domain-driven-hexagon](https://github.com/sairyss/domain-driven-hexagon)
- [Rust CQRS Documentation](https://doc.rust-cqrs.org/theory_ddd.html)
