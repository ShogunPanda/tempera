//! Module that allows define and use custom styles.

use crate::errors::Error;
use regex::Regex;
use std::collections::HashMap;
use std::sync::{LazyLock, RwLock};

static INVALID_STYLE_MATCHER: LazyLock<Regex> =
  LazyLock::new(|| Regex::new(r"(?i)[\s\{\}]").expect("Invalid style matcher regex"));

// This must be public due to tests.
pub static CUSTOM: LazyLock<RwLock<HashMap<String, Vec<String>>>> = LazyLock::new(|| RwLock::new(HashMap::new()));

/// Adds a new custom style.
pub fn add_style(name: &str, styles: &[&str]) -> Result<(), Error> {
  // Check syntax
  if INVALID_STYLE_MATCHER.is_match(name) {
    return Err(Error::InvalidSyntax);
  }

  let mut custom = CUSTOM.write().map_err(|_| Error::LockPoisoned)?;

  // Add the style and return
  custom.insert(
    name.to_lowercase().to_string(),
    styles.iter().map(|c| c.to_string()).collect(),
  );

  Ok(())
}

/// Deletes one or more custom styles.
pub fn delete_styles(names: &[&str]) -> Result<(), Error> {
  let mut custom = CUSTOM.write().map_err(|_| Error::LockPoisoned)?;

  for name in names.iter() {
    custom.remove(&name.to_lowercase().to_string());
  }

  Ok(())
}

/// Resolve all custom styles.
pub fn resolve_styles(names: &[&str]) -> Result<Vec<String>, Error> {
  let custom = CUSTOM.read().map_err(|_| Error::LockPoisoned)?;
  let mut resolved: Vec<String> = vec![];

  // For each name
  for name in names.iter() {
    // Attempt to find it
    match custom.get(&name.to_lowercase().to_string()) {
      // Custom style found, add it
      Some(styles) => {
        resolved.extend_from_slice(styles);
      }
      // No result, add the token unchanged
      None => resolved.push(name.to_string()),
    }
  }

  Ok(resolved)
}
