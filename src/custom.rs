//! Module that allows define and use custom styles.

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
  static ref INVALID_STYLE_MATCHER: Regex = Regex::new(r"(?i)[\s\{\}]").unwrap();
  static ref CUSTOM: Mutex<HashMap<String, Vec<String>>> = Mutex::new(HashMap::new());
}

/// An error that occurred when adding a new custom style.
pub enum CustomStyleError {
  /// Error raised when the style name contains spaces or curly braces.
  InvalidSyntax,
}

/// Adds a new custom style.
pub fn add_style(name: &str, styles: &[&str]) -> Result<(), CustomStyleError> {
  // Check syntax
  if INVALID_STYLE_MATCHER.is_match(&name) {
    return Err(CustomStyleError::InvalidSyntax);
  }

  // Add the style and return
  CUSTOM
    .lock()
    .unwrap()
    .insert(name.to_string(), styles.iter().map(|c| c.to_string()).collect());

  Ok(())
}

/// Deletes one or more custom styles.
pub fn delete_styles(names: &[&str]) {
  for name in names.iter() {
    CUSTOM.lock().unwrap().remove(&name.to_string());
  }
}

/// Resolve all custom styles.
pub fn resolve_styles(names: &[&str]) -> Vec<String> {
  let mut resolved: Vec<String> = vec![];

  // For each name
  for name in names.iter() {
    // Attempt to find it
    match CUSTOM.lock().unwrap().get(&name.to_string()) {
      // Custom style found, add it
      Some(styles) => {
        resolved.extend_from_slice(styles);
      }
      // No result, add the token unchanged
      None => resolved.push(name.to_string()),
    }
  }

  resolved
}
