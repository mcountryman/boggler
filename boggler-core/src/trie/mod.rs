//! Word prefix tree.

pub mod from_words;
pub mod node;
pub mod serialize;

use self::node::PrefixNode;
use crate::charset::{english_alpha::EnglishAlpha, Charset};

/// An english alphabet prefix tree.
pub type EnglishAlphaPrefixes = Prefixes<EnglishAlpha>;

/// A word prefix tree.
#[derive(Debug, Default)]
pub struct Prefixes<C: Charset> {
  root: Option<usize>,
  arena: Vec<PrefixNode<C>>,
}

impl<C: Charset> Prefixes<C> {
  /// Gets the root node.
  pub fn root(&self) -> Option<PrefixNodeRef<C>> {
    Some(PrefixNodeRef::new(self.root?, self))
  }

  /// Gets the index of the root node in the arena.
  pub fn root_index(&self) -> Option<usize> {
    self.root
  }

  /// Gets a reference to arena.
  pub fn arena(&self) -> &[PrefixNode<C>] {
    &self.arena
  }

  /// Get a [Prefixes] with a root index.
  pub fn with_root(mut self, root: usize) -> Self {
    self.root = Some(root);
    self
  }

  /// Get a [Prefixes] with an arena.
  pub fn with_arena(mut self, arena: Vec<PrefixNode<C>>) -> Self {
    self.arena = arena;
    self
  }
}

/// A prefix node reference.
#[derive(Debug, Clone, Copy)]
pub struct PrefixNodeRef<'prefixes, C: Charset> {
  node: usize,
  prefixes: &'prefixes Prefixes<C>,
}

impl<'prefixes, C: Charset> PrefixNodeRef<'prefixes, C> {
  /// Create a [PrefixNodeRef] from node index and [Prefixes].
  pub fn new(node: usize, prefixes: &'prefixes Prefixes<C>) -> Self {
    Self { node, prefixes }
  }

  /// Gets node word.
  pub fn word(&self) -> Option<usize> {
    self.prefixes.arena[self.node].word()
  }

  /// Gets child node reference.
  pub fn child(&self, ch: u8) -> Option<Self> {
    Some(Self::new(
      self.prefixes.arena[self.node].child(ch)?,
      self.prefixes,
    ))
  }
}
