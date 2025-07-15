use std::collections::{HashMap, HashSet};

use aoc_trait::Day;

pub struct D19;

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

    None
  }
}
