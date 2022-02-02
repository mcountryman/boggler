//! Prefix tree nodes.

use std::marker::PhantomData;

use super::charset::PrefixCharset;

/// A word prefix node.
pub enum PrefixNode<C> {
  Leaf(PrefixLeaf<C>),
  Branch(PrefixBranch<C>),
}

impl<C> PrefixNode<C> {
  /// Gets child of node matching supplied ASCII byte.
  pub fn child(&self, ch: u8) -> Option<usize> {
    self.children().get(ch as usize).copied().flatten()
  }

  /// Gets children of node.
  pub fn children(&self) -> &[Option<usize>] {
    match self {
      Self::Leaf(leaf) => leaf.children.as_slice(),
      Self::Branch(branch) => branch.children.as_slice(),
    }
  }

  /// Gets node word.
  pub fn word(&self) -> Option<usize> {
    match self {
      Self::Leaf(leaf) => Some(leaf.word),
      Self::Branch(_) => None,
    }
  }
}

/// A word prefix leaf node.
pub struct PrefixLeaf<C: PrefixCharset> {
  children: Box<[Option<usize>; C::SIZE]>,
  word: usize,
  _phantom: PhantomData<C>,
}

/// A word prefix branch node.
pub struct PrefixBranch<C: PrefixCharset> {
  children: Box<[Option<usize>; C::SIZE]>,
  _phantom: PhantomData<C>,
}
