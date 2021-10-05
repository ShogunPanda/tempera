//! Module that handles template tagged strings.

use crate::custom::resolve_styles;
use crate::styling::style_to_ansi;
use lazy_static::lazy_static;
use regex::{Captures, Regex};

lazy_static! {
  static ref PARSER: Regex =
    Regex::new(r"(?i)(?:\{(?P<styleSingle>[^\{\}]+)\})|(?:\{\{(?P<styleDouble>[^\{\}]+)\}\})").unwrap();
  static ref SPLITTER: Regex = Regex::new(r"[\|\s]+").unwrap();
}

static RESET: &'static str = "\u{1b}[0m";

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

  if to_remove.is_none() {
    return;
  }

  // Unset the style by applying the closing ANSI codes
  for id in to_remove.unwrap() {
    replacement.push_str(style_to_ansi(&id).1.as_str());
  }

  // If the applied styles stack is still non-empty, it means we have to restore the previous style
  if !styles.is_empty() {
    for id in styles.last().unwrap() {
      replacement.push_str(style_to_ansi(&id).0.as_str());
    }
  }
}

/// Apply colors to a template tagged string.
pub fn colorize_template(content: &str) -> String {
  let mut applied: Vec<Vec<String>> = vec![];
  let mut replaced = false;

  // For each tag in the string
  let mut modified = PARSER.replace_all(content, |captures: &Captures| {
    // Get the styles
    let ids = captures.name("styleSingle").or(captures.name("styleDouble"));

    if ids.is_none() {
      return String::from(captures.get(0).unwrap().as_str());
    }

    let mut replacement = String::from("");
    let mut current: Vec<String> = vec![];

    // Split ids
    for raw_id in resolve_styles(&SPLITTER.split(ids.unwrap().as_str()).collect::<Vec<&str>>()).iter() {
      let id = raw_id.trim();

      match id {
        "-" => {
          // Close all styles applied so far
          if !applied.is_empty() {
            if applied.last().unwrap().get(0).unwrap() == "ignore" {
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

    // If we added any style in this tag, add to the applied styles stack (in reverse order in order to guarantee proper closing)
    if !current.is_empty() {
      current.reverse();
      applied.push(current);
    }

    return replacement;
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
