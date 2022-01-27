//! Boggle grid enumeration.

pub mod graph;
pub mod words;

use eyre::Result;

/// A boggle grid.
pub struct Grid {
  grid: Vec<char>,
  min_word_len: usize,
  neighbors_kernel: Vec<(isize, isize)>,
}

impl Grid {
  /// Create a new grid with options.
  pub fn new(
    grid: impl AsRef<str>,
    min_word_len: usize,
    neighbors_kernel: impl Into<Vec<(isize, isize)>>,
  ) -> Result<Self> {
    let grid = grid.as_ref();
    let grid = grid.chars().collect::<Vec<_>>();
    let grid_len_sqrt = (grid.len() as f64).sqrt();
    if (grid_len_sqrt.floor() - grid_len_sqrt).abs() > std::f64::EPSILON {
      eyre::bail!("expected grid square grid, got {grid_len_sqrt} by {grid_len_sqrt}");
    }

    Ok(Self {
      grid,
      min_word_len,
      neighbors_kernel: neighbors_kernel.into(),
    })
  }

  /// Create a new grid from a flattened square grid where nrows is length divided by two.
  pub fn from_grid(grid: impl AsRef<str>) -> Result<Self> {
    Self::new(
      grid,
      3,
      vec![
        (-1, -1),
        (0, -1),
        (-1, 0),
        (1, 1),
        (0, 1),
        (1, 0),
        (-1, 1),
        (1, -1),
      ],
    )
  }

  /// Gets a character in the grid at specified coordinates.
  pub fn at(&self, at: (usize, usize)) -> char {
    self.grid[at.0 * self.size() + at.1]
  }

  /// Gets grid size.
  pub fn size(&self) -> usize {
    (self.grid.len() as f32).sqrt() as usize
  }

  /// Gets the longest allowed word length.
  pub fn max_word_len(&self) -> usize {
    self.grid.len()
  }

  /// Gets the shortest allowed word length.
  pub fn min_word_len(&self) -> usize {
    self.min_word_len
  }

  /// Gets next logical point in the grid from the supplied point.
  pub fn next(&self, at: (usize, usize)) -> Option<(usize, usize)> {
    let size = self.size();

    if at.0 + 1 < size {
      Some((at.0 + 1, at.1))
    } else if at.1 + 1 < size {
      Some((0, at.1 + 1))
    } else {
      None
    }
  }

  pub fn neighbors(&self, at: (usize, usize)) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();

    for kernel in &self.neighbors_kernel {
      let x = at.0 as isize + kernel.0;
      let y = at.1 as isize + kernel.1;

      if x >= 0 && x < self.size() as isize && y >= 0 && y < self.size() as isize {
        neighbors.push((x as usize, y as usize));
      }
    }

    neighbors
  }
}

impl std::fmt::Debug for Grid {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    writeln!(f, "Grid {{")?;
    writeln!(f, "  size: {}", self.size())?;

    for x in 0..self.size() {
      for y in 0..self.size() {
        write!(f, "  {} ", self.at((x, y)))?;
      }

      writeln!(f)?;
    }

    write!(f, "}}")
  }
}

#[cfg(test)]
mod tests {
  use super::Grid;

  #[test]
  fn test_neighbors() {
    let grid = Grid::from_grid("modnstedetripyij").unwrap();
    let neighbors = grid.neighbors((0, 0));

    assert_eq!(neighbors.len(), 3);
    assert!(neighbors.contains(&(0, 1)));
    assert!(neighbors.contains(&(1, 0)));
    assert!(neighbors.contains(&(1, 1)));

    let neighbors = grid.neighbors((1, 1));

    assert_eq!(neighbors.len(), 8);
    assert!(neighbors.contains(&(0, 0)));
    assert!(neighbors.contains(&(1, 0)));
    assert!(neighbors.contains(&(2, 0)));
    assert!(neighbors.contains(&(0, 1)));
    assert!(neighbors.contains(&(0, 2)));
    assert!(neighbors.contains(&(0, 2)));
    assert!(neighbors.contains(&(1, 2)));
    assert!(neighbors.contains(&(2, 2)));
  }
}
