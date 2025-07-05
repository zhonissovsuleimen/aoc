use aoc_trait::{AOC, Day};
mod days;

pub struct AOC15;

impl AOC for AOC15 {
  const YEAR: usize = 2015;

  fn last() -> impl Day {
    days::d5::D5
  }

  fn all() -> Vec<Box<dyn Day>> {
    vec![
      Box::new(days::d1::D1),
      Box::new(days::d2::D2),
      Box::new(days::d3::D3),
      Box::new(days::d5::D5),
    ]
  }
}
