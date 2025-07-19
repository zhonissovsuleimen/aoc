use std::collections::BinaryHeap;

use aoc_trait::Day;

pub struct D22;

/*
--- Day 22: Wizard Simulator 20XX ---

Little Henry Case decides that defeating bosses with swords and stuff is boring. Now he's playing the game with a wizard. Of course, he gets stuck on another boss and needs your help again.

In this version, combat still proceeds with the player and the boss taking alternating turns. The player still goes first. Now, however, you don't get any equipment; instead, you must choose one of your spells to cast. The first character at or below 0 hit points loses.

Since you're a wizard, you don't get to wear armor, and you can't attack normally. However, since you do magic damage, your opponent's armor is ignored, and so the boss effectively has zero armor as well. As before, if armor (from a spell, in this case) would reduce damage below 1, it becomes 1 instead - that is, the boss' attacks always deal at least 1 damage.

On each of your turns, you must select one of your spells to cast. If you cannot afford to cast any spell, you lose. Spells cost mana; you start with 500 mana, but have no maximum limit. You must have enough mana to cast a spell, and its cost is immediately deducted when you cast it. Your spells are Magic Missile, Drain, Shield, Poison, and Recharge.

    Magic Missile costs 53 mana. It instantly does 4 damage.
    Drain costs 73 mana. It instantly does 2 damage and heals you for 2 hit points.
    Shield costs 113 mana. It starts an effect that lasts for 6 turns. While it is active, your armor is increased by 7.
    Poison costs 173 mana. It starts an effect that lasts for 6 turns. At the start of each turn while it is active, it deals the boss 3 damage.
    Recharge costs 229 mana. It starts an effect that lasts for 5 turns. At the start of each turn while it is active, it gives you 101 new mana.

Effects all work the same way. Effects apply at the start of both the player's turns and the boss' turns. Effects are created with a timer (the number of turns they last); at the start of each turn, after they apply any effect they have, their timer is decreased by one. If this decreases the timer to zero, the effect ends. You cannot cast a spell that would start an effect which is already active. However, effects can be started on the same turn they end.

For example, suppose the player has 10 hit points and 250 mana, and that the boss has 13 hit points and 8 damage:

-- Player turn --
- Player has 10 hit points, 0 armor, 250 mana
- Boss has 13 hit points
Player casts Poison.

-- Boss turn --
- Player has 10 hit points, 0 armor, 77 mana
- Boss has 13 hit points
Poison deals 3 damage; its timer is now 5.
Boss attacks for 8 damage.

-- Player turn --
- Player has 2 hit points, 0 armor, 77 mana
- Boss has 10 hit points
Poison deals 3 damage; its timer is now 4.
Player casts Magic Missile, dealing 4 damage.

-- Boss turn --
- Player has 2 hit points, 0 armor, 24 mana
- Boss has 3 hit points
Poison deals 3 damage. This kills the boss, and the player wins.

Now, suppose the same initial conditions, except that the boss has 14 hit points instead:

-- Player turn --
- Player has 10 hit points, 0 armor, 250 mana
- Boss has 14 hit points
Player casts Recharge.

-- Boss turn --
- Player has 10 hit points, 0 armor, 21 mana
- Boss has 14 hit points
Recharge provides 101 mana; its timer is now 4.
Boss attacks for 8 damage!

-- Player turn --
- Player has 2 hit points, 0 armor, 122 mana
- Boss has 14 hit points
Recharge provides 101 mana; its timer is now 3.
Player casts Shield, increasing armor by 7.

-- Boss turn --
- Player has 2 hit points, 7 armor, 110 mana
- Boss has 14 hit points
Shield's timer is now 5.
Recharge provides 101 mana; its timer is now 2.
Boss attacks for 8 - 7 = 1 damage!

-- Player turn --
- Player has 1 hit point, 7 armor, 211 mana
- Boss has 14 hit points
Shield's timer is now 4.
Recharge provides 101 mana; its timer is now 1.
Player casts Drain, dealing 2 damage, and healing 2 hit points.

-- Boss turn --
- Player has 3 hit points, 7 armor, 239 mana
- Boss has 12 hit points
Shield's timer is now 3.
Recharge provides 101 mana; its timer is now 0.
Recharge wears off.
Boss attacks for 8 - 7 = 1 damage!

-- Player turn --
- Player has 2 hit points, 7 armor, 340 mana
- Boss has 12 hit points
Shield's timer is now 2.
Player casts Poison.

-- Boss turn --
- Player has 2 hit points, 7 armor, 167 mana
- Boss has 12 hit points
Shield's timer is now 1.
Poison deals 3 damage; its timer is now 5.
Boss attacks for 8 - 7 = 1 damage!

-- Player turn --
- Player has 1 hit point, 7 armor, 167 mana
- Boss has 9 hit points
Shield's timer is now 0.
Shield wears off, decreasing armor by 7.
Poison deals 3 damage; its timer is now 4.
Player casts Magic Missile, dealing 4 damage.

-- Boss turn --
- Player has 1 hit point, 0 armor, 114 mana
- Boss has 2 hit points
Poison deals 3 damage. This kills the boss, and the player wins.

You start with 50 hit points and 500 mana points. The boss's actual stats are in your puzzle input. What is the least amount of mana you can spend and still win the fight? (Do not include mana recharge effects as "spending" negative mana.)
*/

