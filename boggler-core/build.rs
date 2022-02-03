use std::{
  fs,
  path::PathBuf,
};

fn main() {
  println!("cargo:rerun-if-changed=build.rs");

  let out = std::env::var("OUT_DIR").unwrap();
  let words = PathBuf::from(out).join("words.txt");
  if words.exists() {
    return;
  }

  let body = ureq::get("https://www.wordgamedictionary.com/twl06/download/twl06.txt")
    .call()
    .unwrap()
    .into_string()
    .unwrap().replace("TWL06 Scrabble Word List", "");

  fs::write(words, body).unwrap();
}
