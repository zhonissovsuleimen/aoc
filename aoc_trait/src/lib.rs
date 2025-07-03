pub trait AOC {
  const YEAR: usize;
  fn last() -> impl Day;
}

pub trait Day {
  fn day(&self) -> usize;
  fn input(&self) -> Option<String>;
  fn solution(&self) -> Option<String>;
  fn solution_extra(&self) -> Option<String>;
}