/*
--- Part Two ---

On the next run through the game, you increase the difficulty to hard.

At the start of each player turn (before any other effects apply), you lose 1 hit point. If this brings you to or below 0 hit points, you lose.

With the same starting stats for you and the boss, what is the least amount of mana you can spend and still win the fight?
*/

impl Day for D22 {
  fn day(&self) -> usize {
    22
  }

  fn input(&self) -> Option<String> {
    let path = format!("{}/src/inputs/d22.txt", env!("CARGO_MANIFEST_DIR"));
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

    let mut boss = Boss {
      health: 0,
      damage: 0,
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
        _ => unreachable!(),
      }
    }

    let mut heap = BinaryHeap::<State>::new();
    let initial_state = State {
      hero_turn: true,
      mana_spent: 0,
      hero: Hero {
        health: 50,
        shield: 0,
        mana: 500,
      },
      boss,
      effects: Vec::new(),
    };
    heap.push(initial_state);

    let spellbook = vec![
      (Spell::MagicMissile, 53),
      (Spell::Drain, 73),
      (Spell::Shield, 113),
      (Spell::Poison, 173),
      (Spell::Recharge, 229),
    ];

    let mut min_mana_spent = i32::MAX;
    while let Some(mut pop) = heap.pop() {
      if pop.mana_spent >= min_mana_spent {
        continue;
      }

      pop.tick_effects();
      match pop.eval() {
        StateEval::Win => {
          min_mana_spent = min_mana_spent.min(pop.mana_spent);
          continue;
        }
        StateEval::Lose => {
          continue;
        }
        StateEval::Ongoing => {}
      }

      if pop.hero_turn {
        let castable_spells = spellbook.iter().filter(|spell| spell.1 <= pop.hero.mana);
        if castable_spells.clone().count() == 0 {
          continue;
        }

        let can_shield = !pop.effects.iter().any(|e| e.e_type == EffectType::Shield);
        let can_poison = !pop.effects.iter().any(|e| e.e_type == EffectType::Poison);
        let can_recharge = !pop.effects.iter().any(|e| e.e_type == EffectType::Recharge);
        for spell in castable_spells {
          let mut new_state = pop.clone();
          new_state.hero_turn = false;
          new_state.hero.mana -= spell.1;
          new_state.mana_spent += spell.1;

          match spell.0 {
            Spell::MagicMissile => new_state.boss.health -= 4,
            Spell::Drain => {
              new_state.boss.health -= 2;
              new_state.hero.health += 2;
            }
            Spell::Shield if can_shield => new_state.effects.push(Effect {
              duration: 6,
              e_type: EffectType::Shield,
            }),
            Spell::Poison if can_poison => new_state.effects.push(Effect {
              duration: 6,
              e_type: EffectType::Poison,
            }),
            Spell::Recharge if can_recharge => new_state.effects.push(Effect {
              duration: 5,
              e_type: EffectType::Recharge,
            }),
            _ => continue,
          }

          match new_state.eval() {
            StateEval::Win => {
              min_mana_spent = min_mana_spent.min(new_state.mana_spent);
              continue;
            }
            StateEval::Lose => {
              continue;
            }
            StateEval::Ongoing => {
              heap.push(new_state);
            }
          }
        }
      } else {
        pop.hero.health -= (pop.boss.damage - pop.hero.shield).max(1);
        pop.hero_turn = true;
        match pop.eval() {
          StateEval::Win => {
            min_mana_spent = min_mana_spent.min(pop.mana_spent);
            continue;
          }
          StateEval::Lose => {
            continue;
          }
          StateEval::Ongoing => {
            heap.push(pop);
          }
        }
      }
    }

