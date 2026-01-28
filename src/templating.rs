//! Module that handles template tagged strings.

use crate::custom::resolve_styles;
use crate::styling::style_to_ansi;
use regex::{Captures, Regex};
use std::sync::LazyLock;

static PARSER: LazyLock<Regex> = LazyLock::new(|| {
  Regex::new(r"(?i)(?:\{(?P<styleSingle>[^\{\}]+)\})|(?:\{\{(?P<styleDouble>[^\{\}]+)\}\})")
    .expect("Invalid template parser regex")
});
static SPLITTER: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[\|\s]+").expect("Invalid template splitter regex"));

static RESET: &str = "\u{1b}[0m";

fn add_styles(replacement: &mut String, styles: &mut Vec<String>, id: &str) -> bool {
  let open = style_to_ansi(id);

  if open.0.is_empty() {
    styles.push("ignore".to_string());
    return false;
  }

  styles.push(id.to_string());
  replacement.push_str(open.0.as_str());

  true
}

fn remove_styles(replacement: &mut String, styles: &mut Vec<Vec<String>>) {
  // Pop a the style from the list of the applied ones
  let to_remove = styles.pop();

  // Unset the style by applying the closing ANSI codes
  for id in to_remove.unwrap_or_default() {
    replacement.push_str(style_to_ansi(&id).1.as_str());
  }

  // If the applied styles stack is still non-empty, it means we have to restore the previous style
  if let Some(lasts) = styles.last() {
    for id in lasts {
      replacement.push_str(style_to_ansi(id).0.as_str());
    }
  }
}

/// Apply colors to a template tagged string.
pub fn colorize_template(content: &str) -> String {
  let mut applied: Vec<Vec<String>> = vec![];
  let mut replaced = false;

  // For each tag in the string
  let mut modified = PARSER.replace_all(content, |captures: &Captures| {
    let mut replacement = String::from("");
    let mut current: Vec<String> = vec![];

    // Get the styles - the regex guarantees one of these groups will match
    let ids = captures
      .name("styleSingle")
      .or_else(|| captures.name("styleDouble"))
      .expect("Regex should always capture either styleSingle or styleDouble");

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
          "-" => {
            // Close all styles applied so far
            if let Some(last) = applied.last() {
              // Note: We never push empty vectors to applied, so first() always exists
              let first = last.first().expect("Applied styles vector should never be empty");
              if first == "ignore" {
                applied.pop();
              } else {
                remove_styles(&mut replacement, &mut applied);
              }
            }

            // Do not process anything else in this style
            break;
          }
          "reset" => {
            // Reset all styles, it means drop all the styles applied so far so do not unset anything
            applied.clear();

            // Do not process anything else in this style
            break;
          }
          _ => {
            // Adding a new style
            if add_styles(&mut replacement, &mut current, id) {
              replaced = true
            }
          }
        }
      }
    }

    // If we added any style in this tag, add to the applied styles stack (in reverse order in order to guarantee proper closing)
    if !current.is_empty() {
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
