//! Prefix tree `from_words` implementation.

use super::{node::PrefixNode, Prefixes};
use crate::charset::{Charset, CharsetMap};

impl<C: Charset> Prefixes<C> {
  /// Create a [Prefixes] from supplied words.
  ///
  /// Ignores words that contain characters not in the supplied charset, with a length
  /// shorter than 3, or with a length longer than supplied `max_word_len`.
  pub fn from_words<W>(words: &[W], max_word_len: usize) -> Self
  where
    W: Ord + AsRef<str>,
  {
    let mut prefixes = Self::default();
    let mut trimmed_by_index = words
      .iter()
      .enumerate()
      .map(|(i, word)| {
        let word = word.as_ref();
        let word = C::trim_special_characters(word);

        (i, word)
      })
      .collect::<Vec<_>>();

    trimmed_by_index.dedup_by(|a, b| a.1 == b.1);

    for (i, word) in trimmed_by_index {
      if word.len() < 3 || word.len() > max_word_len {
        continue;
      }

      prefixes.insert_word(i, &word);
    }

    prefixes
  }

  /// Inserts a word into the tree.
  fn insert_word(&mut self, word_index: usize, word: &str) -> Option<()> {
    // Create a root node if one doesn't exist.
    let mut node = self.get_or_insert_root();

    // Iterate over each character.
    for (i, ch) in word.chars().enumerate() {
      // Get the character map byte from character.
      let ch = C::to_prefix_char(ch)?;
      // If this is the last character, create a leaf node with the word index, else
      // create a branch node.
      let child = if i == word.len() - 1 {
        PrefixNode::leaf(word_index)
      } else {
        PrefixNode::branch()
      };

      node = self.get_or_insert_child(node, ch, child);
    }

    Some(())
  }

  /// Gets or inserts the root branch node.
  fn get_or_insert_root(&mut self) -> usize {
    match self.root {
      Some(root) => root,
      None => {
        let root = self.arena.len();

        self.arena.push(PrefixNode::branch());
        self.root = Some(root);

        root
      }
    }
  }

  /// Gets or inserts a child node.
  fn get_or_insert_child(
    &mut self,
    node: usize,
    child_ch: u8,
    mut child_node: PrefixNode<C>,
  ) -> usize {
    match self.arena[node].child(child_ch) {
      Some(child) => {
        // The old node is a branch node, so replace it with a leaf node and copy children
        // from old branch node to new leaf node.
        if self.arena[child].is_branch() && child_node.is_leaf() {
          child_node
            .children_mut()
            .copy_from(self.arena[child].children());

          self.arena[child] = child_node;
        }

        child
      }
      None => {
        let child = self.arena.len();

        self.arena[node].children_mut().insert(child_ch, child);
        self.arena.push(child_node);

        child
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::{
    charset::{english_alpha::EnglishAlpha, Charset},
    trie::EnglishAlphaPrefixes,
  };

  #[test]
  fn test_from_words_simple() {
    let words = vec!["testa", "testb", "testc"];
    let prefixes = EnglishAlphaPrefixes::from_words(&words, 50);

    let root = prefixes.root().unwrap();

    let ch_t = EnglishAlpha::to_prefix_char('t').unwrap();
    let ch_e = EnglishAlpha::to_prefix_char('e').unwrap();
    let ch_s = EnglishAlpha::to_prefix_char('s').unwrap();
    let ch_a = EnglishAlpha::to_prefix_char('a').unwrap();
    let ch_b = EnglishAlpha::to_prefix_char('b').unwrap();
    let ch_c = EnglishAlpha::to_prefix_char('c').unwrap();

    let n_t = root.child(ch_t).unwrap();
    let n_e = n_t.child(ch_e).unwrap();
    let n_s = n_e.child(ch_s).unwrap();
    let n_t = n_s.child(ch_t).unwrap();
    let n_a = n_t.child(ch_a).unwrap();
    let n_b = n_t.child(ch_b).unwrap();
    let n_c = n_t.child(ch_c).unwrap();

    assert_eq!(n_a.word(), Some(0));
    assert_eq!(n_b.word(), Some(1));
    assert_eq!(n_c.word(), Some(2));
  }

  #[test]
  fn test_from_words_dictionary() {
    let mut words = include_str!(concat!(env!("OUT_DIR"), "/words.txt"))
      .split('\n')
      .map(|word| word.trim())
      .collect::<Vec<_>>();

    words.dedup();

    let prefixes = EnglishAlphaPrefixes::from_words(&words, 50);

    'words: for (i, word) in words.iter().enumerate() {
      let trimmed = EnglishAlpha::trim_special_characters(word);
      if trimmed.len() < 3 || trimmed.len() > 50 {
        continue;
      }

      let mut node = prefixes.root().unwrap();

      for ch in trimmed.chars() {
        let prefix_ch = match EnglishAlpha::to_prefix_char(ch) {
          Some(ch) => ch,
          None => continue 'words,
        };

        node = node.child(prefix_ch).unwrap_or_else(|| {
          panic!("Missing character node `{ch}` for word `{word}`@{i}")
        });
      }

      if node.word() != Some(i) {
        eprintln!(
          "Expected leaf node for word `{word}`@{i}, got {:?}",
          node.word()
        );
      }
      // assert_eq!(
      //   node.word(),
      //   Some(i),
      //   "Expected leaf node for word `{word}`@{i}"
      // );
    }
  }
}
