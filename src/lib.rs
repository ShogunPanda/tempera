//! Template based terminal coloring made really easy.

mod codes;
pub mod custom;
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

  for style in styles {
    let tokens: Vec<&str> = style.split(' ').collect();

    if let Ok(resolved_styles) = resolve_styles(&tokens) {
      for resolved in resolved_styles {
        let (open, close) = styling::style_to_ansi(&resolved);

        if !open.is_empty() {
          // Closing codes are prepended so nested styles are closed in reverse order.
          header.push_str(open.as_str());
          footer.insert_str(0, close.as_str());
        }
      }
    }
  }

  format!("{}{}{}", header, content, footer)
}
