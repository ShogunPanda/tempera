//! A trait for colorizing text output in the terminal.

use crate::colorize;

macro_rules! impl_colorize {
  ($type:ty) => {
    impl Colorize for $type {
      fn with_style(&self, style: &str) -> String {
        colorize(self, &[style])
      }

      fn with_styles(&self, style: &[&str]) -> String {
        colorize(self, style)
      }

      fn reset(&self) -> String {
        colorize(self, &["reset"])
      }

      fn foreground(&self) -> String {
        colorize(self, &["foreground"])
      }

      fn background(&self) -> String {
        colorize(self, &["background"])
      }

      fn bold(&self) -> String {
        colorize(self, &["bold"])
      }

      fn dim(&self) -> String {
        colorize(self, &["dim"])
      }

      fn italic(&self) -> String {
        colorize(self, &["italic"])
      }

      fn underline(&self) -> String {
        colorize(self, &["underline"])
      }

      fn inverse(&self) -> String {
        colorize(self, &["inverse"])
      }

      fn hidden(&self) -> String {
        colorize(self, &["hidden"])
      }

      fn strikethrough(&self) -> String {
        colorize(self, &["strikethrough"])
      }

      fn black(&self) -> String {
        colorize(self, &["black"])
      }

      fn red(&self) -> String {
        colorize(self, &["red"])
      }

      fn green(&self) -> String {
        colorize(self, &["green"])
      }

      fn yellow(&self) -> String {
        colorize(self, &["yellow"])
      }

      fn blue(&self) -> String {
        colorize(self, &["blue"])
      }

      fn magenta(&self) -> String {
        colorize(self, &["magenta"])
      }

      fn cyan(&self) -> String {
        colorize(self, &["cyan"])
      }

      fn white(&self) -> String {
        colorize(self, &["white"])
      }

      fn gray(&self) -> String {
        colorize(self, &["gray"])
      }

      fn bg_black(&self) -> String {
        colorize(self, &["bg_black"])
      }

      fn bg_red(&self) -> String {
        colorize(self, &["bg_red"])
      }

      fn bg_green(&self) -> String {
        colorize(self, &["bg_green"])
      }

      fn bg_yellow(&self) -> String {
        colorize(self, &["bg_yellow"])
      }

      fn bg_blue(&self) -> String {
        colorize(self, &["bg_blue"])
      }

      fn bg_magenta(&self) -> String {
        colorize(self, &["bg_magenta"])
      }

      fn bg_cyan(&self) -> String {
        colorize(self, &["bg_cyan"])
      }

      fn bg_white(&self) -> String {
        colorize(self, &["bg_white"])
      }

      fn bright_red(&self) -> String {
        colorize(self, &["bright_red"])
      }

      fn bright_green(&self) -> String {
        colorize(self, &["bright_green"])
      }

      fn bright_yellow(&self) -> String {
        colorize(self, &["bright_yellow"])
      }

      fn bright_blue(&self) -> String {
        colorize(self, &["bright_blue"])
      }

      fn bright_magenta(&self) -> String {
        colorize(self, &["bright_magenta"])
      }

      fn bright_cyan(&self) -> String {
        colorize(self, &["bright_cyan"])
      }

      fn bright_white(&self) -> String {
        colorize(self, &["bright_white"])
      }

      fn bright_bg_black(&self) -> String {
        colorize(self, &["bright_bg_black"])
      }

      fn bright_bg_red(&self) -> String {
        colorize(self, &["bright_bg_red"])
      }

      fn bright_bg_green(&self) -> String {
        colorize(self, &["bright_bg_green"])
      }

      fn bright_bg_yellow(&self) -> String {
        colorize(self, &["bright_bg_yellow"])
      }

      fn bright_bg_blue(&self) -> String {
        colorize(self, &["bright_bg_blue"])
      }

      fn bright_bg_magenta(&self) -> String {
        colorize(self, &["bright_bg_magenta"])
      }

      fn bright_bg_cyan(&self) -> String {
        colorize(self, &["bright_bg_cyan"])
      }

      fn bright_bg_white(&self) -> String {
        colorize(self, &["bright_bg_white"])
      }
    }
  };
}

pub trait Colorize {
  fn with_style(&self, style: &str) -> String;
  fn with_styles(&self, style: &[&str]) -> String;

  fn reset(&self) -> String;
  fn foreground(&self) -> String;
  fn background(&self) -> String;
  // Style
  fn bold(&self) -> String;
  fn dim(&self) -> String;
  fn italic(&self) -> String;
  fn underline(&self) -> String;
  fn inverse(&self) -> String;
  fn hidden(&self) -> String;
  fn strikethrough(&self) -> String;
  // Foreground colors
  fn black(&self) -> String;
  fn red(&self) -> String;
  fn green(&self) -> String;
  fn yellow(&self) -> String;
  fn blue(&self) -> String;
  fn magenta(&self) -> String;
  fn cyan(&self) -> String;
  fn white(&self) -> String;
  fn gray(&self) -> String;
  // Background colors
  fn bg_black(&self) -> String;
  fn bg_red(&self) -> String;
  fn bg_green(&self) -> String;
  fn bg_yellow(&self) -> String;
  fn bg_blue(&self) -> String;
  fn bg_magenta(&self) -> String;
  fn bg_cyan(&self) -> String;
  fn bg_white(&self) -> String;
  // Bright foreground colors
  fn bright_red(&self) -> String;
  fn bright_green(&self) -> String;
  fn bright_yellow(&self) -> String;
  fn bright_blue(&self) -> String;
  fn bright_magenta(&self) -> String;
  fn bright_cyan(&self) -> String;
  fn bright_white(&self) -> String;
  // Bright background colors
  fn bright_bg_black(&self) -> String;
  fn bright_bg_red(&self) -> String;
  fn bright_bg_green(&self) -> String;
  fn bright_bg_yellow(&self) -> String;
  fn bright_bg_blue(&self) -> String;
  fn bright_bg_magenta(&self) -> String;
  fn bright_bg_cyan(&self) -> String;
  fn bright_bg_white(&self) -> String;
}

impl_colorize!(str);
impl_colorize!(String);
