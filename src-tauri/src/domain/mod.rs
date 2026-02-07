//! Domain Layer
//!
//! This module contains the core business logic and domain model.
//! It has no dependencies on infrastructure concerns (database, file system, etc.)

pub mod entities;
pub mod errors;
pub mod repositories;
pub mod search;
pub mod value_objects;
