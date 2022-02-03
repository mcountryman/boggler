//! Simple implementation of [Board].

pub mod cell;
pub mod neighbors;

use self::{cell::SimpleBoardCell, neighbors::SimpleNeighbors};
use super::Board;
use crate::charset::{english_alpha::EnglishAlpha, Charset};
use eyre::Result;
use std::marker::PhantomData;

/// An english alphabet simple board.
pub type EnglishAlphaSimpleBoard = SimpleBoard<EnglishAlpha>;

/// Simple boggle board.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimpleBoard<C: Charset> {
  buf: Vec<u8>,
  _phantom: PhantomData<C>,
}

impl<C: Charset> SimpleBoard<C> {
  /// Create a [SimpleBoard] from a string.
  pub fn new(board: &str) -> Result<Self> {
    let board = Self {
      buf: board
        .chars()
        .map(|ch| {
          C::to_prefix_char(ch)
            .ok_or(eyre::eyre!("board contains invalid character `{ch}`"))
        })
        .collect::<Result<Vec<_>>>()?,
      _phantom: Default::default(),
    };

    if (u16::MAX as usize) < board.size() {
      eyre::bail!("number type `N` is not large enough to represent the board size");
    }

    if !super::is_1d_len_square_in_2d(board.buf.len()) {
      eyre::bail!("expected square board");
    }

    Ok(board)
  }
}

impl<'board, C: Charset + 'board> Board<'board> for SimpleBoard<C> {
  type Cell = SimpleBoardCell;
  type Neighbors = SimpleNeighbors<'board, C>;

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
