//! Value Objects
//!
//! Immutable objects that are defined by their attributes rather than identity.
//! Value objects encapsulate validation and domain constraints.

mod color;
mod file_path;
mod tag_value;

pub use color::Color;
pub use file_path::FilePath;
pub use tag_value::TagValue;
