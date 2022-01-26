use crate::grid::{graph::GridGraph, Grid};
use std::str::Chars;

pub struct GridWords<'grid, 'words> {
  grid: &'grid Grid,

  node: usize,
  graph: GridGraph,

  word: usize,
  words: &'words [String],
  chars: Chars<'words>,
}

impl<'grid, 'words> GridWords<'grid, 'words> {
  pub fn new(grid: &'grid Grid, words: &'words [String]) -> Self {
    let graph = GridGraph::new(grid);
    let node = graph.root();

    Self {
      grid,
      node,
      graph,
      word: 0,
      words,
      chars: words[0].chars(),
    }
  }

  fn next_word(&mut self) -> bool {
    let mut word_i = self.word;

    loop {
      word_i += 1;

      let word = match self.words.get(word_i) {
        Some(word) => word,
        None => return false,
      };

      if word.len() < self.grid.min_word_len() || word.len() > self.grid.max_word_len() {
        continue;
      }

      self.word = word_i;
      self.chars = word.chars();
      self.node = self.graph.root();

      return true;
    }
  }
}

impl<'grid, 'words> Iterator for GridWords<'grid, 'words> {
  type Item = &'words String;

  fn next(&mut self) -> Option<Self::Item> {
    loop {
      let ch = self.chars.next();
      let ch = match ch {
        Some(ch) => ch,
        None => return Some(&self.words[self.word]),
      };

      let node = self.graph.next_node(self.node, ch);
      let node = match node {
        Some(node) => node,
        None => {
          if !self.next_word() {
            return None;
          }

          continue;
        }
      };

      self.node = node;
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::grid::{graph::GridGraph, Grid};
  use crate::words::GridWords;

  #[test]
  fn test_solve() {
    let grid = Grid::new("modnstedetripyij").unwrap();
    let words = include_str!("../data/words.txt")
      .lines()
      .map(|word| word.to_lowercase())
      .collect::<Vec<_>>();
    let mut words = GridWords::new(&grid, &words);

    for word in &mut words {
      println!("got word `{}`", word);
    }

    dbg!(words.word);
  }
}
