use aoc_trait::Day;

pub struct D17;

/*
--- Day 17: No Such Thing as Too Much ---

The elves bought too much eggnog again - 150 liters this time. To fit it all into your refrigerator, you'll need to move it into smaller containers. You take an inventory of the capacities of the available containers.

For example, suppose you have containers of size 20, 15, 10, 5, and 5 liters. If you need to store 25 liters, there are four ways to do it:

    15 and 10
    20 and 5 (the first 5)
    20 and 5 (the second 5)
    15, 5, and 5

Filling all containers entirely, how many different combinations of containers can exactly fit all 150 liters of eggnog?
*/
impl Day for D17 {
  fn day(&self) -> usize {
    17
  }

  fn input(&self) -> Option<String> {
    let path = format!("{}/src/inputs/d17.txt", env!("CARGO_MANIFEST_DIR"));
    let result = std::fs::read_to_string(path);

    if let Ok(value) = result {
      Some(value)
    } else {
      None
    }
  }

  fn solution(&self) -> Option<String> {
    let Some(input) = self.input() else {
      return None;
    };

    let mut containers = vec![];

    for line in input.lines() {
      let volume = line.parse::<usize>().unwrap();
      containers.push(volume);
    }

    let target_volume = 150;
    let mut dp_solutions = vec![0; target_volume + 1];
    dp_solutions[0] = 1;

    for &container in &containers {
      for i in (container..=target_volume).rev() {
        dp_solutions[i] += dp_solutions[i - container];
      }
    }

    let target_solution = dp_solutions[target_volume];
    Some(target_solution.to_string())
  }

  fn solution_extra(&self) -> Option<String> {
    let Some(input) = self.input() else {
      return None;
    };

    None
  }
}