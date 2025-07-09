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

/*
--- Part Two ---

Seeing how reindeer move in bursts, Santa decides he's not pleased with the old scoring system.

Instead, at the end of each second, he awards one point to the reindeer currently in the lead. (If there are multiple reindeer tied for the lead, they each get one point.) He keeps the traditional 2503 second time limit, of course, as doing otherwise would be entirely ridiculous.

Given the example reindeer from above, after the first second, Dancer is in the lead and gets one point. He stays in the lead until several seconds into Comet's second burst: after the 140th second, Comet pulls into the lead and gets his first point. Of course, since Dancer had been in the lead for the 139 seconds before that, he has accumulated 139 points by the 140th second.

After the 1000th second, Dancer has accumulated 689 points, while poor Comet, our old champion, only has 312. So, with the new scoring system, Dancer would win (if the race ended at 1000 seconds).

Again given the descriptions of each reindeer (in your puzzle input), after exactly 2503 seconds, how many points does the winning reindeer have?
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

    let max_distabce_per_sec = (1..=2503)
      .into_iter()
      .map(|time| 
        fleet
        .iter()
        .map(|rd| rd.fly(time))
        .max().unwrap()
      )
      .collect::<Vec<u32>>();

    let final_points_per_reindeer = fleet
      .iter()
      .map(|rd| (1..=2503)
        .into_iter()
        .map(|time| if max_distabce_per_sec[time - 1] == rd.fly(time as u32) { 1u32 } else { 0u32 })
        .sum::<u32>()
      )
      .collect::<Vec<u32>>();

    let final_winner_score = final_points_per_reindeer.iter().max().unwrap();
    Some(final_winner_score.to_string())
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