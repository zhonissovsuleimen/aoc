pub trait AOC {
  const YEAR: usize;
  fn last() -> String;
  fn all() -> Vec<String>;
}