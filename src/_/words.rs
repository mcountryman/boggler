//! Words graph.

use eyre::Result;
use std::collections::HashMap;

pub struct Words<'words> {
  root: usize,
  words: Vec<&'words str>,
  arena: Vec<WordNode<'words>>,
}

impl<'words> Words<'words> {
  /// Create a [Words] graph from a list of words.
  pub fn new(words: Vec<&'words str>) -> Result<Self> {
    let mut arena = vec![WordNode {
      word: None,
      children: HashMap::new(),
    }];

    for word in &words {
      let mut leaf = 0usize;

      for (i, ch) in word.chars().enumerate() {
        let next = arena.len();
        let node = &mut arena[leaf];

        leaf = *node.children.entry(ch).or_insert(next);
        arena.push(WordNode {
          word: None,
          children: HashMap::new(),
        });

        // Is this the last character?
        if i == word.len() {
          arena[leaf].word = Some(word);
        }
      }
    }

    Ok(Self {
      root: 0,
      words,
      arena,
    })
  }

  /// Gets a [WordNode] at index.
  pub fn at(&self, at: usize) -> Option<&WordNode<'words>> {
    self.arena.get(at)
  }

  /// Gets the root [WordNode].
  pub fn root(&self) -> &WordNode<'words> {
    self.at(self.root).expect("Root node not found")
  }
}

pub struct WordNode<'words> {
  word: Option<&'words str>,
  children: HashMap<char, usize>,
}

impl<'words> WordNode<'words> {
  /// Gets a child [WordNode] for supplied character.
  pub fn child_at<'tree>(
    &self,
    words: &'tree Words<'words>,
    ch: char,
  ) -> Option<&'tree WordNode<'words>> {
    self.children.get(&ch).and_then(|&idx| words.at(idx))
  }
}

#[cfg(test)]
mod tests {
  use super::Words;

  #[test]
  fn test_new_words() {
    let words = include_str!("../data/words.txt")
      .lines()
      .collect::<Vec<_>>();
    let words = Words::new(words).unwrap();
  }
}
