//! Settings Entity
//!
//! Defines default values for application settings.

use std::collections::HashMap;

/// Known setting keys and their default values.
pub struct SettingsDefaults;

impl SettingsDefaults {
    /// Returns the default value for a known setting key.
    pub fn get(key: &str) -> Option<&'static str> {
        match key {
            "usn_auto_refresh" => Some("false"),
            "usn_refresh_on_missing" => Some("true"),
            "usn_cross_volume_match" => Some("true"),
            "thumbnail_size" => Some("256"),
            "thumbnail_force_shell_cache" => Some("false"),
            "thumbnail_cache_max_mb" => Some("500"),
            _ => None,
        }
    }

    /// Returns all known setting keys with their default values.
    pub fn all() -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("usn_auto_refresh".into(), "false".into());
        map.insert("usn_refresh_on_missing".into(), "true".into());
        map.insert("usn_cross_volume_match".into(), "true".into());
        map.insert("thumbnail_size".into(), "256".into());
        map.insert("thumbnail_force_shell_cache".into(), "false".into());
        map.insert("thumbnail_cache_max_mb".into(), "500".into());
        map
    }
}
