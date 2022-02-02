//! Word prefix tree.
pub mod charset;
pub mod node;

use self::node::PrefixNode;

/// A word prefix tree.
pub struct Prefixes<C> {
  root: Option<usize>,
  arena: Vec<PrefixNode<C>>,
}

impl<C> Prefixes<C> {
  /// Create a [Prefixes] from supplied words.
  pub fn from_words<I, W>(words: I) -> Self
  where
    I: Into<Vec<W>>,
    W: Ord + AsRef<str>,
  {
    let mut words = words.into();

    words.sort_unstable();

    todo!()
  }
}

#[cfg(test)]
mod tests {
  use super::Prefixes;

  #[test]
  fn test_from_words() {
    let prefixes = Prefixes::<26>::from_words(&["", "", ""][..]);
    let prefixes = Prefixes::<26>::from_words(vec!["", "", ""]);
  }
}
