use std::collections::{HashMap, HashSet};

use aoc_trait::Day;
use rand::seq::SliceRandom;

pub struct D19;

/*
--- Day 19: Medicine for Rudolph ---

Rudolph the Red-Nosed Reindeer is sick! His nose isn't shining very brightly, and he needs medicine.

Red-Nosed Reindeer biology isn't similar to regular reindeer biology; Rudolph is going to need custom-made medicine. Unfortunately, Red-Nosed Reindeer chemistry isn't similar to regular reindeer chemistry, either.

The North Pole is equipped with a Red-Nosed Reindeer nuclear fusion/fission plant, capable of constructing any Red-Nosed Reindeer molecule you need. It works by starting with some input molecule and then doing a series of replacements, one per step, until it has the right molecule.

However, the machine has to be calibrated before it can be used. Calibration involves determining the number of molecules that can be generated in one step from a given starting point.

For example, imagine a simpler machine that supports only the following replacements:

H => HO
H => OH
O => HH

Given the replacements above and starting with HOH, the following molecules could be generated:

    HOOH (via H => HO on the first H).
    HOHO (via H => HO on the second H).
    OHOH (via H => OH on the first H).
    HOOH (via H => OH on the second H).
    HHHH (via O => HH).

So, in the example above, there are 4 distinct molecules (not five, because HOOH appears twice) after one replacement from HOH. Santa's favorite molecule, HOHOHO, can become 7 distinct molecules (over nine replacements: six from H, and three from O).

The machine replaces without regard for the surrounding characters. For example, given the string H2O, the transition H => OO would result in OO2O.

Your puzzle input describes all of the possible replacements and, at the bottom, the medicine molecule for which you need to calibrate the machine. How many distinct molecules can be created after all the different ways you can do one replacement on the medicine molecule?
*/

/*
--- Part Two ---

Now that the machine is calibrated, you're ready to begin molecule fabrication.

Molecule fabrication always begins with just a single electron, e, and applying replacements one at a time, just like the ones during calibration.

For example, suppose you have the following replacements:

e => H
e => O
H => HO
H => OH
O => HH

If you'd like to make HOH, you start with e, and then make the following replacements:

    e => O to get O
    O => HH to get HH
    H => OH (on the second H) to get HOH

So, you could make HOH after 3 steps. Santa's favorite molecule, HOHOHO, can be made in 6 steps.

How long will it take to make the medicine? Given the available replacements and the medicine molecule in your puzzle input, what is the fewest number of steps to go from e to the medicine molecule?
*/

impl Day for D19 {
  fn day(&self) -> usize {
    19
  }

  fn input(&self) -> Option<String> {
    let path = format!("{}/src/inputs/d19.txt", env!("CARGO_MANIFEST_DIR"));
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

    let mut replacements = HashMap::<String, Vec<String>>::new();
    let mut all_combos = HashSet::<String>::new();

    let line_with_replacements = input.lines().filter(|line| line.contains("=>"));

    for line in line_with_replacements {
      let tokens = line.split(' ').collect::<Vec<&str>>();

      match tokens.as_slice() {
        [input, "=>", output] => {
          let input = input.to_string();
          let output = output.to_string();
          replacements.entry(input).and_modify(|vec| vec.push(output.clone())).or_insert(vec![output]);
        },
        _ => unreachable!()
      };
    }

    let Some(molecule) = input.lines().last() else {
      return None;
    };

    let molecule = molecule.to_string();

    for (input, replacement_vec) in &replacements {
      let mut offset_id = 0;
      while let Some(id) = molecule[offset_id..].find(input) {
        let start_id = offset_id + id;
        let end_id = start_id + input.len();

        for replacement in replacement_vec {
          let mut new_molecule = molecule.clone();
          new_molecule.replace_range(id..end_id, &replacement);

          all_combos.insert(new_molecule);
        }

        offset_id = end_id + 1;
        if offset_id >= molecule.len() { 
          break;
        }
      }
    } 

    let count = all_combos.len();

    Some(count.to_string())
  }

  fn solution_extra(&self) -> Option<String> {
    let Some(input) = self.input() else {
      return None;
    };

    let mut rev_replacements = Vec::<(String, String)>::new();

    let line_with_replacements = input.lines().filter(|line| line.contains("=>"));

    for line in line_with_replacements {
      let tokens = line.split(' ').collect::<Vec<&str>>();

      match tokens.as_slice() {
        [input, "=>", output] => {
          let input = input.to_string();
          let output = output.to_string();
          rev_replacements.push((output, input));
        }
        _ => unreachable!(),
      };
    }

    let Some(molecule) = input.lines().last() else {
      return None;
    };

    let mut attempt = molecule.to_string();
    let mut count = 0;

    loop {
      let mut done = true;
      for (key, val) in &rev_replacements {
        if attempt.contains(key) {
          attempt = attempt.replacen(key, val, 1);
          done = false;
          count += 1;
        }
      }

      if done {
        if attempt == "e" {
          break;
        } else {
          count = 0;
          attempt = molecule.to_string();
          rev_replacements.shuffle(&mut rand::rng());
        }
      }
    }

    Some(count.to_string())
  }
}
