use aoc_trait::Day;

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

    let mut sum = 0;

    let chars = input.chars().collect::<Vec<char>>();
    let mut i = 0;
    while i < input.len() {
      let mut c = chars[i];

      if c.is_digit(10) {
        let negative = i != 0 && chars[i - 1] == '-';
        let mut digits = vec![];

        while c.is_digit(10) {
          digits.push(c.to_digit(10).unwrap() as i32);
          i += 1;
          c = chars[i];
        }

        let mut number = 0i32;
        for (i, digit) in digits.into_iter().rev().enumerate() {
          number += digit * 10_i32.pow(i as u32);
        }

        sum += if negative { -number } else { number };
      }
      i += 1;
    }

    Some(sum.to_string())
  }

  fn solution_extra(&self) -> Option<String> {
    let Some(input) = self.input() else {
      return None;
    };

    None
  }
}