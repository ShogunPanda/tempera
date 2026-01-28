use thiserror::Error;

/// An error that occurred when adding a new custom style.
#[derive(Error, Debug)]
pub enum Error {
  /// Error raised when the style name contains spaces or curly braces.
  #[error("Custom style cannot contain spaces or curly braces")]
  InvalidSyntax,
  /// Error raised when the color was not a valid number.
  #[error("Invalid color value")]
  InvalidColor,
  /// Error raised when the color was number outside of the allowed bounds.
  #[error("Color value out of bounds: expected between {0} and {1}")]
  OutOfBounds(u8, u8),
  /// Error raised when a lock was poisoned.
  #[error("Failed to acquire lock")]
  LockPoisoned,
}
