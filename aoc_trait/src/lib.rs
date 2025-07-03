pub trait AOC {
  const YEAR: usize;
  fn last() -> impl Day;
  fn all() -> Vec<impl Day>;
}

pub trait Day {
  const DAY: usize;
  fn solution(&self) -> String;
  fn solution_extra(&self) -> String;
}