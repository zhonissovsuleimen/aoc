use std::{collections::{HashMap, HashSet}, result};

use aoc_trait::Day;

pub struct D3;

/*
--- Day 3: Perfectly Spherical Houses in a Vacuum ---

Santa is delivering presents to an infinite two-dimensional grid of houses.

He begins by delivering a present to the house at his starting location, and then an elf at the North Pole calls him via radio and tells him where to move next. Moves are always exactly one house to the north (^), south (v), east (>), or west (<). After each move, he delivers another present to the house at his new location.

However, the elf back at the north pole has had a little too much eggnog, and so his directions are a little off, and Santa ends up visiting some houses more than once. How many houses receive at least one present?

For example:

    > delivers presents to 2 houses: one at the starting location, and one to the east.
    ^>v< delivers presents to 4 houses in a square, including twice to the house at his starting/ending location.
    ^v^v^v^v^v delivers a bunch of presents to some very lucky children at only 2 houses.
*/

impl Day for D3 {
  fn day(&self) -> usize {
    3
  }

  fn input(&self) -> Option<String> {
    let path = format!("{}/src/inputs/d3.txt", env!("CARGO_MANIFEST_DIR"));
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

    let mut hset = HashSet::<(i32, i32)>::new();
    let mut coords = (0, 0);

    hset.insert(coords);
    for c in input.chars() {
      coords.0 += match c {
        '>' => 1,
        '<' => -1,
        _ => 0
      };

      coords.1 += match c {
        '^' => 1,
        'v' => -1,
        _ => 0
      };

      hset.insert(coords);
    }

    let result = hset.len();
    Some(result.to_string())
  }

  fn solution_extra(&self) -> Option<String> {
    None
  }
}
