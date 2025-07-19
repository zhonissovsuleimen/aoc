use std::collections::BinaryHeap;

use aoc_trait::Day;

pub struct D22;

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

    None
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
