use aoc_trait::Day;

pub struct D11;

impl Day for D11 {
  fn day(&self) -> usize {
    11
  }

  fn input(&self) -> Option<String> {
    Some(String::from("cqjxjnds"))
  }

  fn solution(&self) -> Option<String> {
    let Some(input) = self.input() else {
      return None;
    };

    let mut password = input;

    while !is_legal(&password) {
      increment(&mut password);
    }

    Some(password)
  }

  fn solution_extra(&self) -> Option<String> {
    let Some(input) = self.input() else {
      return None;
    };

    None
  }
}

fn increment(password: &mut String) {
  let len = password.len();
  
  let old_chars = password.chars().map(|c| c as u8).collect::<Vec<u8>>();
  let mut new_chars = old_chars.clone();

  let mut increment = true;
  for (i, c) in old_chars.into_iter().rev().enumerate() {
    if increment {
      if c == b'z' {
        increment = true;
        new_chars[len - i - 1] = b'a';
      } else {
        increment = false;
        new_chars[len - i - 1] = c + 1;
      }
    }
  }

  *password = String::from_utf8(new_chars).unwrap();
}

fn is_legal(password: &str) -> bool {
  let three_in_a_row = password.as_bytes().windows(3).any(|window| window[0] == window[1] - 1 && window[0] == window[2] - 2);

  let mut no_banned_chars = true;
  let banned = ['i', 'o', 'l'];
  for i in banned {
    if password.contains(i) {
      no_banned_chars = false;
      break;
    }
  }

  let mut count = 0;
  let mut first_pair_char = b'$';
  let mut i = 0usize;
  let bytes = password.as_bytes();
  while i < password.len() - 1 {
    if bytes[i] == bytes[i + 1] && bytes[i] != first_pair_char {
      first_pair_char = bytes[i];
      count += 1;
      i += 1;
    }
    i += 1;
  }
  let two_unique_pairs = count >= 2;

  three_in_a_row && no_banned_chars && two_unique_pairs
}