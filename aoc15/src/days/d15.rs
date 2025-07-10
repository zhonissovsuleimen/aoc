use aoc_trait::Day;

/*
--- Day 15: Science for Hungry People ---

Today, you set out on the task of perfecting your milk-dunking cookie recipe. All you have to do is find the right balance of ingredients.

Your recipe leaves room for exactly 100 teaspoons of ingredients. You make a list of the remaining ingredients you could use to finish the recipe (your puzzle input) and their properties per teaspoon:

    capacity (how well it helps the cookie absorb milk)
    durability (how well it keeps the cookie intact when full of milk)
    flavor (how tasty it makes the cookie)
    texture (how it improves the feel of the cookie)
    calories (how many calories it adds to the cookie)

You can only measure ingredients in whole-teaspoon amounts accurately, and you have to be accurate so you can reproduce your results in the future. The total score of a cookie can be found by adding up each of the properties (negative totals become 0) and then multiplying together everything except calories.

For instance, suppose you have these two ingredients:

Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3

Then, choosing to use 44 teaspoons of butterscotch and 56 teaspoons of cinnamon (because the amounts of each ingredient must add up to 100) would result in a cookie with the following properties:

    A capacity of 44*-1 + 56*2 = 68
    A durability of 44*-2 + 56*3 = 80
    A flavor of 44*6 + 56*-2 = 152
    A texture of 44*3 + 56*-1 = 76

Multiplying these together (68 * 80 * 152 * 76, ignoring calories for now) results in a total score of 62842880, which happens to be the best score possible given these ingredients. If any properties had produced a negative total, it would have instead become zero, causing the whole score to multiply to zero.

Given the ingredients in your kitchen and their properties, what is the total score of the highest-scoring cookie you can make?
*/

/*
--- Part Two ---

Your cookie recipe becomes wildly popular! Someone asks if you can make another recipe that has exactly 500 calories per cookie (so they can use it as a meal replacement). Keep the rest of your award-winning process the same (100 teaspoons, same ingredients, same scoring system).

For example, given the ingredients above, if you had instead selected 40 teaspoons of butterscotch and 60 teaspoons of cinnamon (which still adds to 100), the total calorie count would be 40*8 + 60*3 = 500. The total score would go down, though: only 57600000, the best you can do in such trying circumstances.

Given the ingredients in your kitchen and their properties, what is the total score of the highest-scoring cookie you can make with a calorie total of 500?
*/
pub struct D15;

impl Day for D15 {
  fn day(&self) -> usize {
    15
  }

  fn input(&self) -> Option<String> {
    let path = format!("{}/src/inputs/d15.txt", env!("CARGO_MANIFEST_DIR"));
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
    let mut ingredients = vec![];

    for line in input.lines() {
      let tokens = line.split(' ').collect::<Vec<&str>>();

      match tokens.as_slice() {
        [
          _,
          "capacity", capacity,
          "durability", durability,
          "flavor", flavor,
          "texture", texture,
          "calories", calories,
        ] => {
          let capacity = capacity[..capacity.len() - 1].parse::<i64>().unwrap();
          let durability = durability[..durability.len() - 1].parse::<i64>().unwrap();
          let flavor = flavor[..flavor.len() - 1].parse::<i64>().unwrap();
          let texture = texture[..texture.len() - 1].parse::<i64>().unwrap();
          let calories = calories.parse::<i64>().unwrap();

          ingredients.push((capacity, durability, flavor, texture, calories));
        }
        _ => unreachable!(),
      }
    }

    let mut counts = vec![0usize; ingredients.len()];
    let mut best = 0;

    let mut done = false;
    while !done {
      while counts.iter().sum::<usize>() > 100 {
        if counts[counts.len() - 1] == 100 {
          done = true;
          break;
        }

        for i in 0..counts.len() - 1 {
          if counts[i] != 0 {
            counts[i] = 0;
            counts[i + 1] += 1;
            break;
          }
        }

        if let Some(last_count) = counts.last() {
          done = *last_count == 100;
        }
      }

      let cookie = ingredients.iter().enumerate().fold((0, 0, 0, 0, 0), |mut acc, (i, x)| {
        let count = counts[i] as i64;
        acc.0 += x.0 * count;
        acc.1 += x.1 * count;
        acc.2 += x.2 * count;
        acc.3 += x.3 * count;
        acc.4 += x.4 * count;

        acc
      });

      let score = cookie.0.max(0) * cookie.1.max(0) * cookie.2.max(0) * cookie.3.max(0);
      best = best.max(score);

      counts[0] += 1;
    }

    Some(best.to_string())
  }

  fn solution_extra(&self) -> Option<String> {
    let Some(input) = self.input() else {
      return None;
    };
    let mut ingredients = vec![];

    for line in input.lines() {
      let tokens = line.split(' ').collect::<Vec<&str>>();

      match tokens.as_slice() {
        [
          _,
          "capacity", capacity,
          "durability", durability,
          "flavor", flavor,
          "texture", texture,
          "calories", calories,
        ] => {
          let capacity = capacity[..capacity.len() - 1].parse::<i64>().unwrap();
          let durability = durability[..durability.len() - 1].parse::<i64>().unwrap();
          let flavor = flavor[..flavor.len() - 1].parse::<i64>().unwrap();
          let texture = texture[..texture.len() - 1].parse::<i64>().unwrap();
          let calories = calories.parse::<i64>().unwrap();

          ingredients.push((capacity, durability, flavor, texture, calories));
        }
        _ => unreachable!(),
      }
    }

    let mut counts = vec![0usize; ingredients.len()];
    let mut best = 0;

    let mut done = false;
    while !done {
      while counts.iter().sum::<usize>() > 100 {
        if counts[counts.len() - 1] == 100 {
          done = true;
          break;
        }

        for i in 0..counts.len() - 1 {
          if counts[i] != 0 {
            counts[i] = 0;
            counts[i + 1] += 1;
            break;
          }
        }

        if let Some(last_count) = counts.last() {
          done = *last_count == 100;
        }
      }

      let cookie = ingredients.iter().enumerate().fold((0, 0, 0, 0, 0), |mut acc, (i, x)| {
        let count = counts[i] as i64;
        acc.0 += x.0 * count;
        acc.1 += x.1 * count;
        acc.2 += x.2 * count;
        acc.3 += x.3 * count;
        acc.4 += x.4 * count;

        acc
      });

      let score = cookie.0.max(0) * cookie.1.max(0) * cookie.2.max(0) * cookie.3.max(0);
      if cookie.4 == 500 {
        best = best.max(score);
      }

      counts[0] += 1;
    }

    Some(best.to_string())
  }
}
