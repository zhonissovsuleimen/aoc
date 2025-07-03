use aoc_trait::{AOC, Day};
use aoc15::AOC15;

fn main() {
  println!("AOC {}", AOC15::YEAR);
  
  let last_day = AOC15::last();
  println!("Day {}", last_day.solution());
  println!("Day {} Extra", last_day.solution_extra());
}
