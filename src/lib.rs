//! Template based terminal coloring made really easy.

mod codes;
pub mod custom; // This must be public due to tests
mod errors;
mod styling;
mod templating;
mod traits;
pub use codes::CODES;
pub use custom::{add_style, delete_styles, resolve_styles};
pub use errors::Error;
pub use templating::{clean_template, colorize_template};
pub use traits::Colorize;

/// Colorize a string using given styles.
pub fn colorize(content: &str, styles: &[&str]) -> String {
  let mut header = String::new();
  let mut footer = String::new();

  // For each requested style
  for style in styles {
    // Split styles by space
    let tokens: Vec<&str> = style.split(' ').collect();

    // Resolve custom styles
    if let Ok(resolved_styles) = resolve_styles(&tokens) {
      for resolved in resolved_styles {
        // Translate style to ANSI escape codes
        let (open, close) = styling::style_to_ansi(&resolved);

        // If the codes are valid, prepend and append to the string
        // Note: style_to_ansi always returns both empty or both non-empty
        if !open.is_empty() {
          header.push_str(open.as_str());
          footer.insert_str(0, close.as_str());
        }
      }
    }
  }

  format!("{}{}{}", header, content, footer)
}