    Some(min_mana_spent.to_string())
  }

  fn solution_extra(&self) -> Option<String> {
    let Some(input) = self.input() else {
      return None;
    };

    let mut boss = Boss {
      health: 0,
      damage: 0,
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
        _ => unreachable!(),
      }
    }

    let mut heap = BinaryHeap::<State>::new();
    let initial_state = State {
      hero_turn: true,
      mana_spent: 0,
      hero: Hero {
        health: 50,
        shield: 0,
        mana: 500,
      },
      boss,
      effects: Vec::new(),
    };
    heap.push(initial_state);

    let spellbook = vec![
      (Spell::MagicMissile, 53),
      (Spell::Drain, 73),
      (Spell::Shield, 113),
      (Spell::Poison, 173),
      (Spell::Recharge, 229),
    ];

    let mut min_mana_spent = i32::MAX;
    while let Some(mut pop) = heap.pop() {
      if pop.mana_spent >= min_mana_spent {
        continue;
      }

      if pop.hero_turn {
        pop.hero.health -= 1;
        match pop.eval() {
          StateEval::Lose => {
            continue;
          }
          _ => {}
        }
      }

      pop.tick_effects();
      match pop.eval() {
        StateEval::Win => {
          min_mana_spent = min_mana_spent.min(pop.mana_spent);
          continue;
        }
        StateEval::Lose => {
          continue;
        }
        StateEval::Ongoing => {}
      }

      if pop.hero_turn {
        let castable_spells = spellbook.iter().filter(|spell| spell.1 <= pop.hero.mana);
        if castable_spells.clone().count() == 0 {
          continue;
        }

        let can_shield = !pop.effects.iter().any(|e| e.e_type == EffectType::Shield);
        let can_poison = !pop.effects.iter().any(|e| e.e_type == EffectType::Poison);
        let can_recharge = !pop.effects.iter().any(|e| e.e_type == EffectType::Recharge);
        for spell in castable_spells {
          let mut new_state = pop.clone();
          new_state.hero_turn = false;
          new_state.hero.mana -= spell.1;
          new_state.mana_spent += spell.1;

          match spell.0 {
            Spell::MagicMissile => new_state.boss.health -= 4,
            Spell::Drain => {
              new_state.boss.health -= 2;
              new_state.hero.health += 2;
            }
            Spell::Shield if can_shield => new_state.effects.push(Effect {
              duration: 6,
              e_type: EffectType::Shield,
            }),
            Spell::Poison if can_poison => new_state.effects.push(Effect {
              duration: 6,
              e_type: EffectType::Poison,
            }),
            Spell::Recharge if can_recharge => new_state.effects.push(Effect {
              duration: 5,
              e_type: EffectType::Recharge,
            }),
            _ => continue,
          }

          match new_state.eval() {
            StateEval::Win => {
              min_mana_spent = min_mana_spent.min(new_state.mana_spent);
              continue;
            }
            StateEval::Lose => {
              continue;
            }
            StateEval::Ongoing => {
              heap.push(new_state);
            }
          }
        }
      } else {
        pop.hero.health -= (pop.boss.damage - pop.hero.shield).max(1);
        pop.hero_turn = true;
        match pop.eval() {
          StateEval::Win => {
            min_mana_spent = min_mana_spent.min(pop.mana_spent);
            continue;
          }
          StateEval::Lose => {
            continue;
          }
          StateEval::Ongoing => {
            heap.push(pop);
          }
        }
      }
    }

    Some(min_mana_spent.to_string())
  }
}

#[derive(Eq, PartialEq, Clone)]
enum Spell {
  MagicMissile,
  Drain,
  Shield,
  Poison,
  Recharge,
}

#[derive(Eq, PartialEq, Clone)]
struct Effect {
  duration: usize,
  e_type: EffectType,
}

#[derive(Eq, PartialEq, Clone)]
enum EffectType {
  Shield,
  Poison,
  Recharge,
}

#[derive(Eq, PartialEq, Clone)]
struct Hero {
  health: i32,
  shield: i32,
  mana: i32,
}

#[derive(Eq, PartialEq, Clone)]
struct Boss {
  health: i32,
  damage: i32,
}

#[derive(Eq, PartialEq, Clone)]
struct State {
  hero_turn: bool,
  mana_spent: i32,
  hero: Hero,
  boss: Boss,
  effects: Vec<Effect>,
}

impl Ord for State {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    other.mana_spent.cmp(&self.mana_spent)
  }
}

impl PartialOrd for State {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
  }
}

impl State {
  fn eval(&self) -> StateEval {
    if self.hero.health <= 0 {
      return StateEval::Lose;
    }

    if self.boss.health <= 0 {
      return StateEval::Win;
    }

    StateEval::Ongoing
  }

  fn tick_effects(&mut self) {
    self.hero.shield = 0;
    for effect in self.effects.iter_mut() {
      match effect.e_type {
        EffectType::Shield => {
          self.hero.shield = 7;
        }
        EffectType::Poison => {
          self.boss.health -= 3;
        }
        EffectType::Recharge => {
          self.hero.mana += 101;
        }
      }
      effect.duration -= 1;
    }

    self.effects.retain(|e| e.duration > 0);
  }
}

enum StateEval {
  Win,
  Lose,
  Ongoing,
}
