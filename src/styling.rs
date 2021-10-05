//! Module that handles rendering of colors.

extern crate lazy_static;

use lazy_static::lazy_static;
use regex::{Captures, Regex};

lazy_static! {
  static ref ANSI_MATCHER: Regex = Regex::new(r"(?i)^(?:bg)?ansi:(\d{0,3})(?:[,;](\d)[,;](\d))?").unwrap();
  static ref RGB_MATCHER: Regex = Regex::new(r"(?i)^(?:bg)?rgb:(\d{0,3})[,;](\d{0,3})[,;](\d{0,3})").unwrap();
  static ref HEX_MATCHER: Regex = Regex::new(r"(?i)^(?:bg)?hex:(?:#?)([0-9a-f]{2})([0-9a-f]{2})([0-9a-f]{2})").unwrap();
}

/// An error that occurred when parsing colors.
#[derive(Debug)]
pub enum ColorError {
  /// Error raised when the color was not a valid number.
  InvalidColor,
  /// Error raised when the color was number outside of the allowed bounds.
  OutOfBounds(u8, u8),
}

/// Joins multiple color codes in a valid ANSI escape code.
fn escape_ansi(codes: &[u8]) -> String {
  format!(
    "\u{1b}[{}m",
    codes
      .iter()
      .map(|c| format!("{}", c))
      .collect::<Vec<String>>()
      .join(";")
  )
}

/// Gets the basic color spec.
fn spec_to_ansi(spec: &str) -> &crate::codes::ANSICode {
  let key = if spec.starts_with("bg") {
    "background"
  } else {
    "foreground"
  };

  crate::codes::CODES.get(key).unwrap()
}

/// Parses a color component.
fn parse_color_component(raw: &str, base: u8, min: u8, max: u8) -> Result<u8, ColorError> {
  // Parse the raw as a number
  match u8::from_str_radix(raw, base.into()) {
    Ok(number) => {
      // Check bounds
      if number >= min && number <= max {
        Ok(number)
      } else {
        Err(ColorError::OutOfBounds(min, max))
      }
    }
    Err(_) => Err(ColorError::InvalidColor),
  }
}

/// Parses a color component.
fn parse_color(components: &[&str], base: u8, min: u8, max: u8) -> Result<[u8; 3], ColorError> {
  let mut parsed: [u8; 3] = [0; 3];

  // Parse each component and push the result to the array
  for (i, component) in components.iter().enumerate() {
    match parse_color_component(component, base, min, max) {
      Ok(num) => parsed[i] = num,
      Err(e) => return Err(e),
    }
  }

  Ok(parsed)
}

/// Converts an ANSI color spec.
fn convert_ansi_color(spec: Captures) -> (String, String) {
  let mut open = String::new();
  let codes = spec_to_ansi(spec.get(0).unwrap().as_str());

  // Simple color code by index, range 16 to 255
  if spec.get(2).is_none() {
    // Parse the color and escape it
    if let Ok(color) = parse_color_component(&spec[1], 10, 16, 255) {
      open = escape_ansi(&[codes.open, 5, color]);
    }
  // Full spec, parse each component and then join them using proper conversion.
  } else if let Ok([r, g, b]) = parse_color(&[&spec[1], &spec[2], &spec[3]], 10, 0, 5) {
    open = escape_ansi(&[codes.open, 5, 16 + (36 * r) + (6 * g) + b]);
  }

  // Return the opening and closing codes
  (open, escape_ansi(&[codes.close]))
}

/// Converts an RGB color spec.
fn convert_rgb_color(spec: Captures, base: u8) -> (String, String) {
  let mut open = String::new();
  let codes = spec_to_ansi(spec.get(0).unwrap().as_str());

  // Full spec, parse each component and then join them.
  if let Ok([r, g, b]) = parse_color(&[&spec[1], &spec[2], &spec[3]], base, 0, 255) {
    open = escape_ansi(&[codes.open, 2, r, g, b]);
  }

  // Return the opening and closing codes
  (open, escape_ansi(&[codes.close]))
}

/// Converts a style to a sequence of valid ANSI escape codes.
pub fn style_to_ansi(style: &str) -> (String, String) {
  if let Some(spec) = ANSI_MATCHER.captures(style) {
    if spec.len() > 1 {
      return convert_ansi_color(spec);
    }
  } else if let Some(spec) = RGB_MATCHER.captures(style) {
    if spec.len() > 1 {
      return convert_rgb_color(spec, 10);
    }
  } else if let Some(spec) = HEX_MATCHER.captures(style) {
    if spec.len() > 1 {
      return convert_rgb_color(spec, 16);
    }
  } else if let Some(spec) = crate::codes::CODES.get(style) {
    return (escape_ansi(&[spec.open]), escape_ansi(&[spec.close]));
  }

  (String::new(), String::new())
}
