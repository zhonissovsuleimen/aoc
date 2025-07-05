use std::collections::HashMap;

use aoc_trait::Day;

pub struct D5;

/*
--- Day 5: Doesn't He Have Intern-Elves For This? ---

Santa needs help figuring out which strings in his text file are naughty or nice.

A nice string is one with all of the following properties:

    It contains at least three vowels (aeiou only), like aei, xazegov, or aeiouaeiouaeiou.
    It contains at least one letter that appears twice in a row, like xx, abcdde (dd), or aabbccdd (aa, bb, cc, or dd).
    It does not contain the strings ab, cd, pq, or xy, even if they are part of one of the other requirements.

For example:

    ugknbfddgicrmopn is nice because it has at least three vowels (u...i...o...), a double letter (...dd...), and none of the disallowed substrings.
    aaa is nice because it has at least three vowels and a double letter, even though the letters used by different rules overlap.
    jchzalrnumimnmhp is naughty because it has no double letter.
    haegwjzuvuyypxyu is naughty because it contains the string xy.
    dvszwmarrgswjxmb is naughty because it contains only one vowel.

How many strings are nice?
*/

/*
--- Part Two ---

Realizing the error of his ways, Santa has switched to a better model of determining whether a string is naughty or nice. None of the old rules apply, as they are all clearly ridiculous.

Now, a nice string is one with all of the following properties:

    It contains a pair of any two letters that appears at least twice in the string without overlapping, like xyxy (xy) or aabcdefgaa (aa), but not like aaa (aa, but it overlaps).
    It contains at least one letter which repeats with exactly one letter between them, like xyx, abcdefeghi (efe), or even aaa.

For example:

    qjhvhtzxzqqjkmpb is nice because is has a pair that appears twice (qj) and a letter that repeats with exactly one letter between them (zxz).
    xxyxx is nice because it has a pair that appears twice and a letter that repeats with one between, even though the letters used by each rule overlap.
    uurcxstgmygtbstg is naughty because it has a pair (tg) but no repeat with a single letter between them.
    ieodomkazucvgmuy is naughty because it has a repeating letter with one between (odo), but no pair that appears twice.

How many strings are nice under these new rules?
*/

impl Day for D5 {
  fn day(&self) -> usize {
    5
  }

  fn input(&self) -> Option<String> {
    let path = format!("{}/src/inputs/d5.txt", env!("CARGO_MANIFEST_DIR"));
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

    let mut result = 0;

    for line in input.lines() {
      let chars = line.chars().collect::<Vec<char>>();

      let vowels = chars.iter().filter(|&&c| "aeiou".contains(c)).count() >= 3;
      let two_in_a_row = chars.windows(2).any(|w| w[0] == w[1]);
      let banned =
        line.contains("ab") || line.contains("cd") || line.contains("pq") || line.contains("xy");

      if vowels && two_in_a_row && !banned {
        result += 1;
      }
    }

    Some(result.to_string())
  }

  fn solution_extra(&self) -> Option<String> {
    let Some(input) = self.input() else {
      return None;
    };
    let mut result = 0;

    for line in input.lines() {
      let chars = line.chars().collect::<Vec<char>>();
      let mut hmap = HashMap::<&[char], usize>::new();

      let two_pairs = chars.windows(2).enumerate().any(|(this_i, this_pair)| {
        match hmap.get(this_pair) {
          Some(that_i) => {
            return this_i > that_i + 1;
          },
          None => {
            hmap.insert(this_pair, this_i);
            return false;
          },
        }
      });

      let repeat = chars.windows(3).any(|w| w.len() == 3 && w[0] == w[2]);

      if two_pairs && repeat {
        result += 1;
      }
    }

    Some(result.to_string())
  }
}
