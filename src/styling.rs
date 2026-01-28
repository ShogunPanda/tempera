//! Module that handles rendering of colors.

use crate::errors::Error;
use regex::{Captures, Regex};
use std::fmt::Write;
use std::sync::LazyLock;

static ANSI_MATCHER: LazyLock<Regex> = LazyLock::new(|| {
  Regex::new(r"(?i)^(?:bg_)?ansi:(\d{0,3})(?:[,;](\d)[,;](\d))?").expect("Invalid ANSI matcher regex")
});
static RGB_MATCHER: LazyLock<Regex> = LazyLock::new(|| {
  Regex::new(r"(?i)^(?:bg_)?rgb:(\d{0,3})[,;](\d{0,3})[,;](\d{0,3})").expect("Invalid RGB matcher regex")
});
static HEX_MATCHER: LazyLock<Regex> = LazyLock::new(|| {
  Regex::new(r"(?i)^(?:bg_)?hex:(?:#?)([0-9a-f]{2})([0-9a-f]{2})([0-9a-f]{2})").expect("Invalid HEX matcher regex")
});

/// Joins multiple color codes in a valid ANSI escape code.
fn escape_ansi(codes: &[u8]) -> String {
  let mut result = String::with_capacity(3 + codes.len() * 4);
  write!(result, "\u{1b}[").unwrap();
  for (i, code) in codes.iter().enumerate() {
    if i > 0 {
      result.push(';');
    }
    write!(result, "{}", code).unwrap();
  }
  result.push('m');
  result
}

/// Gets the basic color spec.
fn spec_to_ansi(spec: &str) -> &crate::codes::ANSICode {
  let key = if spec.starts_with("bg_") {
    "background"
  } else {
    "foreground"
  };

  crate::codes::CODES.get(key).expect("Failed to get ANSI code spec")
}

/// Parses a color component.
fn parse_color_component(raw: &str, base: u8, min: u8, max: u8) -> Result<u8, Error> {
  // Parse the raw as a number
  match u8::from_str_radix(raw, base.into()) {
    Ok(number) => {
      // Check bounds
      if number >= min && number <= max {
        Ok(number)
      } else {
        Err(Error::OutOfBounds(min, max))
      }
    }
    Err(_) => Err(Error::InvalidColor),
  }
}

/// Parses a color component.
fn parse_color(components: &[&str], base: u8, min: u8, max: u8) -> Result<[u8; 3], Error> {
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
  let codes = spec_to_ansi(spec.get(0).expect("Missing ANSI capture group").as_str());

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
  let codes = spec_to_ansi(spec.get(0).expect("Missing RGB capture group").as_str());

  // Full spec, parse each component and then join them.
  if let Ok([r, g, b]) = parse_color(&[&spec[1], &spec[2], &spec[3]], base, 0, 255) {
    open = escape_ansi(&[codes.open, 2, r, g, b]);
  }

  // Return the opening and closing codes
  (open, escape_ansi(&[codes.close]))
}

/// Converts a style to a sequence of valid ANSI escape codes.
pub fn style_to_ansi(raw_style: &str) -> (String, String) {
  let style = raw_style.to_lowercase();

  if let Some(spec) = ANSI_MATCHER.captures(&style) {
    convert_ansi_color(spec)
  } else if let Some(spec) = RGB_MATCHER.captures(&style) {
    convert_rgb_color(spec, 10)
  } else if let Some(spec) = HEX_MATCHER.captures(&style) {
    convert_rgb_color(spec, 16)
  } else if let Some(spec) = crate::codes::CODES.get(style.as_str()) {
    (escape_ansi(&[spec.open]), escape_ansi(&[spec.close]))
  } else {
    (String::new(), String::new())
  }
}
