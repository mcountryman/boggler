//! Prefix tree serialization.

use std::io::Write;
use crate::charset::Charset;
use crate::trie::node::{PrefixBranch, PrefixLeaf, PrefixNode};
use crate::trie::Prefixes;

pub trait Codegen {
  fn codegen<W: Write>(&self, f: &mut W) -> std::io::Result<()>;
}

impl<C: Charset> Codegen for Prefixes<C> {
  fn codegen<W: Write>(&self, f: &mut W) -> std::io::Result<()> {
    write!(f, "{{")?;
    write!(f, "Prefixes::default()");

    if let Some(root) = self.root_index() {
      write!(f, ".with_root({root})")?;
    }

    write!(f, ".with_arena(vec![");
    for node in self.arena() {
      node.codegen(f)?;
      write!(f, ",")?;
    }
    write!(f, "])");

    write!(f, "}}")
  }
}

impl<C: Charset> Codegen for PrefixNode<C> {
  fn codegen<W: Write>(&self, f: &mut W) -> std::io::Result<()> {
    match self {
      Self::Leaf(leaf) => {
        write!(f, "PrefixNode::Leaf(")?;
        leaf.codegen(f)?;
        write!(f, ")")
      },
      Self::Branch(branch) => {
        write!(f, "PrefixNode::Branch(")?;
        branch.codegen(f)?;
        write!(f, ")")
      }
    }
  }
}

impl<C: Charset> Codegen for PrefixLeaf<C> {
  fn codegen<W: Write>(&self, f: &mut W) -> std::io::Result<()> {
    write!(f, "{{")?;
    write!(f, "let mut v = PrefixLeaf::new({});", self.word);
    self.children.codegen(f)?;
    write!(f, "v")?;
    write!(f, "}}")
  }
}

impl<C: Charset> Codegen for PrefixBranch<C> {
  fn codegen<W: Write>(&self, f: &mut W) -> std::io::Result<()> {
    write!(f, "{{")?;
    write!(f, "let mut v = PrefixBranch::default();");
    self.children.codegen(f)?;
    write!(f, "v")?;
    write!(f, "}}")
  }
}
