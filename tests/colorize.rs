use tempera::{CODES, colorize};

#[test]
fn applies_known_styles() {
  assert_eq!(
    colorize("ABC", &["bg_Black", "red"]),
    "\u{1b}[40m\u{1b}[31mABC\u{1b}[39m\u{1b}[49m"
  );

  assert_eq!(
    colorize("ABC", &["bg_Black red unknown_style_xyz"]),
    "\u{1b}[40m\u{1b}[31mABC\u{1b}[39m\u{1b}[49m"
  );

  assert_eq!(colorize("ABC", &[]), "ABC");
}

#[test]
fn ignores_unknown_styles() {
  assert_eq!(colorize("ABC", &["whatever", "red"]), "\u{1b}[31mABC\u{1b}[39m");
}

#[test]
fn test_all_builtin_codes() {
  // Access all builtin codes to ensure the lazy static is initialized
  let codes = &*CODES;

  // Verify we have all expected codes
  assert!(codes.contains_key("reset"));
  assert!(codes.contains_key("bold"));
  assert!(codes.contains_key("dim"));
  assert!(codes.contains_key("italic"));
  assert!(codes.contains_key("underline"));
  assert!(codes.contains_key("inverse"));
  assert!(codes.contains_key("hidden"));
  assert!(codes.contains_key("strikethrough"));

  // Test all color codes
  let all_styles = [
    "reset",
    "foreground",
    "background",
    "bold",
    "dim",
    "italic",
    "underline",
    "inverse",
    "hidden",
    "strikethrough",
    "black",
    "red",
    "green",
    "yellow",
    "blue",
    "magenta",
    "cyan",
    "white",
    "gray",
    "bg_black",
    "bg_red",
    "bg_green",
    "bg_yellow",
    "bg_blue",
    "bg_magenta",
    "bg_cyan",
    "bg_white",
    "bright_red",
    "bright_green",
    "bright_yellow",
    "bright_blue",
    "bright_magenta",
    "bright_cyan",
    "bright_white",
    "bright_bg_black",
    "bright_bg_red",
    "bright_bg_green",
    "bright_bg_yellow",
    "bright_bg_blue",
    "bright_bg_magenta",
    "bright_bg_cyan",
    "bright_bg_white",
  ];

  for style in &all_styles {
    assert!(codes.contains_key(style), "Missing style: {}", style);
    let result = colorize("test", &[style]);
    assert!(result.contains("test"));
  }
}

#[test]
fn supports_ansi_256_colors() {
  assert_eq!(colorize("ABC", &["ANSI:232"]), "\u{1b}[38;5;232mABC\u{1b}[39m");
  assert_eq!(colorize("ABC", &["bg_ANSI:333"]), "ABC");
  assert_eq!(colorize("ABC", &["bg_ansi:2,4,0"]), "\u{1b}[48;5;112mABC\u{1b}[49m");
  assert_eq!(colorize("ABC", &["ANSI:2,4,6"]), "ABC");
}

#[test]
fn validates_ansi_format() {
  assert_eq!(colorize("ABC", &["ANSI:"]), "ABC");
  assert_eq!(colorize("ABC", &["ANSI:,"]), "ABC");
  assert_eq!(colorize("ABC", &["ANSI:1,2"]), "ABC");
  assert_eq!(colorize("ABC", &["ANSI:1,2,"]), "ABC");
  assert_eq!(colorize("ABC", &["bg_ANSI:,,"]), "ABC");
}

#[test]
fn validates_ansi_16_colors() {
  assert_eq!(colorize("ABC", &["ANSI:15"]), "ABC");
  assert_eq!(colorize("ABC", &["ANSI:0"]), "ABC");
  assert_eq!(colorize("ABC", &["bg_ANSI:10"]), "ABC");
}

#[test]
fn validates_ansi_256_colors() {
  // Test ANSI color component outside of the valid range (0-5)
  // When specifying r,g,b values for ANSI, each must be between 0 and 5
  assert_eq!(colorize("ABC", &["ANSI:-1,2,3"]), "ABC");
  assert_eq!(colorize("ABC", &["bg_ANSI:0,0,6"]), "ABC");
  assert_eq!(colorize("ABC", &["ANSI:0,7,0"]), "ABC");
}

#[test]
fn supports_ansi_16_milions_colors() {
  assert_eq!(
    colorize("ABC", &["rgb:255,232,0"]),
    "\u{1b}[38;2;255;232;0mABC\u{1b}[39m"
  );
  assert_eq!(
    colorize("ABC", &["bg_RGB:33,66,99"]),
    "\u{1b}[48;2;33;66;99mABC\u{1b}[49m"
  );
  assert_eq!(colorize("ABC", &["bg_RGB:999,999,999"]), "ABC");
  assert_eq!(colorize("ABC", &["bg_RGB:1,999,999"]), "ABC");
  assert_eq!(colorize("ABC", &["bg_RGB:1,2,999"]), "ABC");
}

#[test]
fn validates_hex_colors_format() {
  assert_eq!(colorize("ABC", &["hex:"]), "ABC");
  assert_eq!(colorize("ABC", &["HEX:#"]), "ABC");
  assert_eq!(colorize("ABC", &["bg_HEX:12"]), "ABC");
  assert_eq!(colorize("ABC", &["hex:12345"]), "ABC");
  assert_eq!(colorize("ABC", &["hex:GG1122"]), "ABC");
}

#[test]
fn valdates_ansi_16_milions_colors_format() {
  // Test various malformed RGB patterns
  assert_eq!(colorize("ABC", &["rgb:"]), "ABC");
  assert_eq!(colorize("ABC", &["RGB:100"]), "ABC");
  assert_eq!(colorize("ABC", &["bg_RGB:100,200"]), "ABC");
  assert_eq!(colorize("ABC", &["rgb:-1,100,100"]), "ABC");
}

#[test]
fn validates_ansi_16_milions_colors() {
  assert_eq!(colorize("ABC", &["rgb:abc,100,100"]), "ABC");
  assert_eq!(colorize("ABC", &["bg_RGB:100,def,100"]), "ABC");
  assert_eq!(colorize("ABC", &["RGB:100,100,xyz"]), "ABC");

  assert_eq!(colorize("ABC", &["rgb:,,100"]), "ABC");
  assert_eq!(colorize("ABC", &["bg_RGB:100,,100"]), "ABC");
  assert_eq!(colorize("ABC", &["RGB:100,100,"]), "ABC");
}

#[test]
fn supports_hex_colors() {
  assert_eq!(colorize("ABC", &["hex:F0d030"]), "\u{1b}[38;2;240;208;48mABC\u{1b}[39m");
  assert_eq!(
    colorize("ABC", &["bg_HEX:0099FF"]),
    "\u{1b}[48;2;0;153;255mABC\u{1b}[49m"
  );
  assert_eq!(colorize("ABC", &["bg_HEX:0099GG"]), "ABC");
}

#[test]
fn validates_hex_colors() {
  assert_eq!(colorize("ABC", &["hex:GGGGGG"]), "ABC");
  assert_eq!(colorize("ABC", &["bg_HEX:ZZZZZZ"]), "ABC");

  assert_eq!(colorize("ABC", &["hex:FFF"]), "ABC");
  assert_eq!(colorize("ABC", &["bg_HEX:12345"]), "ABC");
  assert_eq!(colorize("ABC", &["hex:1"]), "ABC");
}
