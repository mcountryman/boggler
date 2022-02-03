// use boggler_core::{charset::english_alpha::EnglishAlpha, trie::Prefixes};
// use std::{fs, path::PathBuf};
//
fn main() {
  println!("cargo:rerun-if-changed=build.rs");

  // write(
  //   "https://www.wordgamedictionary.com/twl06/download/twl06.txt",
  //   "english",
  // );
}
//
// fn write(url: &str, name: &str) {
//   let out = std::env::var("OUT_DIR").unwrap();
//
//   let mut words_txt = name.to_string();
//   words_txt.push_str(".txt");
//   let words_txt = PathBuf::from(&out).join(words_txt);
//
//   let mut words_rs = name.to_string();
//   words_rs.push_str(".rs");
//   let words_rs = PathBuf::from(&out).join(words_rs);
//
//   let words = ureq::get(url)
//     .call()
//     .unwrap()
//     .into_string()
//     .unwrap()
//     .replace("TWL06 Scrabble Word List", "")
//     .split('\n')
//     .map(|word| word.trim().to_string())
//     .filter(|word| !word.is_empty())
//     .collect::<Vec<_>>();
//
//   let prefixes = Prefixes::<EnglishAlpha>::from_words(&words, 50);
//   let words = words.join("\n");
//
//   uneval::to_file(&prefixes, words_rs).unwrap();
//   fs::write(words_txt, &words).unwrap();
// }
