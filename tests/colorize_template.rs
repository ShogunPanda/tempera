use tempera::colorize_template;

#[test]
fn does_not_fail_when_empty() {
  assert_eq!(colorize_template("ABC"), "ABC");
}

#[test]
fn applies_known_styles_and_closes_them_in_the_right_order() {
  assert_eq!(
    colorize_template("{red}ABC{bright_green}CDE{-}EFG{-}HIJ"),
    "\u{1b}[31mABC\u{1b}[92mCDE\u{1b}[39m\u{1b}[31mEFG\u{1b}[39mHIJ\u{1b}[0m"
  );
}

#[test]
fn ignores_unknown_styles() {
  assert_eq!(
    colorize_template("{red}ABC{yolla}CDE{-}EFG{-}HIJ"),
    "\u{1b}[31mABCCDEEFG\u{1b}[39mHIJ\u{1b}[0m"
  );

  assert_eq!(colorize_template("{invalidStyle}Text{-}"), "Text");
  assert_eq!(colorize_template("{invalid1 invalid2}Text{-}"), "Text");

  assert_eq!(
    colorize_template("{red invalidStyle}Mixed{-}"),
    "\u{1b}[31mMixed\u{1b}[0m"
  )
}

#[test]
fn ignores_malformed_tags() {
  assert_eq!(colorize_template("Text with {-} at start"), "Text with  at start");
  assert_eq!(colorize_template("Text with {} empty"), "Text with {} empty");
  assert_eq!(colorize_template("Start {-} without open"), "Start  without open");
}

#[test]
fn ignores_empty_reset_stack() {
  assert_eq!(colorize_template("{reset}Text after reset"), "Text after reset");
  assert_eq!(
    colorize_template("{-}Closing without opening"),
    "Closing without opening"
  );
}

#[test]
fn ignores_unbalanced_parenthesis() {
  assert_eq!(colorize_template("{red}}ABC{-}"), "\u{1b}[31m}ABC\u{1b}[39m\u{1b}[0m");
}

#[test]
fn ignores_unbalanced_tags() {
  assert_eq!(colorize_template("{red}ABC"), "\u{1b}[31mABC\u{1b}[0m");
}

#[test]
fn double_curly_braces_are_respected() {
  assert_eq!(colorize_template("{{red}"), "{\u{1b}[31m\u{1b}[0m");
}

#[test]
fn closing_tag_ignores_further_specs() {
  assert_eq!(
    colorize_template("{red}ABC{green}CDE{- yellow}EFG{-}HIJ"),
    "\u{1b}[31mABC\u{1b}[32mCDE\u{1b}[39m\u{1b}[31mEFG\u{1b}[39mHIJ\u{1b}[0m"
  );
}

#[test]
fn reset_tag_cleans_the_stack() {
  assert_eq!(
    colorize_template("{red}ABC{green}CDE{reset red}EFG{-}HIJ"),
    "\u{1b}[31mABC\u{1b}[32mCDEEFGHIJ\u{1b}[0m"
  );
}

#[test]
fn supports_ansi_rgb_and_hex_colors() {
  assert_eq!(
    colorize_template("{ANSI:5,0,0}ABC{RGB:0,255,0}CDE{bg_HEX:#0000FF}EFG"),
    "\u{1b}[38;5;196mABC\u{1b}[38;2;0;255;0mCDE\u{1b}[48;2;0;0;255mEFG\u{1b}[0m"
  );
}
