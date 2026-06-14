//! Module that handles template tagged strings.

use crate::custom::resolve_styles;
use crate::styling::style_to_ansi;
use regex::{Captures, Regex};
use std::sync::LazyLock;

static PARSER: LazyLock<Regex> =
  LazyLock::new(|| Regex::new(r"(?i)<(?P<style>[^<>]+)>").expect("Invalid template parser regex"));
static SPLITTER: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[\|\s]+").expect("Invalid template splitter regex"));

static RESET: &str = "\u{1b}[0m";

/// Apply colors to a template tagged string.
pub fn colorize_template(content: &str) -> String {
  // Each entry is one opened tag. Its styles are reversed so closing emits correct nested order.
  let mut applied: Vec<Vec<String>> = vec![];
  let mut replaced = false;

  let mut modified = PARSER.replace_all(content, |captures: &Captures| {
    let mut replacement = String::from("");
    let mut current: Vec<String> = vec![];

    let ids = captures.name("style").expect("Regex should always capture style");

    if ids.as_str() == "/" {
      if let Some(last) = applied.last() {
        // Empty vectors are never pushed, so first() always exists here.
        let first = last.first().expect("Applied styles vector should never be empty");
        if first == "ignore" {
          applied.pop();
        } else {
          // Close the current tag, then reopen the previous one to preserve nesting.
          for id in applied.pop().unwrap_or_default() {
            replacement.push_str(style_to_ansi(&id).1.as_str());
          }

          if let Some(lasts) = applied.last() {
            for id in lasts {
              replacement.push_str(style_to_ansi(id).0.as_str());
            }
          }
        }
      }

      return replacement;
    }

    if let Ok(resolved) = resolve_styles(
      &SPLITTER
        .split(ids.as_str())
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>(),
    ) {
      for raw_id in resolved.iter() {
        let id = raw_id.trim();

        match id {
          "reset" => {
            // Reset drops the stack without emitting close codes because ANSI reset already handles it.
            applied.clear();

            break;
          }
          _ => {
            let open = style_to_ansi(id);

            if open.0.is_empty() {
              // Unknown-only tags still occupy the stack so their matching close is consumed.
              current.push("ignore".to_string());
            } else {
              current.push(id.to_string());
              replacement.push_str(open.0.as_str());
              replaced = true
            }
          }
        }
      }
    }

    if !current.is_empty() {
      // Reverse once here so closing a multi-style tag follows ANSI nesting rules.
      current.reverse();
      applied.push(current);
    }

    replacement
  });

  if replaced {
    modified += RESET;
  }

  modified.to_string()
}

/// Removes all templates tags from the string.
pub fn clean_template(content: &str) -> String {
  PARSER.replace_all(content, "").to_string()
}
