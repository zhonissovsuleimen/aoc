use aoc_trait::Day;

/*
--- Day 14: Reindeer Olympics ---

This year is the Reindeer Olympics! Reindeer can fly at high speeds, but must rest occasionally to recover their energy. Santa would like to know which of his reindeer is fastest, and so he has them race.

Reindeer can only either be flying (always at their top speed) or resting (not moving at all), and always spend whole seconds in either state.

For example, suppose you have the following Reindeer:

    Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
    Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.

After one second, Comet has gone 14 km, while Dancer has gone 16 km. After ten seconds, Comet has gone 140 km, while Dancer has gone 160 km. On the eleventh second, Comet begins resting (staying at 140 km), and Dancer continues on for a total distance of 176 km. On the 12th second, both reindeer are resting. They continue to rest until the 138th second, when Comet flies for another ten seconds. On the 174th second, Dancer flies for another 11 seconds.

In this example, after the 1000th second, both reindeer are resting, and Comet is in the lead at 1120 km (poor Dancer has only gotten 1056 km by that point). So, in this situation, Comet would win (if the race ended at 1000 seconds).

Given the descriptions of each reindeer (in your puzzle input), after exactly 2503 seconds, what distance has the winning reindeer traveled?
*/

pub struct D14;

impl Day for D14 {
  fn day(&self) -> usize {
    14
  }

  fn input(&self) -> Option<String> {
    let path = format!("{}/src/inputs/d14.txt", env!("CARGO_MANIFEST_DIR"));
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

    let mut fleet = vec![];

    for line in input.lines() {
      let tokens = line.split(' ').collect::<Vec<&str>>();

      match tokens.as_slice() {
        [name, "can", "fly", speed, "km/s", "for", endurance, .., "for", cooldown, "seconds."] => {
          fleet.push(Reindeer {
            name: name.to_string(),
            speed: speed.parse().unwrap(),
            endurance: endurance.parse().unwrap(),
            cooldown: cooldown.parse().unwrap(),
          });
        }
        _ => unreachable!(),
      }
    }

    let winner_distance = fleet.iter().map(|rd| rd.fly(2503)).max().unwrap();
    Some(winner_distance.to_string())
  }

  fn solution_extra(&self) -> Option<String> {
    let Some(input) = self.input() else {
      return None;
    };

    None
  }
}

struct Reindeer {
  name: String,
  speed: u32,
  endurance: u32,
  cooldown: u32,
}

impl Reindeer {
  fn fly(&self, total_time: u32) -> u32{
    let mut timer = 0;
    let mut distance = 0;

    while timer < total_time {
      let time_left = total_time - timer;

      if time_left > self.endurance {
        distance += self.endurance * self.speed;
        timer += self.endurance;
      } else {
        return distance + time_left * self.speed;
      }

      if time_left > self.cooldown {
        timer += self.cooldown;
      } else {
        return distance;
      }
    }

    distance
  }
}