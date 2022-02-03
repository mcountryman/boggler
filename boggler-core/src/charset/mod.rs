//! Prefix tree charset.

pub mod english_alpha;

use std::fmt::Debug;
use crate::trie::serialize::Codegen;

/// Describes a prefix charset type.
pub trait Charset: Default {
  /// The character map type.
  type PrefixCharMap: CharsetMap;

  /// Gets a copy of supplied word with special characters removed.
  fn trim_special_characters(word: &str) -> String;

  /// Convert a character to a prefix charset compatible character if possible.
  fn to_prefix_char(ch: char) -> Option<u8>;
}

/// Describes a type that maps character bytes to `usize`.
pub trait CharsetMap: Default + Debug + Codegen {
  /// Get a value from map using supplied character as a key.
  fn get(&self, ch: u8) -> Option<usize>;

  /// Inserts a value into map using supplied character as a key.
  fn insert(&mut self, ch: u8, val: usize);

  /// Copies map contents from another map.
  fn copy_from(&mut self, other: &Self);
}
