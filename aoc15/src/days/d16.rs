use std::collections::HashMap;

use aoc_trait::Day;

pub struct D16;

/*
--- Day 16: Aunt Sue ---

Your Aunt Sue has given you a wonderful gift, and you'd like to send her a thank you card. However, there's a small problem: she signed it "From, Aunt Sue".

You have 500 Aunts named "Sue".

So, to avoid sending the card to the wrong person, you need to figure out which Aunt Sue (which you conveniently number 1 to 500, for sanity) gave you the gift. You open the present and, as luck would have it, good ol' Aunt Sue got you a My First Crime Scene Analysis Machine! Just what you wanted. Or needed, as the case may be.

The My First Crime Scene Analysis Machine (MFCSAM for short) can detect a few specific compounds in a given sample, as well as how many distinct kinds of those compounds there are. According to the instructions, these are what the MFCSAM can detect:

    children, by human DNA age analysis.
    cats. It doesn't differentiate individual breeds.
    Several seemingly random breeds of dog: samoyeds, pomeranians, akitas, and vizslas.
    goldfish. No other kinds of fish.
    trees, all in one group.
    cars, presumably by exhaust or gasoline or something.
    perfumes, which is handy, since many of your Aunts Sue wear a few kinds.

In fact, many of your Aunts Sue have many of these. You put the wrapping from the gift into the MFCSAM. It beeps inquisitively at you a few times and then prints out a message on ticker tape:

children: 3
cats: 7
samoyeds: 2
pomeranians: 3
akitas: 0
vizslas: 0
goldfish: 5
trees: 3
cars: 2
perfumes: 1

You make a list of the things you can remember about each Aunt Sue. Things missing from your list aren't zero - you simply don't remember the value.

What is the number of the Sue that got you the gift?
*/

/*
--- Part Two ---

As you're about to send the thank you note, something in the MFCSAM's instructions catches your eye. Apparently, it has an outdated retroencabulator, and so the output from the machine isn't exact values - some of them indicate ranges.

In particular, the cats and trees readings indicates that there are greater than that many (due to the unpredictable nuclear decay of cat dander and tree pollen), while the pomeranians and goldfish readings indicate that there are fewer than that many (due to the modial interaction of magnetoreluctance).

What is the number of the real Aunt Sue?
*/

impl Day for D16 {
  fn day(&self) -> usize {
    16
  }

  fn input(&self) -> Option<String> {
    let path = format!("{}/src/inputs/d16.txt", env!("CARGO_MANIFEST_DIR"));
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

    let mut suspects = vec![];

    for line in input.lines() {
      let tokens = line.split(' ').collect::<Vec<&str>>();

      match tokens.as_slice() {
        ["Sue", _id, a, a_value, b, b_value, c, c_value] => {
          let a_value = a_value[..a_value.len() - 1].parse::<u32>().unwrap();
          let b_value = b_value[..b_value.len() - 1].parse::<u32>().unwrap();
          let c_value = c_value.parse::<u32>().unwrap();

          let mut evidence = HashMap::<String, u32>::with_capacity(3);
          evidence.insert(a.to_string(), a_value);
          evidence.insert(b.to_string(), b_value);
          evidence.insert(c.to_string(), c_value);

          suspects.push(evidence);
        }
        _ => unreachable!(),
      }
    }

    let mut target = HashMap::new();
    target.insert(String::from("children:"), 3);
    target.insert(String::from("cats:"), 7);
    target.insert(String::from("samoyeds:"), 2);
    target.insert(String::from("pomeranians:"), 3);
    target.insert(String::from("akitas:"), 0);
    target.insert(String::from("vizslas:"), 0);
    target.insert(String::from("goldfish:"), 5);
    target.insert(String::from("trees:"), 3);
    target.insert(String::from("cars:"), 2);
    target.insert(String::from("perfumes:"), 1);

    let mut sue_id = 0;
    for (i, evidence) in suspects.into_iter().enumerate() {
      let mut found = true;
      for (name, evidence_value) in evidence.iter() {
        if let Some(target_value) = target.get(name) {
          if target_value != evidence_value {
            found = false;
          }
        }
      }

      if found {
        sue_id = i + 1;
        break;
      }
    }

    Some(sue_id.to_string())
  }

  fn solution_extra(&self) -> Option<String> {
    let Some(input) = self.input() else {
      return None;
    };

    let mut suspects = vec![];

    for line in input.lines() {
      let tokens = line.split(' ').collect::<Vec<&str>>();

      match tokens.as_slice() {
        ["Sue", _id, a, a_value, b, b_value, c, c_value] => {
          let a_value = a_value[..a_value.len() - 1].parse::<u32>().unwrap();
          let b_value = b_value[..b_value.len() - 1].parse::<u32>().unwrap();
          let c_value = c_value.parse::<u32>().unwrap();

          let mut evidence = HashMap::<String, u32>::with_capacity(3);
          evidence.insert(a.to_string(), a_value);
          evidence.insert(b.to_string(), b_value);
          evidence.insert(c.to_string(), c_value);

          suspects.push(evidence);
        }
        _ => unreachable!(),
      }
    }

    let mut target = HashMap::new();
    target.insert(String::from("children:"), 3);
    target.insert(String::from("cats:"), 7);
    target.insert(String::from("samoyeds:"), 2);
    target.insert(String::from("pomeranians:"), 3);
    target.insert(String::from("akitas:"), 0);
    target.insert(String::from("vizslas:"), 0);
    target.insert(String::from("goldfish:"), 5);
    target.insert(String::from("trees:"), 3);
    target.insert(String::from("cars:"), 2);
    target.insert(String::from("perfumes:"), 1);

    let mut sue_id = 0;
    for (i, evidence) in suspects.into_iter().enumerate() {
      let mut found = true;
      for (name, evidence_value) in evidence.iter() {
        if let Some(target_value) = target.get(name) {
          match name.as_str() {
            "cats:" | "trees:" => {
              if target_value >= evidence_value {
                found = false;
              }
            }
            "pomeranians:" | "goldfish:" => {
              if target_value <= evidence_value {
                found = false;
              }
            }
            _ => {
              if target_value != evidence_value {
                found = false;
              }
            }
          }
        }
      }

      if found {
        sue_id = i + 1;
        break;
      }
    }

    Some(sue_id.to_string())
  }
}