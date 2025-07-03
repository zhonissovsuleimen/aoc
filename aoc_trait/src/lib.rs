pub trait AOC {
  const YEAR: usize;
  fn last() -> impl Day;
}

pub trait Day {
  const INPUT: &'static str;
  fn day(&self) -> usize;
  fn solution(&self) -> Option<String>;
  fn solution_extra(&self) -> Option<String>;
}