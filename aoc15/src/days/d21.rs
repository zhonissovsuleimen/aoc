use aoc_trait::Day;

pub struct D21;

/*
--- Day 21: RPG Simulator 20XX ---

Little Henry Case got a new video game for Christmas. It's an RPG, and he's stuck on a boss. He needs to know what equipment to buy at the shop. He hands you the controller.

In this game, the player (you) and the enemy (the boss) take turns attacking. The player always goes first. Each attack reduces the opponent's hit points by at least 1. The first character at or below 0 hit points loses.

Damage dealt by an attacker each turn is equal to the attacker's damage score minus the defender's armor score. An attacker always does at least 1 damage. So, if the attacker has a damage score of 8, and the defender has an armor score of 3, the defender loses 5 hit points. If the defender had an armor score of 300, the defender would still lose 1 hit point.

Your damage score and armor score both start at zero. They can be increased by buying items in exchange for gold. You start with no items and have as much gold as you need. Your total damage or armor is equal to the sum of those stats from all of your items. You have 100 hit points.

Here is what the item shop is selling:

Weapons:    Cost  Damage  Armor
Dagger        8     4       0
Shortsword   10     5       0
Warhammer    25     6       0
Longsword    40     7       0
Greataxe     74     8       0

Armor:      Cost  Damage  Armor
Leather      13     0       1
Chainmail    31     0       2
Splintmail   53     0       3
Bandedmail   75     0       4
Platemail   102     0       5

Rings:      Cost  Damage  Armor
Damage +1    25     1       0
Damage +2    50     2       0
Damage +3   100     3       0
Defense +1   20     0       1
Defense +2   40     0       2
Defense +3   80     0       3

You must buy exactly one weapon; no dual-wielding. Armor is optional, but you can't use more than one. You can buy 0-2 rings (at most one for each hand). You must use any items you buy. The shop only has one of each item, so you can't buy, for example, two rings of Damage +3.

For example, suppose you have 8 hit points, 5 damage, and 5 armor, and that the boss has 12 hit points, 7 damage, and 2 armor:

    The player deals 5-2 = 3 damage; the boss goes down to 9 hit points.
    The boss deals 7-5 = 2 damage; the player goes down to 6 hit points.
    The player deals 5-2 = 3 damage; the boss goes down to 6 hit points.
    The boss deals 7-5 = 2 damage; the player goes down to 4 hit points.
    The player deals 5-2 = 3 damage; the boss goes down to 3 hit points.
    The boss deals 7-5 = 2 damage; the player goes down to 2 hit points.
    The player deals 5-2 = 3 damage; the boss goes down to 0 hit points.

In this scenario, the player wins! (Barely.)

You have 100 hit points. The boss's actual stats are in your puzzle input. What is the least amount of gold you can spend and still win the fight?
*/

/*
--- Part Two ---

Turns out the shopkeeper is working with the boss, and can persuade you to buy whatever items he wants. The other rules still apply, and he still only has one of each item.

What is the most amount of gold you can spend and still lose the fight?
*/

impl Day for D21 {
  fn day(&self) -> usize {
    21
  }

