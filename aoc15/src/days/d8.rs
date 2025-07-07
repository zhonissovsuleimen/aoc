use aoc_trait::Day;

pub struct D8;

impl Day for D8 {
  fn day(&self) -> usize {
    8
  }

  fn input(&self) -> Option<String> {
    let path = format!("{}/src/inputs/d8.txt", env!("CARGO_MANIFEST_DIR"));
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
      let code_len = line.len();
      let mut mem_len = code_len - 2;

      let chars = line.chars().collect::<Vec<char>>();
      let mut i = 0;
      while i < code_len - 1 {
        match (chars[i], chars[i + 1]) {
          ('\\', '\\') => {
            mem_len -= 1;
            i += 2;
          }
          ('\\', '"') => {
            mem_len -= 1;
            i += 2;
          }
          ('\\', 'x') => {
            mem_len -= 3;
            i += 4;
          }
          _ => {
            i += 1;
          }
        }
      }

      result += code_len - mem_len;
      println!("{line}");
      println!("{}-{}", code_len, mem_len);
    }

    Some(result.to_string())
  }

  fn solution_extra(&self) -> Option<String> {
    let Some(input) = self.input() else {
      return None;
    };

    None
  }
}
