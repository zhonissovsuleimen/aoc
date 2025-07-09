use std::collections::{HashMap, HashSet};

use aoc_trait::Day;

pub struct D13;


/*
--- Day 13: Knights of the Dinner Table ---

In years past, the holiday feast with your family hasn't gone so well. Not everyone gets along! This year, you resolve, will be different. You're going to find the optimal seating arrangement and avoid all those awkward conversations.

You start by writing up a list of everyone invited and the amount their happiness would increase or decrease if they were to find themselves sitting next to each other person. You have a circular table that will be just big enough to fit everyone comfortably, and so each person will have exactly two neighbors.

For example, suppose you have only four attendees planned, and you calculate their potential happiness as follows:

Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.

Then, if you seat Alice next to David, Alice would lose 2 happiness units (because David talks so much), but David would gain 46 happiness units (because Alice is such a good listener), for a total change of 44.

If you continue around the table, you could then seat Bob next to Alice (Bob gains 83, Alice gains 54). Finally, seat Carol, who sits next to Bob (Carol gains 60, Bob loses 7) and David (Carol gains 55, David gains 41). The arrangement looks like this:

     +41 +46
+55   David    -2
Carol       Alice
+60    Bob    +54
     -7  +83

After trying every other seating arrangement in this hypothetical scenario, you find that this one is the most optimal, with a total change in happiness of 330.

What is the total change in happiness for the optimal seating arrangement of the actual guest list?
*/


/*
--- Part Two ---

In all the commotion, you realize that you forgot to seat yourself. At this point, you're pretty apathetic toward the whole thing, and your happiness wouldn't really go up or down regardless of who you sit next to. You assume everyone else would be just as ambivalent about sitting next to you, too.

So, add yourself to the list, and give all happiness relationships that involve you a score of 0.

What is the total change in happiness for the optimal seating arrangement that actually includes yourself?
*/

impl Day for D13 {
  fn day(&self) -> usize {
    13
  }

  fn input(&self) -> Option<String> {
    let path = format!("{}/src/inputs/d13.txt", env!("CARGO_MANIFEST_DIR"));
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

    let mut guests = HashSet::<String>::new();
    let mut hapiness_map = HashMap::<(String, String), i32>::new();

    for line in input.lines() {
      let tokens = line.split(' ').collect::<Vec<&str>>();

      match tokens.as_slice() {
        [from, "would", effect, num, "happiness", "units", "by", "sitting", "next", "to", to] => {
          let from = from.to_string();
          let to = to[..to.len()-1].to_string();
          let num = num.parse::<i32>().unwrap();
          let sign = if *effect == "gain" { 1 } else { -1 };

          hapiness_map.entry((from.clone(), to.clone())).and_modify(|value| *value += num * sign).or_insert(num * sign);
          hapiness_map.entry((to.clone(), from.clone())).and_modify(|value| *value += num * sign).or_insert(num * sign);

          guests.insert(from);
          guests.insert(to);
        }
        _ => unreachable!(),
      }
    }

    let mut paths = gen_sitting_order(guests.into_iter().collect());

    for path in paths.iter_mut() {
      let first = path.first().unwrap();
      path.push(first.clone());
    }

    let mut max = 0i32;
    for path in paths {
      let mut local_max = 0i32;

      for window in path.windows(2) {
        let from = &window[0];
        let to = &window[1];

        local_max = local_max.saturating_add(*hapiness_map.get(&(from.clone(), to.clone())).unwrap_or(&0));
      }

      max = max.max(local_max);
    }

    Some(max.to_string())
  }

  fn solution_extra(&self) -> Option<String> {
    let Some(input) = self.input() else {
      return None;
    };

    let mut guests = HashSet::<String>::new();
    let mut hapiness_map = HashMap::<(String, String), i32>::new();

    for line in input.lines() {
      let tokens = line.split(' ').collect::<Vec<&str>>();

      match tokens.as_slice() {
        [from, "would", effect, num, "happiness", "units", "by", "sitting", "next", "to", to] => {
          let from = from.to_string();
          let to = to[..to.len()-1].to_string();
          let num = num.parse::<i32>().unwrap();
          let sign = if *effect == "gain" { 1 } else { -1 };

          hapiness_map.entry((from.clone(), to.clone())).and_modify(|value| *value += num * sign).or_insert(num * sign);
          hapiness_map.entry((to.clone(), from.clone())).and_modify(|value| *value += num * sign).or_insert(num * sign);

          guests.insert(from);
          guests.insert(to);
        }
        _ => unreachable!(),
      }
    }

    guests.insert("Me".to_string());
    for guest in &guests {
      hapiness_map.insert(("Me".to_string(), guest.clone()), 0);
      hapiness_map.insert((guest.clone(), "Me".to_string()), 0);
    }

    let mut paths = gen_sitting_order(guests.into_iter().collect());

    for path in paths.iter_mut() {
      let first = path.first().unwrap();
      path.push(first.clone());
    }

    let mut max = 0i32;
    for path in paths {
      let mut local_max = 0i32;

      for window in path.windows(2) {
        let from = &window[0];
        let to = &window[1];

        local_max = local_max.saturating_add(*hapiness_map.get(&(from.clone(), to.clone())).unwrap_or(&0));
      }

      max = max.max(local_max);
    }

    Some(max.to_string())
  }
}

fn gen_sitting_order(mut cities: Vec<String>) -> Vec<Vec<String>> {
  let mut result = vec![];

  let len = cities.len();
  heaps_algorithm(&mut result, &mut cities, len);

  result
}

//https://en.wikipedia.org/wiki/Heap%27s_algorithm
fn heaps_algorithm(result: &mut Vec<Vec<String>>, guests: &mut Vec<String>, index: usize) {
  if index == 1 {
    result.push(guests.clone());
  } else {
    heaps_algorithm(result, guests, index - 1);

    for i in 0..(index - 1) {
      if index % 2 == 0 {
        guests.swap(i, index - 1);
      } else {
        guests.swap(0, index - 1);
      }

      heaps_algorithm(result, guests, index - 1);
    }
  }
}