  fn input(&self) -> Option<String> {
    let path = format!("{}/src/inputs/d21.txt", env!("CARGO_MANIFEST_DIR"));
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

    let me = Unit {
      health: 100,
      damage: 0,
      armor: 0,
    };

    let mut boss = Unit {
      health: 0,
      damage: 0,
      armor: 0,
    };

    for line in input.lines() {
      let tokens = line.split(' ').collect::<Vec<&str>>();

      match tokens.as_slice() {
        ["Hit", "Points:", hp] => {
          boss.health = hp.parse().unwrap();
        }
        ["Damage:", dmg] => {
          boss.damage = dmg.parse().unwrap();
        }
        ["Armor:", armor] => {
          boss.armor = armor.parse().unwrap();
        }
        _ => unreachable!(),
      }
    }

    let weapon_shop = vec![
      (8, 4),
      (10, 5),
      (25, 6),
      (40, 7),
      (74, 8),
    ];

    let armor_shop = vec![
      None,
      Some((13, 1)),
      Some((31, 2)),
      Some((53, 3)),
      Some((75, 4)),
      Some((102, 5)),
    ];

    let ring_shop = vec![
      None,
      None,
      Some((25, (1, 0))),
      Some((50, (2, 0))),
      Some((100, (3, 0))),
      Some((20, (0, 1))),
      Some((40, (0, 2))),
      Some((80, (0, 3))),
    ];

    let mut min_cost = u32::MAX;
    for (w_cost, dmg) in &weapon_shop {
      for armor in &armor_shop {
        let (a_cost, armor) = armor.unwrap_or((0,0));
        for i in 0..ring_shop.len() - 1 {
          for j in (i + 1)..ring_shop.len() {
            let (ra_cost, ra_stats) = ring_shop[i].unwrap_or((0, (0, 0)));
            let (rb_cost, rb_stats) = ring_shop[j].unwrap_or((0, (0, 0)));

            let local_cost = w_cost + a_cost + ra_cost + rb_cost;
            let mut new_me = me.clone();
            new_me.damage += dmg + ra_stats.0 + rb_stats.0;
            new_me.armor += armor + ra_stats.1 + rb_stats.1;

            if new_me.beats(&boss) {
              min_cost = min_cost.min(local_cost);
            }
          }
        }

      }
    }

    Some(min_cost.to_string())
  }

  fn solution_extra(&self) -> Option<String> {
    let Some(input) = self.input() else {
      return None;
    };

    let me = Unit {
      health: 100,
      damage: 0,
      armor: 0,
    };

    let mut boss = Unit {
      health: 0,
      damage: 0,
      armor: 0,
    };

    for line in input.lines() {
      let tokens = line.split(' ').collect::<Vec<&str>>();

      match tokens.as_slice() {
        ["Hit", "Points:", hp] => {
          boss.health = hp.parse().unwrap();
        }
        ["Damage:", dmg] => {
          boss.damage = dmg.parse().unwrap();
        }
        ["Armor:", armor] => {
          boss.armor = armor.parse().unwrap();
        }
        _ => unreachable!(),
      }
    }

    let weapon_shop = vec![
      (8, 4),
      (10, 5),
      (25, 6),
      (40, 7),
      (74, 8),
    ];

    let armor_shop = vec![
      None,
      Some((13, 1)),
      Some((31, 2)),
      Some((53, 3)),
      Some((75, 4)),
      Some((102, 5)),
    ];

    let ring_shop = vec![
      None,
      None,
      Some((25, (1, 0))),
      Some((50, (2, 0))),
      Some((100, (3, 0))),
      Some((20, (0, 1))),
      Some((40, (0, 2))),
      Some((80, (0, 3))),
    ];

    let mut max_cost = 0;
    for (w_cost, dmg) in &weapon_shop {
      for armor in &armor_shop {
        let (a_cost, armor) = armor.unwrap_or((0,0));
        for i in 0..ring_shop.len() - 1 {
          for j in (i + 1)..ring_shop.len() {
            let (ra_cost, ra_stats) = ring_shop[i].unwrap_or((0, (0, 0)));
            let (rb_cost, rb_stats) = ring_shop[j].unwrap_or((0, (0, 0)));

            let local_cost = w_cost + a_cost + ra_cost + rb_cost;
            let mut new_me = me.clone();
            new_me.damage += dmg + ra_stats.0 + rb_stats.0;
            new_me.armor += armor + ra_stats.1 + rb_stats.1;

            if !new_me.beats(&boss) {
              max_cost = max_cost.max(local_cost);
            }
          }
        }

      }
    }

    Some(max_cost.to_string())
  }
}

#[derive(Clone)]
struct Unit {
  health: u32,
  damage: u32,
  armor: u32,
}

impl Unit {
  fn beats(&self, enemy: &Unit) -> bool {
    let mut my_hp = self.health;
    let mut enemy_hp = enemy.health;
    loop {
      enemy_hp = enemy_hp.saturating_sub(self.damage.saturating_sub(enemy.armor).max(1));
      if enemy_hp == 0 {
        return true;
      }

      my_hp = my_hp.saturating_sub(enemy.damage.saturating_sub(self.armor).max(1));
      if my_hp == 0 {
        return false;
      }
    }
  }
}