use crate::grid::{graph::GridGraph, Grid};
use linked_hash_set::LinkedHashSet;

pub struct GridWords<'grid, 'chars> {
  grid: &'grid Grid,
  graph: GridGraph,

  word: usize,
  words: &'chars [String],
}

impl<'grid, 'chars> GridWords<'grid, 'chars> {
  pub fn new(grid: &'grid Grid, words: &'chars [String]) -> Self {
    let graph = GridGraph::new(grid);

    Self {
      grid,
      graph,

      word: 0,
      words,
    }
  }

  fn next_word(&mut self) -> Option<&'chars String> {
    loop {
      let word = self.words.get(self.word)?;

      let too_long = word.len() > self.grid.max_word_len();
      let too_short = word.len() < self.grid.min_word_len();
      if too_long || too_short {
        self.word += 1;
        continue;
      }

      self.word += 1;

      break Some(word);
    }
  }

  fn get_is_word_in_grid(&mut self, word: &str) -> bool {
    let mut path = LinkedHashSet::new();
    path.insert(self.graph.root());

    let mut paths = vec![path];
    let mut skip_u = false;

    for ch in word.chars() {
      if skip_u {
        skip_u = false;
        continue;
      }

      if paths.is_empty() {
        return false;
      }

      for i in (0..paths.len()).rev() {
        let path = &mut paths[i];
        let node = path.back().copied().unwrap();
        let children = self.graph.get_children_matching_ch(node, ch);
        let children = match children {
          Some(next) => next,
          None => {
            paths.remove(i);
            continue;
          }
        };

        let mut branches = children
          .iter()
          .filter(|node| !path.contains(node))
          .map(|node| {
            let mut branch = path.clone();
            branch.insert(*node);
            branch
          })
          .collect::<Vec<_>>();

        if ch == 'q' {
          if let Some(children) = self.graph.get_children_matching_ch(node, 'u') {
            skip_u = true;

            for node in children {
              if !path.contains(node) {
                let mut branch = path.clone();
                branch.insert(*node);
                branches.push(branch);
              }
            }
          }
        };

        paths.remove(i);
        paths.extend(branches);
      }

      if paths.is_empty() {
        return false;
      }
    }

    true
  }
}

impl<'grid, 'chars> Iterator for GridWords<'grid, 'chars> {
  type Item = &'chars str;

  fn next(&mut self) -> Option<Self::Item> {
    loop {
      let word = self.next_word()?;
      if self.get_is_word_in_grid(word) {
        return Some(word);
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::grid::words::GridWords;
  use crate::grid::Grid;
  use std::fs;
  use std::path::PathBuf;

  #[test]
  fn test_solve_4x4_1() {
    test_solve("test_grid4x4_1.txt", "test_words4x4_1.txt");
  }

  #[test]
  fn test_solve_10x10_1() {
    test_solve("test_grid10x10_1.txt", "test_words10x10_1.txt");
  }

  fn test_solve(grid: &str, words: &str) {
    let grid = PathBuf::new()
      .join(env!("CARGO_MANIFEST_DIR"))
      .join("data")
      .join(grid);
    let words = PathBuf::new()
      .join(env!("CARGO_MANIFEST_DIR"))
      .join("data")
      .join(words);

    let grid = fs::read_to_string(grid).unwrap();
    let grid = Grid::from_grid(grid).unwrap();

    let words = fs::read_to_string(words)
      .unwrap()
      .lines()
      .map(|word| word.trim().to_lowercase())
      .filter(|word| !word.is_empty())
      .collect::<Vec<_>>();

    let expected = words.clone();

    let words = GridWords::new(&grid, &words).collect::<Vec<_>>();

    for word in &words {
      if !expected.contains(&word.to_string()) {
        panic!("Matches does not contain `{}`", word);
      }
    }

    for word in &expected {
      if !words.contains(&word.as_str()) {
        panic!("Matches does not contain `{}`", word);
      }
    }
  }
}
