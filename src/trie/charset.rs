//! Prefix tree charset.

/// Describes a prefix charset type.
pub trait PrefixCharset {
  /// The number of characters in charset.
  const SIZE: usize = 26;

  /// Convert a character to a prefix charset compatible character if possible.
  fn to_prefix_char(ch: char) -> Option<u8>;
}
