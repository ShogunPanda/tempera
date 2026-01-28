//! Module containing ANSI codes definitions.

use std::{collections::HashMap, sync::LazyLock};

/// A struct defining an ANSI open an close codes.
#[derive(Clone, Default)]
pub struct ANSICode {
  /// The opening code.
  pub open: u8,

  /// The closing code.
  pub close: u8,
}

/// A HashMap defining the list of builtin styles.
pub static CODES: LazyLock<HashMap<&'static str, ANSICode>> = LazyLock::new(|| {
  HashMap::from([
    ("reset", ANSICode { open: 0, close: 0 }),
    ("foreground", ANSICode { open: 38, close: 39 }),
    ("background", ANSICode { open: 48, close: 49 }),
    // Style
    ("bold", ANSICode { open: 1, close: 22 }), // 21 isn't widely supported and 22 does the same thing
    ("dim", ANSICode { open: 2, close: 22 }),
    ("italic", ANSICode { open: 3, close: 23 }),
    ("underline", ANSICode { open: 4, close: 24 }),
    ("inverse", ANSICode { open: 7, close: 27 }),
    ("hidden", ANSICode { open: 8, close: 28 }),
    ("strikethrough", ANSICode { open: 9, close: 29 }),
    // Foreground colors
    ("black", ANSICode { open: 30, close: 39 }),
    ("red", ANSICode { open: 31, close: 39 }),
    ("green", ANSICode { open: 32, close: 39 }),
    ("yellow", ANSICode { open: 33, close: 39 }),
    ("blue", ANSICode { open: 34, close: 39 }),
    ("magenta", ANSICode { open: 35, close: 39 }),
    ("cyan", ANSICode { open: 36, close: 39 }),
    ("white", ANSICode { open: 37, close: 39 }),
    ("gray", ANSICode { open: 90, close: 39 }),
    // Background colors
    ("bg_black", ANSICode { open: 40, close: 49 }),
    ("bg_red", ANSICode { open: 41, close: 49 }),
    ("bg_green", ANSICode { open: 42, close: 49 }),
    ("bg_yellow", ANSICode { open: 43, close: 49 }),
    ("bg_blue", ANSICode { open: 44, close: 49 }),
    ("bg_magenta", ANSICode { open: 45, close: 49 }),
    ("bg_cyan", ANSICode { open: 46, close: 49 }),
    ("bg_white", ANSICode { open: 47, close: 49 }),
    // Bright foreground colors
    ("bright_red", ANSICode { open: 91, close: 39 }),
    ("bright_green", ANSICode { open: 92, close: 39 }),
    ("bright_yellow", ANSICode { open: 93, close: 39 }),
    ("bright_blue", ANSICode { open: 94, close: 39 }),
    ("bright_magenta", ANSICode { open: 95, close: 39 }),
    ("bright_cyan", ANSICode { open: 96, close: 39 }),
    ("bright_white", ANSICode { open: 97, close: 39 }),
    // Bright background colors
    ("bright_bg_black", ANSICode { open: 100, close: 49 }),
    ("bright_bg_red", ANSICode { open: 101, close: 49 }),
    ("bright_bg_green", ANSICode { open: 102, close: 49 }),
    ("bright_bg_yellow", ANSICode { open: 103, close: 49 }),
    ("bright_bg_blue", ANSICode { open: 104, close: 49 }),
    ("bright_bg_magenta", ANSICode { open: 105, close: 49 }),
    ("bright_bg_cyan", ANSICode { open: 106, close: 49 }),
    ("bright_bg_white", ANSICode { open: 107, close: 49 }),
  ])
});
