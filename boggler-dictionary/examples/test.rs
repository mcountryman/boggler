use boggler_core::{charset::english_alpha::EnglishAlpha, trie::Prefixes};
use std::{fs, path::PathBuf};
use std::fs::File;
use std::io::BufWriter;
use std::time::Instant;
use boggler_core::trie::serialize::Codegen;

fn main() {
  let time = Instant::now();
  let val = include!("../../english2.rs");
}

fn write(url: &str, name: &str) {
  let out = ".";

  let mut words_txt = name.to_string();
  words_txt.push_str(".txt");
  let words_txt = PathBuf::from(&out).join(words_txt);

  let mut words_bin = name.to_string();
  words_bin.push_str("2.rs");
  let words_bin = PathBuf::from(&out).join(words_bin);

  let words = ureq::get(url)
    .call()
    .unwrap()
    .into_string()
    .unwrap()
    .replace("TWL06 Scrabble Word List", "")
    .split('\n')
    .map(|word| word.trim().to_string())
    .filter(|word| !word.is_empty())
    .collect::<Vec<_>>();

  let prefixes = Prefixes::<EnglishAlpha>::from_words(&words, 50);
  let words_bin = File::create(words_bin).unwrap();
  let mut words_bin = BufWriter::new(words_bin);
  prefixes.codegen(&mut words_bin).unwrap();
  let words = words.join("\n");

  // bincode::serialize_into(File::create(words_bin).unwrap(), &prefixes);
  fs::write(words_txt, &words).unwrap();
}
