use boggler::trie::EnglishAlphaPrefixes;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
  let words = include_str!(concat!(env!("OUT_DIR"), "/words.txt"))
    .split('\n')
    .map(|word| word.trim())
    .collect::<Vec<_>>();

  c.bench_with_input(
    BenchmarkId::new("sandbox", "words.txt"),
    &words,
    |b, words| b.iter(|| EnglishAlphaPrefixes::from_words(words, 50)),
  );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
