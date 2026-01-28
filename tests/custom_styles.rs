use tempera::{Error, add_style, colorize, colorize_template, delete_styles};

#[test]
fn allow_to_define_custom_styles_supported_both_by_colorize_and_colorize_template() {
  assert_eq!(colorize("ABC", &["customRed@@"]), "ABC");
  assert_eq!(
    colorize_template("{customRed@@ green}ABC{-}"),
    "\u{1b}[32mABC\u{1b}[39m\u{1b}[0m"
  );

  assert_eq!(add_style("customRed@@", &["red", "underline"]).is_ok(), true);

  assert_eq!(
    colorize("ABC", &["customRed@@"]),
    "\u{1b}[31m\u{1b}[4mABC\u{1b}[24m\u{1b}[39m"
  );
  assert_eq!(
    colorize_template("{customRed@@ green}ABC{-}"),
    "\u{1b}[31m\u{1b}[4m\u{1b}[32mABC\u{1b}[39m\u{1b}[24m\u{1b}[39m\u{1b}[0m"
  );
  assert_eq!(
    colorize_template("{{customRed@@ green}}ABC{{-}}"),
    "\u{1b}[31m\u{1b}[4m\u{1b}[32mABC\u{1b}[39m\u{1b}[24m\u{1b}[39m\u{1b}[0m"
  );
  let _ = delete_styles(&["customRed@@"]);

  assert_eq!(colorize("ABC", &["customRed@@"]), "ABC");
  assert_eq!(
    colorize_template("{customRed@@ green}ABC{-}"),
    "\u{1b}[32mABC\u{1b}[39m\u{1b}[0m"
  );
}

#[test]
fn should_reject_custom_styles_name_which_contain_spaces_or_curly_brace() {
  assert!(matches!(add_style("{invalid", &["red"]), Err(Error::InvalidSyntax)));
  assert!(matches!(add_style("invalid}", &["red"]), Err(Error::InvalidSyntax)));
  assert!(matches!(add_style("no spaces", &["red"]), Err(Error::InvalidSyntax)));
}
