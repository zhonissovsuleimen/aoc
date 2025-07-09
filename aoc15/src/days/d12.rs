use aoc_trait::Day;
use serde_json::Value;

pub struct D12;

/*
--- Day 12: JSAbacusFramework.io ---

Santa's Accounting-Elves need help balancing the books after a recent order. Unfortunately, their accounting software uses a peculiar storage format. That's where you come in.

They have a JSON document which contains a variety of things: arrays ([1,2,3]), objects ({"a":1, "b":2}), numbers, and strings. Your first job is to simply find all of the numbers throughout the document and add them together.

For example:

    [1,2,3] and {"a":2,"b":4} both have a sum of 6.
    [[[3]]] and {"a":{"b":4},"c":-1} both have a sum of 3.
    {"a":[-1,1]} and [-1,{"a":1}] both have a sum of 0.
    [] and {} both have a sum of 0.

You will not encounter any strings containing numbers.

What is the sum of all numbers in the document?
*/

impl Day for D12 {
  fn day(&self) -> usize {
    12
  }

  fn input(&self) -> Option<String> {
    let path = format!("{}/src/inputs/d12.txt", env!("CARGO_MANIFEST_DIR"));
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

    let v: Value = serde_json::from_str(&input).unwrap();

    Some(count_nums(v).to_string())
  }

  fn solution_extra(&self) -> Option<String> {
    let Some(input) = self.input() else {
      return None;
    };

    None
  }
}

fn count_nums(json: Value) -> i32 {
  match json {
    Value::Null => 0,
    Value::Bool(_) => 0,
    Value::Number(number) => number.as_i64().unwrap_or(0) as i32,
    Value::String(_) => 0,
    Value::Array(values) => {
      let mut sum = 0;
      for child_value in values {
        sum += count_nums(child_value);
      }

      sum
    }
    Value::Object(map) => {
      let mut sum = 0;
      for child_value in map.into_values() {
        sum += count_nums(child_value);
      }
      sum
    }
  }
}
