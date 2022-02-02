pub mod board;
pub mod grid;

use clap::StructOpt;
use crossterm::terminal;
use eyre::{Context, Result};
use grid::{words::GridWords, Grid};
use std::{
  collections::HashMap,
  fs::File,
  io::{BufRead, BufReader},
  time::Instant,
};

#[derive(clap::Parser, Debug)]
#[clap(author, version, about)]
pub struct Args {
  /// The characters in the grid.  
  ///
  /// Example: "modnstedetripyij"
  #[clap(short, long)]
  pub grid: String,

  /// The optional path to a text file containing words delimited by newline characters.
  ///
  /// Default: Uses dictionary baked into executable.
  #[clap(short, long)]
  pub dictionary: Option<String>,

  /// The shortest word length to match.
  #[clap(short, long, default_value = "3")]
  pub min_word_len: usize,

  /// A space separated list of comma separated tuples to determine which neighboring cells
  /// to use when checking a cell.
  ///
  /// Defaults to all neighbors.
  #[clap(short, long, default_value = "-1,-1 0,-1 -1,0 1,1 0,1 1,0 -1,1 1,-1")]
  pub neighbors_kernel: String,
}

fn main() -> Result<()> {
  let time = Instant::now();
  let args = Args::parse();
  let neighbors_kernel = parse_neighbors_kernel(&args.neighbors_kernel)?;
  let grid = Grid::new(
    args.grid.to_lowercase(),
    args.min_word_len,
    neighbors_kernel,
  )?;

  let words = get_words(&args.dictionary)?;

  let mut width = 0;
  let width_max = terminal::size()?.0 as usize;

  let mut words = GridWords::new(&grid, &words).collect::<Vec<_>>();

  words.sort_unstable();
  words.dedup();

  let mut by_len = HashMap::new();

  for word in &words {
    let word_width = word.len() + 2;
    if word_width + width > width_max && width > 0 {
      println!();
      width = 0;
    }

    print!("{word}, ");
    width += word_width;
    *by_len.entry(word.len()).or_insert(0) += 1;
  }

  println!();
  println!();

  let mut by_len = by_len
    .into_iter()
    .map(|(len, count)| (len, count))
    .collect::<Vec<_>>();

  by_len.sort_unstable_by_key(|(len, _)| *len);

  for (len, count) in by_len {
    println!("len {len}: {count}");
  }

  println!("total: {}", words.len());
  println!("done in {:?}", time.elapsed());

  Ok(())
}

fn get_words(dictionary: &Option<String>) -> Result<Vec<String>> {
  Ok(match dictionary {
    // Read lines from dictionary file.
    Some(dictionary) => BufReader::new(File::open(dictionary)?)
      .lines()
      .collect::<std::io::Result<Vec<_>>>()?
      .into_iter()
      .map(|word| word.trim().to_lowercase())
      .filter(|word| !word.is_empty())
      .collect(),
    // Read lines from embedded dictionary.
    None => include_str!(concat!(env!("OUT_DIR"), "/words.txt"))
      .lines()
      .map(|word| word.trim().to_lowercase())
      .filter(|word| !word.is_empty())
      .collect(),
  })
}

fn parse_neighbors_kernel(kernel: &str) -> Result<Vec<(isize, isize)>> {
  kernel
    .split(' ')
    .filter_map(|kernel| {
      kernel.split_once(',').map(|(x, y)| {
        Ok::<_, eyre::Error>((
          x.parse::<isize>()
            .wrap_err(format!("Invalid number `{x}`"))?,
          y.parse::<isize>()
            .wrap_err(format!("Invalid number `{y}`"))?,
        ))
      })
    })
    .collect::<Result<Vec<_>>>()
}
