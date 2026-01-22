//! Repository Traits
//!
//! Repository interfaces define the contract for data persistence.
//! The domain layer depends on these abstractions, not concrete implementations.

mod item_repository;
mod tag_group_repository;
mod tag_repository;
mod tag_template_repository;

pub use item_repository::ItemRepository;
pub use tag_group_repository::TagGroupRepository;
pub use tag_repository::TagRepository;
pub use tag_template_repository::TagTemplateRepository;
