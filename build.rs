use flate2::read::GzDecoder;
use regex::bytes::Regex;
use std::{
  fs::File,
  io::{Read, Write},
  path::PathBuf,
};

fn main() {
  println!("cargo:rerun-if-changed=build.rs");

  let out = std::env::var("OUT_DIR").unwrap();
  let words = PathBuf::from(out).join("words.txt");
  if words.exists() {
    return;
  }

  let body = ureq::get("https://en-word.net/static/english-wordnet-2021.xml.gz")
    .call()
    .unwrap()
    .into_reader();

  let mut read = GzDecoder::new(body);
  let mut body = Vec::with_capacity(4096);

  let mut words = File::create(words).unwrap();

  read.read_to_end(&mut body).unwrap();

  let re = Regex::new("<Lemma writtenForm=\"([^\"\\s]+)\"").unwrap();
  for cap in re.captures_iter(&body) {
    words.write_all(&cap[1]).unwrap();
    words.write_all(b"\n").unwrap();
  }
}
