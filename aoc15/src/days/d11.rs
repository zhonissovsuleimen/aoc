use aoc_trait::Day;

pub struct D11;


/*
--- Day 11: Corporate Policy ---

Santa's previous password expired, and he needs help choosing a new one.

To help him remember his new password after the old one expires, Santa has devised a method of coming up with a password based on the previous one. Corporate policy dictates that passwords must be exactly eight lowercase letters (for security reasons), so he finds his new password by incrementing his old password string repeatedly until it is valid.

Incrementing is just like counting with numbers: xx, xy, xz, ya, yb, and so on. Increase the rightmost letter one step; if it was z, it wraps around to a, and repeat with the next letter to the left until one doesn't wrap around.

Unfortunately for Santa, a new Security-Elf recently started, and he has imposed some additional password requirements:

    Passwords must include one increasing straight of at least three letters, like abc, bcd, cde, and so on, up to xyz. They cannot skip letters; abd doesn't count.
    Passwords may not contain the letters i, o, or l, as these letters can be mistaken for other characters and are therefore confusing.
    Passwords must contain at least two different, non-overlapping pairs of letters, like aa, bb, or zz.

For example:

    hijklmmn meets the first requirement (because it contains the straight hij) but fails the second requirement requirement (because it contains i and l).
    abbceffg meets the third requirement (because it repeats bb and ff) but fails the first requirement.
    abbcegjk fails the third requirement, because it only has one double letter (bb).
    The next password after abcdefgh is abcdffaa.
    The next password after ghijklmn is ghjaabcc, because you eventually skip all the passwords that start with ghi..., since i is not allowed.

Given Santa's current password (your puzzle input), what should his next password be?
*/

/*
--- Part Two ---

Santa's password expired again. What's the next one?
*/

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

    let mut password = input;

    while !is_legal(&password) {
      increment(&mut password);
    }

    increment(&mut password);

    while !is_legal(&password) {
      increment(&mut password);
    }

    Some(password)
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