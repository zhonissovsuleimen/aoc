use aoc_trait::Day;

pub struct D1;

impl Day for D1 {
  const DAY: usize = 1;

  fn solution(&self) -> String {
    format!("Solution for day {}", Self::DAY)
  }

  fn solution_extra(&self) -> String {
    format!("Solution for extra day {}", Self::DAY)
  }
}
