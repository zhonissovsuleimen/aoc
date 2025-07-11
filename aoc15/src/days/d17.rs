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

/*
--- Part Two ---

While playing with all the containers in the kitchen, another load of eggnog arrives! The shipping and receiving department is requesting as many containers as you can spare.

Find the minimum number of containers that can exactly fit all 150 liters of eggnog. How many different ways can you fill that number of containers and still hold exactly 150 litres?

In the example above, the minimum number of containers was two. There were three ways to use that many containers, and so the answer there would be 3.
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

    let mut containers = vec![];

    for line in input.lines() {
      let volume = line.parse::<usize>().unwrap();
      containers.push(volume);
    }

    let target_volume = 150;
    let mut minimum_containers_dp = vec![None; target_volume + 1];
    minimum_containers_dp[0] = Some(0usize);

    for &container in &containers {
      for i in (container..=target_volume).rev() {
        if let Some(prev_min) = minimum_containers_dp[i - container] {
          let new_min = minimum_containers_dp[i]
            .map_or(prev_min + 1, |existing_val| existing_val.min(prev_min + 1));
          minimum_containers_dp[i] = Some(new_min);
        }
      }
    }

    let min_containers_target = minimum_containers_dp[target_volume].unwrap();

    //solutions for volumes up to a target, but instead they are split into number of cups, up to minimum, cus we don't need to calculate if they exceed that
    let mut dp_solutions = vec![vec![0usize; min_containers_target + 1]; target_volume + 1];
    dp_solutions[0] = vec![1; min_containers_target + 1];

    for &container in &containers {
      for i in (container..=target_volume).rev() {
        for container_count in 1..=min_containers_target {
          dp_solutions[i][container_count] += dp_solutions[i - container][container_count - 1];
        }
      }
    }

    let target_solution = dp_solutions[target_volume][min_containers_target];
    Some(target_solution.to_string())
  }
}
