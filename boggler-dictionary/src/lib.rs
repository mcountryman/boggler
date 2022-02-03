use once_cell::unsync::OnceCell;
use boggler_core::{
  charset::{english_alpha::EnglishAlpha, Charset},
  trie::Prefixes,
};

pub struct Dictionary<C: Charset + 'static> {
  words: &'static Vec<&'static str>,
  prefixes: &'static Prefixes<C>,
}

impl<C: Charset + 'static> Dictionary<C> {
  /// Gets words in dictionary.
  pub fn words(&self) -> &[&str] {
    self.words
  }

  /// Gets prefix trie of dictionary.
  pub fn prefixes(&self) -> &Prefixes<C> {
    self.prefixes
  }
}

impl Dictionary<EnglishAlpha> {
  /// Gets the english dictionary.
  pub fn english() -> Self {
    // static WORDS: OnceCell<Vec<&'static str>> = OnceCell::new();
    // static PREFIXES: OnceCell<&Prefixes<EnglishAlpha>> = OnceCell::new();
    // // static PREFIX_BYTES: OnceCell<AlignedVec> = OnceCell::new();
    //
    // let words = WORDS.get_or_init(|| {
    //   include_str!(concat!(env!("OUT_DIR"), "/english.txt"))
    //     .lines()
    //     .collect::<Vec<_>>()
    // });
    //
    // let prefixes = PREFIXES.get_or_init(|| {
    //   include!(concat!(env!("OUT_DIR"), "/english.rs"))
    // });
    //
    // Self { words, prefixes }
    todo!()
  }
}

#[cfg(test)]
mod tests {
  use crate::Dictionary;
  use std::time::Instant;

  #[test]
  fn test_english() {
    let time = Instant::now();
    let dictionary = Dictionary::english();

    println!("done: {:?}", time.elapsed());
    println!("words: {}", dictionary.words().len());
    println!("prefixes: {:?}", dictionary.prefixes().arena.len());
  }
}
