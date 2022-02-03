//! English alphabet charset.

use std::io::Write;
use super::{Charset, CharsetMap};
use serde::{Deserialize, Serialize};
use crate::trie::serialize::Codegen;

/// An english alphabet charset.
#[derive(Serialize, Deserialize, Default, Debug, Clone, Copy)]
pub struct EnglishAlpha;

impl Charset for EnglishAlpha {
  type PrefixCharMap = EnglishAlphaMap;

  #[inline]
  fn trim_special_characters(word: &str) -> String {
    word.to_lowercase().replace("qu", "q")
  }

  #[inline]
  fn to_prefix_char(ch: char) -> Option<u8> {
    match ch {
      'a'..='z' => Some(ch as u8 - b'a'),
      'A'..='Z' => Some(ch as u8 - b'A'),
      _ => None,
    }
  }
}

/// An english alphabet charset map.
#[derive(Serialize, Deserialize, Default, Debug, Clone, Copy)]
pub struct EnglishAlphaMap(pub [Option<usize>; 26]);

impl CharsetMap for EnglishAlphaMap {
  /// # Panics
  /// May panic if the byte is not a valid charset character.
  #[inline]
  fn get(&self, ch: u8) -> Option<usize> {
    self.0[ch as usize]
  }

  /// # Panics
  /// May panic if the byte is not a valid charset character.
  #[inline]
  fn insert(&mut self, ch: u8, val: usize) {
    self.0[ch as usize] = Some(val);
  }

  #[inline]
  fn copy_from(&mut self, other: &Self) {
    for i in 0..26 {
      self.0[i] = other.0[i];
    }
  }
}

impl Codegen for EnglishAlphaMap {
  fn codegen<W: Write>(&self, f: &mut W) -> std::io::Result<()> {
    write!(f, "EnglishAlphaMap([");

    for val in self.0 {
      if let Some(val) = val {
        write!(f, "Some({val}),")?;
      } else {
        write!(f, "None,")?;
      }
    }

    write!(f, "])")
  }
}
