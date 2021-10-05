use tempera::colorize;

#[test]
fn applies_known_styles() {
  assert_eq!(
    colorize("ABC", &["bgBlack", "red"]),
    "\u{1b}[40m\u{1b}[31mABC\u{1b}[39m\u{1b}[49m"
  );
}

#[test]
fn ignores_unknown_styles() {
  assert_eq!(colorize("ABC", &["whatever", "red"]), "\u{1b}[31mABC\u{1b}[39m");
}

#[test]
fn supports_ansi_256_colors() {
  assert_eq!(colorize("ABC", &["ANSI:232"]), "\u{1b}[38;5;232mABC\u{1b}[39m");
  assert_eq!(colorize("ABC", &["bgANSI:333"]), "ABC");
  assert_eq!(colorize("ABC", &["bgansi:2,4,0"]), "\u{1b}[48;5;112mABC\u{1b}[49m");
  assert_eq!(colorize("ABC", &["ANSI:2,4,6"]), "ABC");
}

#[test]
fn supports_ansi_16_milions_colors() {
  assert_eq!(
    colorize("ABC", &["rgb:255,232,0"]),
    "\u{1b}[38;2;255;232;0mABC\u{1b}[39m"
  );
  assert_eq!(
    colorize("ABC", &["bgRGB:33,66,99"]),
    "\u{1b}[48;2;33;66;99mABC\u{1b}[49m"
  );
  assert_eq!(colorize("ABC", &["bgRGB:999,999,999"]), "ABC");
  assert_eq!(colorize("ABC", &["bgRGB:1,999,999"]), "ABC");
  assert_eq!(colorize("ABC", &["bgRGB:1,2,999"]), "ABC");
}

#[test]
fn supports_hex_colors() {
  assert_eq!(colorize("ABC", &["hex:F0d030"]), "\u{1b}[38;2;240;208;48mABC\u{1b}[39m");
  assert_eq!(
    colorize("ABC", &["bgHEX:0099FF"]),
    "\u{1b}[48;2;0;153;255mABC\u{1b}[49m"
  );
  assert_eq!(colorize("ABC", &["bgHEX:0099GG"]), "ABC");
}
