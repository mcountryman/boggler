//! Simple implementation of [Board].

pub mod cell;
pub mod neighbors;

use self::{cell::SimpleBoardCell, neighbors::SimpleNeighbors};
use super::Board;
use eyre::Result;

/// Simple boggle board.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimpleBoard<'buf> {
  buf: &'buf [u8],
}

impl<'buf> SimpleBoard<'buf> {
  /// Create a [SimpleBoard] from a string.
  pub fn new(board: &'buf str) -> Result<Self> {
    let buf = board.as_bytes();
    let board = Self { buf };

    if (u16::MAX as usize) < board.size() {
      eyre::bail!("number type `N` is not large enough to represent the board size");
    }

    if !super::is_1d_len_square_in_2d(buf.len()) {
      eyre::bail!("expected square board");
    }

    Ok(board)
  }
}

impl<'board, 'buf: 'board> Board<'board> for SimpleBoard<'buf> {
  type Cell = SimpleBoardCell;
  type Neighbors = SimpleNeighbors<'board, 'buf>;

  fn at(&self, x: u16, y: u16) -> Option<Self::Cell> {
    let index = y as usize * self.size() + x as usize;

    self
      .buf
      .get(index)
      .copied()
      .map(|ch| SimpleBoardCell::new(x, y, ch))
  }

  fn size(&self) -> usize {
    (self.buf.len() as f32).sqrt() as usize
  }

  fn neighbors(&'board self, x: u16, y: u16) -> Self::Neighbors {
    SimpleNeighbors::new(self, x, y)
  }
}
