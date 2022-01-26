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
    let mut arena = vec![GridNode::default()];
    let size = grid.size();

    for x in 0..size {
      for y in 0..size {
        let i = x * size + y;
        let ch = grid.at((x, y));

        arena.push(GridNode::default());
        arena[root]
          .children
          .entry(ch)
          .or_insert_with(Vec::default)
          .push(i);
      }
    }

    for x in 0..size {
      for y in 0..size {
        let i = x * size + y;
        let node = &mut arena[i + 1];
        let neighbors = grid.neighbors((x, y));

        for (x, y) in neighbors {
          let neighbor_i = x * size + y;
          let neighbor_ch = grid.at((x, y));

          node
            .children
            .entry(neighbor_ch)
            .or_insert_with(Vec::default)
            .push(neighbor_i);
        }
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

#[derive(Default, Debug, Clone)]
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

    println!("{:?}", graph.arena[graph.root()]);
  }
}
