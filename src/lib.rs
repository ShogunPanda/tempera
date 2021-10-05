//! Template based terminal coloring made really easy.

extern crate lazy_static;

mod codes;
mod custom;
mod styling;
mod templating;

pub use codes::CODES;
pub use custom::{add_style, delete_styles, resolve_styles, CustomStyleError};
pub use styling::ColorError;
pub use templating::{clean_template, colorize_template};

/// Colorize a string using given styles.
pub fn colorize(content: &str, styles: &[&str]) -> String {
  let mut header = String::new();
  let mut footer = String::new();

  // For each requested style
  for style in styles {
    // Split styles by space
    let tokens: Vec<&str> = style.split(" ").collect();

    // Resolve custom styles
    for resolved in custom::resolve_styles(&tokens) {
      // Translate style to ANSI escape codes
      let (open, close) = styling::style_to_ansi(&resolved);

      // If at least one of the codes are valid, prepend and append to the string
      if open != "" && close != "" {
        header.push_str(open.as_str());
        footer.insert_str(0, close.as_str());
      }
    }
  }

  format!("{}{}{}", header, content, footer)
}
