use std::collections::{HashMap, HashSet};

use aoc_trait::Day;

pub struct D9;

/*
Every year, Santa manages to deliver all of his presents in a single night.

This year, however, he has some new locations to visit; his elves have provided him the distances between every pair of locations. He can start and end at any two (different) locations he wants, but he must visit each location exactly once. What is the shortest distance he can travel to achieve this?

For example, given the following distances:

London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141

The possible routes are therefore:

Dublin -> London -> Belfast = 982
London -> Dublin -> Belfast = 605
London -> Belfast -> Dublin = 659
Dublin -> Belfast -> London = 659
Belfast -> Dublin -> London = 605
Belfast -> London -> Dublin = 982

The shortest of these is London -> Dublin -> Belfast = 605, and so the answer is 605 in this example.

What is the distance of the shortest route?
*/

/*
--- Part Two ---

The next year, just to show off, Santa decides to take the route with the longest distance instead.

He can still start and end at any two (different) locations he wants, and he still must visit each location exactly once.

For example, given the distances above, the longest route would be 982 via (for example) Dublin -> London -> Belfast.

What is the distance of the longest route?
*/

impl Day for D9 {
  fn day(&self) -> usize {
    9
  }

  fn input(&self) -> Option<String> {
    let path = format!("{}/src/inputs/d9.txt", env!("CARGO_MANIFEST_DIR"));
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

    let mut cities = HashSet::<String>::new();
    let mut distance_map = HashMap::<(String, String), u32>::new();

    for line in input.lines() {
      let tokens = line.split(' ').collect::<Vec<&str>>();

      match tokens.as_slice() {
        [from, "to", to, "=", distance] => {
          let from = from.to_string();
          let to = to.to_string();
          let distance = distance.parse().unwrap();

          distance_map.insert((from.clone(), to.clone()), distance);

          distance_map.insert((to.clone(), from.clone()), distance);

          cities.insert(from);
          cities.insert(to);
        }
        _ => unreachable!(),
      }
    }

    let paths = gen_travel_paths(cities.into_iter().collect());

    let mut min = u32::MAX;
    for path in paths {
      let mut local_min = 0u32;

      for window in path.windows(2) {
        let from = &window[0];
        let to = &window[1];

        local_min = local_min.saturating_add(*distance_map.get(&(from.clone(), to.clone())).unwrap_or(&u32::MAX));
      }

      min = min.min(local_min);
    }

    Some(min.to_string())
  }

  fn solution_extra(&self) -> Option<String> {
    let Some(input) = self.input() else {
      return None;
    };

    let mut cities = HashSet::<String>::new();
    let mut distance_map = HashMap::<(String, String), u32>::new();

    for line in input.lines() {
      let tokens = line.split(' ').collect::<Vec<&str>>();

      match tokens.as_slice() {
        [from, "to", to, "=", distance] => {
          let from = from.to_string();
          let to = to.to_string();
          let distance = distance.parse().unwrap();

          distance_map.insert((from.clone(), to.clone()), distance);

          distance_map.insert((to.clone(), from.clone()), distance);

          cities.insert(from);
          cities.insert(to);
        }
        _ => unreachable!(),
      }
    }

    let paths = gen_travel_paths(cities.into_iter().collect());

    let mut max = 0u32;
    for path in paths {
      let mut local_max = 0u32;

      for window in path.windows(2) {
        let from = &window[0];
        let to = &window[1];

        local_max = local_max.saturating_add(*distance_map.get(&(from.clone(), to.clone())).unwrap_or(&0));
      }

      max = max.max(local_max);
    }

    Some(max.to_string())
  }
}

fn gen_travel_paths(mut cities: Vec<String>) -> Vec<Vec<String>> {
  let mut result = vec![];

  let len = cities.len();
  heaps_algorithm(&mut result, &mut cities, len);

  result
}

//https://en.wikipedia.org/wiki/Heap%27s_algorithm
fn heaps_algorithm(result: &mut Vec<Vec<String>>, cities: &mut Vec<String>, index: usize) {
  if index == 1 {
    result.push(cities.clone());
  } else {
    heaps_algorithm(result, cities, index - 1);

    for i in 0..(index - 1) {
      if index % 2 == 0 {
        cities.swap(i, index - 1);
      } else {
        cities.swap(0, index - 1);
      }

      heaps_algorithm(result, cities, index - 1);
    }
  }
}
