//! Grid word permutations iterator.

use crate::{grid::Grid, words::Words};
use linked_hash_set::LinkedHashSet;

pub struct GridPermutations {
  words: &'graph Words<'words>,
}

pub struct GridPermutationPath {
  done: LinkedHashSet<(usize, usize)>,
  node: usize,
  coord: (usize, usize),
}

impl<'grid, 'graph, 'words> GridPermutations<'grid, 'graph, 'words> {
  fn at_point_end(&self) -> bool {
    self.paths.is_empty()
  }

  fn next_point(&mut self) -> bool {
    let next = self.grid.next(self.at);
    let next = match next {
      Some(next) => next,
      None => return false,
    };

    self.at = next;
    self.path = 0;
    self.paths = true
  }

  fn extend_paths(&mut self) {
    // let node = node.child_at(words, grid.at(coord));
    // let node = match node {
    //   Some(node) => node,
    //   None => return,
    // };

    // for neighbor in grid.neighbors(coord) {
    //   let node = node.child_at(words, grid.at(coord));
    //   let node = match node {
    //     Some(node) => node,
    //     None => continue,
    //   };
    // }
  }

  fn get_paths_1_deep(&mut self, coord: (usize, usize)) {
    let node = self.words.root();
    let node = node.child_at(self.words, self.grid.at(coord));
    let node = match node {
      Some(node) => node,
      None => return,
    };

    for neighbor in self.grid.neighbors(coord) {
      let node = node.child_at(self.words, self.grid.at(coord));
      let node = match node {
        Some(node) => node,
        None => continue,
      };
    }
  }
}

impl<'grid, 'graph, 'words> Iterator for GridPermutations<'grid, 'graph, 'words> {
  type Item = &'words str;

  fn next(&mut self) -> Option<Self::Item> {
    loop {
      if self.at_point_end() && !self.next_point() {
        return None;
      }
    }

    todo!()
  }
}

///
/// | 0, 0 | 1, 0 | 2, 0 |
/// | 0, 1 | 1, 1 | 2, 1 |
/// | 0, 2 | 1, 2 | 2, 2 |
///
/// (0, 0), (0, 1), (0, 2), (1, 0), (1, 1), (1, 2), (2, 0), (2, 1), (2, 2)
/// (0, 0), (1, 0), (2, 0), (0, 1), (1, 1), (2, 1), (0, 2), (1, 2), (2, 2)
/// (0, 0), (1, 1), (2, 2)
///
#[cfg(test)]
mod tests {}
