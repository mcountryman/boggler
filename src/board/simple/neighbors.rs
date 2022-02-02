//! Simple board cell neighbors iterator.

use super::{cell::SimpleBoardCell, SimpleBoard};
use crate::board::Board;

/// A simple board cell neighbors iterator.
pub struct SimpleNeighbors<'board, 'buf> {
  x: u16,
  y: u16,
  board: &'board SimpleBoard<'buf>,
  kernel: usize,
}

impl<'board, 'buf> SimpleNeighbors<'board, 'buf> {
  /// Create a [SimpleNeighbors] iterator.
  pub fn new(board: &'board SimpleBoard<'buf>, x: u16, y: u16) -> Self {
    Self {
      x,
      y,
      board,
      kernel: 0,
    }
  }
}

impl<'board, 'buf> Iterator for SimpleNeighbors<'board, 'buf> {
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
