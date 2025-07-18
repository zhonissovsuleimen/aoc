use std::collections::HashSet;

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

/*
--- Part Two ---

The next year, to speed up the process, Santa creates a robot version of himself, Robo-Santa, to deliver presents with him.

Santa and Robo-Santa start at the same location (delivering two presents to the same starting house), then take turns moving based on instructions from the elf, who is eggnoggedly reading from the same script as the previous year.

This year, how many houses receive at least one present?

For example:

    ^v delivers presents to 3 houses, because Santa goes north, and then Robo-Santa goes south.
    ^>v< now delivers presents to 3 houses, and Santa and Robo-Santa end up back where they started.
    ^v^v^v^v^v now delivers presents to 11 houses, with Santa going one direction and Robo-Santa going the other.
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
    let Some(input) = self.input() else {
      return None;
    };

    let mut santa = HashSet::<(i32, i32)>::new();
    let mut robosanta = HashSet::<(i32, i32)>::new();
    let mut santa_coords = (0, 0);
    let mut robosanta_coords = (0, 0);

    santa.insert(santa_coords);
    robosanta.insert(robosanta_coords);
    for (i, c) in input.chars().enumerate() {
      let coords;
      let person;

      if i % 2 == 0 {
        person = &mut santa;
        coords = &mut santa_coords;
      } else {
        person = &mut robosanta;
        coords = &mut robosanta_coords;
      }

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

      person.insert(*coords);
    }

    santa.extend(robosanta);

    let result = santa.len();
    Some(result.to_string())
  }
}
