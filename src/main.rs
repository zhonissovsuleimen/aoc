use aoc_trait::{AOC, Day};
use aoc15::AOC15;

fn main() {
  let last_day = AOC15::last();
  println!("AOC {} Day {}", AOC15::YEAR, last_day.day());
  
  let mut no_solutions = true;

  if let Some(solution) = last_day.solution() {
    println!("Soltuion: \n{}\n", solution);
    no_solutions = false;
  }

  if let Some(solution_extra) = last_day.solution_extra() {
    println!("Soltuion Extra: \n{}\n", solution_extra);
    no_solutions = false;
  }

  if no_solutions {
    println!("No solutions yet");
  }
}
