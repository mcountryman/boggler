//! Simple board cell.

use crate::board::BoardCell;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimpleBoardCell {
  x: u16,
  y: u16,
  ch: u8,
}

impl SimpleBoardCell {
  /// Create a [SimpleBoardCell].
  pub fn new(x: u16, y: u16, ch: u8) -> Self {
    Self { x, y, ch }
  }
}

impl BoardCell for SimpleBoardCell {
  fn x(&self) -> u16 {
    self.x
  }

  fn y(&self) -> u16 {
    self.y
  }

  fn ch(&self) -> u8 {
    self.ch
  }
}
