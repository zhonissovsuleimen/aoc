use aoc_trait::Day;

pub struct D10;

impl Day for D10 {
  fn day(&self) -> usize {
    10
  }

  fn input(&self) -> Option<String> {
    Some(String::from("1321131112"))
  }

  fn solution(&self) -> Option<String> {
    let Some(input) = self.input() else {
      return None;
    };

    let values = input.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>();
    let len = look_and_say(0, 40, values).len();

    Some(len.to_string())
  }

  fn solution_extra(&self) -> Option<String> {
    let Some(input) = self.input() else {
      return None;
    };

    let values = input.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>();
    let len = look_and_say(0, 50, values).len();

    Some(len.to_string())
  }
}

fn look_and_say(current_i: usize, max_i: usize, values: Vec<u32>) -> Vec<u32> {
  if current_i == max_i || values.is_empty() {
    return values;
  }
  let mut digits = values.clone();
  digits.dedup();

  let mut counts = vec![];

  let (mut last_num, mut count) = (values[0], 0);
  for num in values {
    if num == last_num {
      count += 1;
    } else {
      counts.push(count);
      count = 1;
    }
    last_num = num;
  }
  counts.push(count);

  let mut new_values = Vec::with_capacity(counts.len() * 2);
  for (&count, &digit) in counts.iter().zip(digits.iter()) {
    new_values.push(count);
    new_values.push(digit);
  }

  look_and_say(current_i + 1, max_i, new_values)
}