use tempera::Colorize;

#[test]
fn test_colorize_trait_on_str() {
  let text = "Hello, World!";

  assert!(!text.bold().is_empty());
  assert!(!text.dim().is_empty());
  assert!(!text.italic().is_empty());
  assert!(!text.underline().is_empty());
  assert!(!text.inverse().is_empty());
  assert!(!text.hidden().is_empty());
  assert!(!text.strikethrough().is_empty());

  assert!(!text.black().is_empty());
  assert!(!text.red().is_empty());
  assert!(!text.green().is_empty());
  assert!(!text.yellow().is_empty());
  assert!(!text.blue().is_empty());
  assert!(!text.magenta().is_empty());
  assert!(!text.cyan().is_empty());
  assert!(!text.white().is_empty());
  assert!(!text.gray().is_empty());

  assert!(!text.bg_black().is_empty());
  assert!(!text.bg_red().is_empty());
  assert!(!text.bg_green().is_empty());
  assert!(!text.bg_yellow().is_empty());
  assert!(!text.bg_blue().is_empty());
  assert!(!text.bg_magenta().is_empty());
  assert!(!text.bg_cyan().is_empty());
  assert!(!text.bg_white().is_empty());
}

#[test]
fn test_colorize_trait_on_string() {
  let text = String::from("Hello, World!");

  assert!(!text.bold().is_empty());
  assert!(!text.dim().is_empty());
  assert!(!text.italic().is_empty());
  assert!(!text.underline().is_empty());
  assert!(!text.inverse().is_empty());
  assert!(!text.hidden().is_empty());
  assert!(!text.strikethrough().is_empty());

  assert!(!text.black().is_empty());
  assert!(!text.red().is_empty());
  assert!(!text.green().is_empty());
  assert!(!text.yellow().is_empty());
  assert!(!text.blue().is_empty());
  assert!(!text.magenta().is_empty());
  assert!(!text.cyan().is_empty());
  assert!(!text.white().is_empty());
  assert!(!text.gray().is_empty());

  assert!(!text.bg_black().is_empty());
  assert!(!text.bg_red().is_empty());
  assert!(!text.bg_green().is_empty());
  assert!(!text.bg_yellow().is_empty());
  assert!(!text.bg_blue().is_empty());
  assert!(!text.bg_magenta().is_empty());
  assert!(!text.bg_cyan().is_empty());
  assert!(!text.bg_white().is_empty());
}

#[test]
fn bright_test_colors_on_str() {
  let text = "Bright colors!";

  assert!(!text.bright_red().is_empty());
  assert!(!text.bright_green().is_empty());
  assert!(!text.bright_yellow().is_empty());
  assert!(!text.bright_blue().is_empty());
  assert!(!text.bright_magenta().is_empty());
  assert!(!text.bright_cyan().is_empty());
  assert!(!text.bright_white().is_empty());

  assert!(!text.bright_bg_black().is_empty());
  assert!(!text.bright_bg_red().is_empty());
  assert!(!text.bright_bg_green().is_empty());
  assert!(!text.bright_bg_yellow().is_empty());
  assert!(!text.bright_bg_blue().is_empty());
  assert!(!text.bright_bg_magenta().is_empty());
  assert!(!text.bright_bg_cyan().is_empty());
  assert!(!text.bright_bg_white().is_empty());
}

#[test]
fn bright_test_colors_on_string() {
  let text = String::from("Bright colors!");

  assert!(!text.bright_red().is_empty());
  assert!(!text.bright_green().is_empty());
  assert!(!text.bright_yellow().is_empty());
  assert!(!text.bright_blue().is_empty());
  assert!(!text.bright_magenta().is_empty());
  assert!(!text.bright_cyan().is_empty());
  assert!(!text.bright_white().is_empty());

  assert!(!text.bright_bg_black().is_empty());
  assert!(!text.bright_bg_red().is_empty());
  assert!(!text.bright_bg_green().is_empty());
  assert!(!text.bright_bg_yellow().is_empty());
  assert!(!text.bright_bg_blue().is_empty());
  assert!(!text.bright_bg_magenta().is_empty());
  assert!(!text.bright_bg_cyan().is_empty());
  assert!(!text.bright_bg_white().is_empty());
}

