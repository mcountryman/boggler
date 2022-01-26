//! A grid graph.

use super::Grid;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct GridGraph {
  root: usize,
  arena: Vec<GridNode>,
}

impl GridGraph {
  /// Constructs a [GridGraph] from a [Grid].
  pub fn new(grid: &Grid) -> Self {
    let root = 0;
    let mut arena_coords = HashMap::new();
    let mut arena = vec![GridNode {
      children: HashMap::new(),
    }];

    for y in 0..grid.size() {
      for x in 0..grid.size() {
        let mut children = HashMap::new();

        for neighbor_coord in grid.neighbors((x, y)) {
          let neighbor_ch = grid.at(neighbor_coord);
          let neighbor = *arena_coords.entry(neighbor_coord).or_insert(arena.len());
          if neighbor == arena.len() {
            arena.push(GridNode {
              children: HashMap::new(),
            });
          }

          children
            .entry(neighbor_ch)
            .or_insert(Vec::new())
            .push(neighbor);
        }

        let node = arena.len();

        arena.push(GridNode { children });
        arena[root]
          .children
          .entry(grid.at((x, y)))
          .or_insert(Vec::new())
          .push(node);
      }
    }

    Self { root, arena }
  }

  /// Gets root node.
  pub fn root(&self) -> usize {
    self.root
  }

  /// Gets next grid nodes for supplied grid matching char.
  pub fn next_node(&mut self, node: usize, ch: char) -> Option<usize> {
    let found = self.arena.get(node);
    let found = match found {
      Some(node) => node,
      None => return None,
    };

    let node = found
      .children
      .get(&ch)
      .and_then(|nodes| nodes.first().cloned());

    node
  }
}

#[derive(Debug, Clone)]
pub struct GridNode {
  children: HashMap<char, Vec<usize>>,
}

#[cfg(test)]
mod tests {
  use super::GridGraph;
  use crate::grid::Grid;

  #[test]
  fn test_graph() {
    let grid = Grid::new("modnstedetripyij").unwrap();
    let graph = GridGraph::new(&grid);

    // dbg!(graph);
  }
}
