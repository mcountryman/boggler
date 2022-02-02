//! Boggle board.
pub mod simple;

/// Describes a boggle board.
pub trait Board<'board> {
  /// The board cell type.
  type Cell: BoardCell;
  /// The board neighbors iterator type.
  type Neighbors: Iterator<Item = Self::Cell>;

  /// Gets a board cell at a specific position.
  fn at(&self, x: u16, y: u16) -> Option<Self::Cell>;

  /// Gets the board size.
  ///
  /// Because the board will always be square this corresponds to both width and height.
  fn size(&self) -> usize;

  /// Gets a cell's neighbors.
  fn neighbors(&'board self, x: u16, y: u16) -> Self::Neighbors;
}

/// Describes a boggle board cell.
pub trait BoardCell {
  /// Gets the x coordinate.
  fn x(&self) -> u16;

  /// Gets the y coordinate.
  fn y(&self) -> u16;

  /// Gets the ascii representation of the cell character.
  ///
  /// In boggle the `q` character character is implicitly both `q` and `u`.
  fn ch(&self) -> u8;
}

/// Determines if a one dimensional array length can be viewed as a two dimensional array
/// with equal width and height.
pub fn is_1d_len_square_in_2d(len: usize) -> bool {
  let len = len as f64;
  let len_sqrt = len.sqrt();

  (len_sqrt.floor() - len_sqrt).abs() <= std::f64::EPSILON
}
