//! Boggle board search.

use crate::{
  board::{Board, BoardCell},
  charset::Charset,
  trie::{PrefixNodeRef, Prefixes},
};
use std::collections::HashSet;

pub fn find_words<'a, 'b: 'a, B: Board<'a>, C: Charset>(
  board: &'b B,
  prefixes: &Prefixes<C>,
  words: &mut Vec<usize>,
) {
  let root = prefixes.root().unwrap();

  for x in 0..board.size() as u16 {
    for y in 0..board.size() as u16 {
      let mut path = HashSet::new();
      path.insert((x, y));
      find_words_at(board, &root, path, x, y, words);
    }
  }
}

fn find_words_at<'a, 'b: 'a, B: Board<'a>, C: Charset>(
  board: &'b B,
  node: &PrefixNodeRef<C>,
  path: HashSet<(u16, u16)>,
  x: u16,
  y: u16,
  words: &mut Vec<usize>,
) -> Option<()> {
  let cell = board.at(x, y)?;
  let node = node.child(cell.ch())?;

  if let Some(word) = node.word() {
    words.push(word);
  }

  for neighbor in board.neighbors(x, y) {
    if path.contains(&(neighbor.x(), neighbor.y())) {
      continue;
    }

    let mut neighbor_path = path.clone();
    neighbor_path.insert((neighbor.x(), neighbor.y()));

    find_words_at(
      board,
      &node,
      neighbor_path,
      neighbor.x(),
      neighbor.y(),
      words,
    )?;
  }

  Some(())
}

#[cfg(test)]
mod tests {
  use super::find_words;
  use crate::{board::simple::EnglishAlphaSimpleBoard, trie::EnglishAlphaPrefixes};

  #[test]
  fn test_4x4() {
    let words = include_str!(concat!(env!("OUT_DIR"), "/words.txt"))
      .split('\n')
      .map(|word| word.trim())
      .collect::<Vec<_>>();

    let board = EnglishAlphaSimpleBoard::new("rrridaetswemrkoeeghheiana").unwrap();
    let prefixes = EnglishAlphaPrefixes::from_words(&words, 50);

    let mut indexes = Vec::new();

    find_words(&board, &prefixes, &mut indexes);

    for i in indexes {
      println!("word: {}", words[i]);
    }
  }
}
