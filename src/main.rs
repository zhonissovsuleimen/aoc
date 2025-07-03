use aoc_trait::{AOC, Day};
use aoc15::AOC15;

fn main() {
  let last_day = AOC15::last();
  println!("AOC {} Day {}", AOC15::YEAR, last_day.day());
  
  println!("Soltuion: \n{}\n", last_day.solution());
  println!("Soltuion Extra: \n{}\n", last_day.solution_extra());
}
