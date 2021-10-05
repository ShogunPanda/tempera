//! Module containing ANSI codes definitions.

use lazy_static::lazy_static;
use std::collections::HashMap;

/// A struct defining an ANSI open an close codes.
#[derive(Clone)]
pub struct ANSICode {
  /// The opening code.
  pub open: u8,

  /// The closing code.
  pub close: u8,
}

lazy_static! {
  /// A HashMap defining the list of builtin styles.
  pub static ref CODES: HashMap<&'static str, ANSICode> = {
    let m: HashMap<&'static str, ANSICode> = [
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
      ("bgBlack", ANSICode { open: 40, close: 49 }),
      ("bgRed", ANSICode { open: 41, close: 49 }),
      ("bgGreen", ANSICode { open: 42, close: 49 }),
      ("bgYellow", ANSICode { open: 43, close: 49 }),
      ("bgBlue", ANSICode { open: 44, close: 49 }),
      ("bgMagenta", ANSICode { open: 45, close: 49 }),
      ("bgCyan", ANSICode { open: 46, close: 49 }),
      ("bgWhite", ANSICode { open: 47, close: 49 }),
      // Bright foreground colors
      ("redBright", ANSICode { open: 91, close: 39 }),
      ("greenBright", ANSICode { open: 92, close: 39 }),
      ("yellowBright", ANSICode { open: 93, close: 39 }),
      ("blueBright", ANSICode { open: 94, close: 39 }),
      ("magentaBright", ANSICode { open: 95, close: 39 }),
      ("cyanBright", ANSICode { open: 96, close: 39 }),
      ("whiteBright", ANSICode { open: 97, close: 39 }),
      // Bright background colors
      ("bgBlackBright", ANSICode { open: 100, close: 49 }),
      ("bgRedBright", ANSICode { open: 101, close: 49 }),
      ("bgGreenBright", ANSICode { open: 102, close: 49 }),
      ("bgYellowBright", ANSICode { open: 103, close: 49 }),
      ("bgBlueBright", ANSICode { open: 104, close: 49 }),
      ("bgMagentaBright", ANSICode { open: 105, close: 49 }),
      ("bgCyanBright", ANSICode { open: 106, close: 49 }),
      ("bgWhiteBright", ANSICode { open: 107, close: 49 }),
    ].iter().cloned().collect();

    m
  };
}
