//! Prefix tree nodes.

use std::io::Write;
use crate::charset::{Charset, CharsetMap};
use std::marker::PhantomData;
use crate::trie::serialize::Codegen;

/// A word prefix node.
#[derive(Debug)]
pub enum PrefixNode<C: Charset> {
  Leaf(PrefixLeaf<C>),
  Branch(PrefixBranch<C>),
}

impl<C: Charset> PrefixNode<C> {
  /// Creates a new leaf node where `word` is the index to the collection of words.
  pub fn leaf(word: usize) -> Self {
    Self::Leaf(PrefixLeaf {
      word,
      ..Default::default()
    })
  }

  /// Creates a new branch node.
  pub fn branch() -> Self {
    Self::Branch(PrefixBranch::default())
  }

  /// Determines if node is a leaf node.
  pub fn is_leaf(&self) -> bool {
    matches!(self, Self::Leaf(_))
  }

  /// Determines if a node is a branch node.
  pub fn is_branch(&self) -> bool {
    matches!(self, Self::Branch(_))
  }

  /// Gets child of node matching supplied ASCII byte.
  pub fn child(&self, ch: u8) -> Option<usize> {
    self.children().get(ch)
  }

  /// Gets children of node.
  pub fn children(&self) -> &C::PrefixCharMap {
    match self {
      Self::Leaf(leaf) => &leaf.children,
      Self::Branch(branch) => &branch.children,
    }
  }

  /// Gets a mutable reference to children of node.
  pub fn children_mut(&mut self) -> &mut C::PrefixCharMap {
    match self {
      Self::Leaf(leaf) => &mut leaf.children,
      Self::Branch(branch) => &mut branch.children,
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
#[derive(Default, Debug, Clone, Copy)]
pub struct PrefixLeaf<C: Charset> {
  pub children: C::PrefixCharMap,
  pub word: usize,
  _phantom: PhantomData<C>,
}

impl<C: Charset> PrefixLeaf<C> {
  /// Create a [PrefixLeaf].
  pub fn new(word: usize) -> Self {
    Self {
      word,
      ..Default::default()
    }
  }
}

/// A word prefix branch node.
#[derive(Default, Debug, Clone, Copy)]
pub struct PrefixBranch<C: Charset> {
  pub children: C::PrefixCharMap,
  _phantom: PhantomData<C>,
}
