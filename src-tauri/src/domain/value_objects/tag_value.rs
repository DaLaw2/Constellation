//! TagValue Value Object
//!
//! Represents a validated tag value.

use crate::domain::errors::DomainError;

/// A validated tag value.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TagValue {
    value: String,
}

impl TagValue {
    /// Creates a new TagValue after validation.
    ///
    /// # Errors
    ///
    /// Returns an error if the value is empty after trimming.
    pub fn new(value: impl Into<String>) -> Result<Self, DomainError> {
        let value = value.into();
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(DomainError::InvalidTagValue(
                "Tag value cannot be empty".to_string(),
            ));
        }

        Ok(Self {
            value: trimmed.to_string(),
        })
    }

    /// Returns the tag value as a string slice.
    pub fn as_str(&self) -> &str {
        &self.value
    }

    /// Consumes the TagValue and returns the inner string.
    pub fn into_string(self) -> String {
        self.value
    }
}

impl std::fmt::Display for TagValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl AsRef<str> for TagValue {
    fn as_ref(&self) -> &str {
        &self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_value_is_accepted() {
        let value = TagValue::new("Important");
        assert!(value.is_ok());
        assert_eq!(value.unwrap().as_str(), "Important");
    }

    #[test]
    fn empty_value_is_rejected() {
        let value = TagValue::new("");
        assert!(matches!(value, Err(DomainError::InvalidTagValue(_))));
    }

    #[test]
    fn whitespace_only_is_rejected() {
        let value = TagValue::new("   ");
        assert!(matches!(value, Err(DomainError::InvalidTagValue(_))));
    }

    #[test]
    fn value_is_trimmed() {
        let value = TagValue::new("  test  ");
        assert!(value.is_ok());
        assert_eq!(value.unwrap().as_str(), "test");
    }
}
