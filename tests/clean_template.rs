use tempera::clean_template;

#[test]
fn removes_style_tags_from_a_template() {
  assert_eq!(clean_template("{red}ABC{green}CDE{-}EFG{-}HIJ"), "ABCCDEEFGHIJ");
  assert_eq!(clean_template("{red}ABC{yolla}CDE{-}EFG{-}HIJ"), "ABCCDEEFGHIJ");
  assert_eq!(clean_template("{red}}ABC{-}"), "}ABC");
  assert_eq!(clean_template("{red}ABC"), "ABC");
  assert_eq!(clean_template("{{red}"), "{");
  assert_eq!(clean_template("{red}ABC{green}CDE{- yellow}EFG{-}HIJ"), "ABCCDEEFGHIJ");
  assert_eq!(clean_template("{red}ABC{green}CDE{reset red}EFG{-}HIJ"), "ABCCDEEFGHIJ");
  assert_eq!(
    clean_template("{ANSI:5,0,0}ABC{RGB:0,255,0}CDE{bgHEX:#0000FF}EFG"),
    "ABCCDEEFG"
  );
}
