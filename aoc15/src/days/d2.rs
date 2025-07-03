use aoc_trait::Day;

pub struct D2;

/*
--- Day 2: I Was Told There Would Be No Math ---

The elves are running low on wrapping paper, and so they need to submit an order for more. They have a list of the dimensions (length l, width w, and height h) of each present, and only want to order exactly as much as they need.

Fortunately, every present is a box (a perfect right rectangular prism), which makes calculating the required wrapping paper for each gift a little easier: find the surface area of the box, which is 2*l*w + 2*w*h + 2*h*l. The elves also need a little extra paper for each present: the area of the smallest side.

For example:

    A present with dimensions 2x3x4 requires 2*6 + 2*12 + 2*8 = 52 square feet of wrapping paper plus 6 square feet of slack, for a total of 58 square feet.
    A present with dimensions 1x1x10 requires 2*1 + 2*10 + 2*10 = 42 square feet of wrapping paper plus 1 square foot of slack, for a total of 43 square feet.

All numbers in the elves' list are in feet. How many total square feet of wrapping paper should they order?
*/

impl Day for D2 {
  fn day(&self) -> usize {
    2
  }

  fn input(&self) -> Option<String> {
    let path = format!("{}/src/inputs/d2.txt", env!("CARGO_MANIFEST_DIR"));
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

    let mut total = 0;

    for line in input.lines() {
      let (x, y, z) = Self::parse(line);
      let xy = x * y;
      let xz = x * z;
      let yz = y * z;

      let min = xy.min(xz).min(yz);
      total += 2 * (xy + xz + yz) + min;
    }

    Some(total.to_string())
  }

  fn solution_extra(&self) -> Option<String> {
    None
  }
}

//extras
impl D2 {
  fn parse(input: &str) -> (u32, u32, u32) {
    let split = input
      .split("x")
      .map(|value| value.parse::<u32>().unwrap_or(0))
      .collect::<Vec<u32>>();
    (split[0], split[1], split[2])
  }
}