#[test]
fn test_with_style_methods() {
  let text = "Styled text";

  let styled = text.with_style("red");
  assert!(styled.contains(text));
  assert!(styled.len() > text.len());

  let multi_styled = text.with_styles(&["red", "bold"]);
  assert!(multi_styled.contains(text));
  assert!(multi_styled.len() > text.len());

  let string_text = String::from("Styled text");
  let styled_string = string_text.with_style("blue");
  assert!(styled_string.contains(&string_text));
  assert!(styled_string.len() > string_text.len());

  let multi_styled_string = string_text.with_styles(&["green", "underline"]);
  assert!(multi_styled_string.contains(&string_text));
  assert!(multi_styled_string.len() > string_text.len());
}

#[test]
fn test_special_methods() {
  let text = "Special methods";

  let reset_text = text.reset();
  assert!(reset_text.contains(text));

  let fg_text = text.foreground();
  assert!(fg_text.contains(text));

  let bg_text = text.background();
  assert!(bg_text.contains(text));

  let string_text = String::from("Special methods");

  let reset_string = string_text.reset();
  assert!(reset_string.contains(&string_text));

  let fg_string = string_text.foreground();
  assert!(fg_string.contains(&string_text));

  let bg_string = string_text.background();
  assert!(bg_string.contains(&string_text));
}

#[test]
fn test_ansi_escape_sequences() {
  let text = "ANSI test";

  let red_text = text.red();
  assert!(red_text.starts_with("\u{1b}["));
  assert!(red_text.ends_with("\u{1b}[39m"));

  let bold_text = text.bold();
  assert!(bold_text.starts_with("\u{1b}["));
  assert!(bold_text.contains(text));

  let bg_blue_text = text.bg_blue();
  assert!(bg_blue_text.starts_with("\u{1b}["));
  assert!(bg_blue_text.ends_with("\u{1b}[49m"));
}

#[test]
fn test_chaining_methods() {
  let text = "Chain test";

  let red_text = text.red();
  let bold_red_text = red_text.bold();

  assert!(bold_red_text.contains(text));
  assert!(bold_red_text.len() > red_text.len());
  assert!(bold_red_text.len() > text.len());
}

#[test]
fn test_empty_string() {
  let empty = "";

  let red_empty = empty.red();
  assert!(red_empty.len() > 0);
  assert!(red_empty.starts_with("\u{1b}["));
  assert!(red_empty.ends_with("\u{1b}[39m"));

  let bold_empty = empty.bold();
  assert!(bold_empty.len() > 0);
  assert!(bold_empty.starts_with("\u{1b}["));

  assert!(empty.green().starts_with("\u{1b}["));

  let empty_string = String::new();
  assert!(empty_string.blue().starts_with("\u{1b}["));
  assert!(empty_string.underline().starts_with("\u{1b}["));

  let styled = empty.cyan();
  let without_ansi = styled
    .chars()
    .filter(|c| !(*c == '\u{1b}' || (*c as u8 >= b'0' && *c as u8 <= b'9') || *c == '[' || *c == 'm'))
    .collect::<String>();
  assert_eq!(without_ansi, "");
}

#[test]
fn test_unicode_strings() {
  let unicode = "Hello 世界 🌍";

  let colored = unicode.red();
  assert!(colored.contains(unicode));
  assert!(colored.contains("世界"));
  assert!(colored.contains("🌍"));

  let unicode_string = String::from("Привет мир 🌈");
  let styled = unicode_string.bold();
  assert!(styled.contains("Привет"));
  assert!(styled.contains("мир"));
  assert!(styled.contains("🌈"));
}

#[test]
fn test_multiline_strings() {
  let multiline = "Line 1\nLine 2\nLine 3";

  let colored = multiline.cyan();
  assert!(colored.contains("Line 1"));
  assert!(colored.contains("Line 2"));
  assert!(colored.contains("Line 3"));
  assert!(colored.contains("\n"));

  assert!(colored.starts_with("\u{1b}["));
  assert!(colored.ends_with("\u{1b}[39m"));
}

#[test]
fn test_content_preservation() {
  let original = "Test content with spaces    and\ttabs";

  let styled = original.red();

  let without_ansi = styled.replace("\u{1b}[31m", "").replace("\u{1b}[39m", "");

  assert_eq!(without_ansi, original);

  let original_string = String::from("Another test\nwith newlines");
  let styled_string = original_string.green();

  let without_ansi_string = styled_string.replace("\u{1b}[32m", "").replace("\u{1b}[39m", "");

  assert_eq!(without_ansi_string, original_string);
}
