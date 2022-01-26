use std::{fs, path::PathBuf};

fn main() {
  let words_path = PathBuf::from("data/words.txt");
  if words_path.exists() {
    return;
  }

  let words =
    ureq::get("https://raw.githubusercontent.com/dwyl/english-words/master/words.txt")
      .call()
      .unwrap()
      .into_string()
      .unwrap();

  fs::write(words_path, words).unwrap();
}
