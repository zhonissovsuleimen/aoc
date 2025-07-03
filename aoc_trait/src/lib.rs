pub trait AOC {
  const YEAR: usize;
  fn last() -> impl Day;
  fn all() -> Vec<impl Day>;
}

pub trait Day {
  const INPUT: &'static str;
  fn day(&self) -> usize;
  fn solution(&self) -> String;
  fn solution_extra(&self) -> String;
}