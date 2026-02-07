//! Color Value Object
//!
//! Represents a validated color (hex format).

use crate::domain::errors::DomainError;

/// A validated color in hex format.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Color {
    value: String,
}

impl Color {
    /// Creates a new Color after validation.
    ///
    /// Accepts hex color formats:
    /// - #RGB (3 hex digits)
    /// - #RRGGBB (6 hex digits)
    /// - #RRGGBBAA (8 hex digits with alpha)
    ///
    /// # Errors
    ///
    /// Returns an error if the color format is invalid.
    pub fn new(value: impl Into<String>) -> Result<Self, DomainError> {
        let value = value.into();
        let trimmed = value.trim();

        Self::validate_hex_color(trimmed)?;

        Ok(Self {
            value: trimmed.to_string(),
        })
    }

    /// Validates that the string is a valid hex color.
    fn validate_hex_color(value: &str) -> Result<(), DomainError> {
        if !value.starts_with('#') {
            return Err(DomainError::InvalidColor(
                "Color must start with '#'".to_string(),
            ));
        }

        let hex_part = &value[1..];
        let valid_lengths = [3, 6, 8]; // #RGB, #RRGGBB, #RRGGBBAA

        if !valid_lengths.contains(&hex_part.len()) {
            return Err(DomainError::InvalidColor(format!(
                "Invalid color length: expected 3, 6, or 8 hex digits, got {}",
                hex_part.len()
            )));
        }

        if !hex_part.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(DomainError::InvalidColor(
                "Color must contain only hexadecimal characters".to_string(),
            ));
        }

        Ok(())
    }

    /// Returns the color as a string slice.
    #[allow(dead_code)]
    pub fn as_str(&self) -> &str {
        &self.value
    }

    /// Consumes the Color and returns the inner string.
    #[allow(dead_code)]
    pub fn into_string(self) -> String {
        self.value
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl AsRef<str> for Color {
    fn as_ref(&self) -> &str {
        &self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_hex_color_6_digits() {
        let color = Color::new("#FF5733");
        assert!(color.is_ok());
    }

    #[test]
    fn valid_hex_color_3_digits() {
        let color = Color::new("#F53");
        assert!(color.is_ok());
    }

    #[test]
    fn valid_hex_color_with_alpha() {
        let color = Color::new("#FF5733AA");
        assert!(color.is_ok());
    }

    #[test]
    fn missing_hash_is_rejected() {
        let color = Color::new("FF5733");
        assert!(matches!(color, Err(DomainError::InvalidColor(_))));
    }

    #[test]
    fn invalid_length_is_rejected() {
        let color = Color::new("#FF57");
        assert!(matches!(color, Err(DomainError::InvalidColor(_))));
    }

    #[test]
    fn invalid_characters_are_rejected() {
        let color = Color::new("#GGGGGG");
        assert!(matches!(color, Err(DomainError::InvalidColor(_))));
    }
}
