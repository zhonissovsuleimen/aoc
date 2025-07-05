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
      let banned = line.contains("ab") || line.contains("cd") || line.contains("pq") || line.contains("xy");

      if vowels && two_in_a_row && !banned {
        result += 1;
      }
    }

    Some(result.to_string())
  }

  fn solution_extra(&self) -> Option<String> {
    None
  }
}