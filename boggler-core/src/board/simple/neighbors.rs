//! Simple board cell neighbors iterator.

use super::{cell::SimpleBoardCell, SimpleBoard};
use crate::{board::Board, charset::Charset};

/// A simple board cell neighbors iterator.
pub struct SimpleNeighbors<'board, C: Charset> {
  x: u16,
  y: u16,
  board: &'board SimpleBoard<C>,
  kernel: usize,
}

impl<'board, C: Charset> SimpleNeighbors<'board, C> {
  /// Create a [SimpleNeighbors] iterator.
  pub fn new(board: &'board SimpleBoard<C>, x: u16, y: u16) -> Self {
    Self {
      x,
      y,
      board,
      kernel: 0,
    }
  }
}

impl<'board, C: Charset> Iterator for SimpleNeighbors<'board, C> {
  type Item = SimpleBoardCell;

  fn next(&mut self) -> Option<Self::Item> {
    const KERNEL: &[(i32, i32)] = &[
      (-1, -1),
      (0, -1),
      (-1, 0),
      (1, 1),
      (0, 1),
      (1, 0),
      (-1, 1),
      (1, -1),
    ];

    loop {
      let kernel = *KERNEL.get(self.kernel)?;
      let x = (self.x as i32 + kernel.0) as u16;
      let y = (self.y as i32 + kernel.1) as u16;

      self.kernel += 1;

      if let Some(cell) = self.board.at(x, y) {
        return Some(cell);
      }
    }
  }
}
