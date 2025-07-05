use aoc_trait::Day;

pub struct D6;

/*
--- Day 6: Probably a Fire Hazard ---

Because your neighbors keep defeating you in the holiday house decorating contest year after year, you've decided to deploy one million lights in a 1000x1000 grid.

Furthermore, because you've been especially nice this year, Santa has mailed you instructions on how to display the ideal lighting configuration.

Lights in your grid are numbered from 0 to 999 in each direction; the lights at each corner are at 0,0, 0,999, 999,999, and 999,0. The instructions include whether to turn on, turn off, or toggle various inclusive ranges given as coordinate pairs. Each coordinate pair represents opposite corners of a rectangle, inclusive; a coordinate pair like 0,0 through 2,2 therefore refers to 9 lights in a 3x3 square. The lights all start turned off.

To defeat your neighbors this year, all you have to do is set up your lights by doing the instructions Santa sent you in order.

For example:

    turn on 0,0 through 999,999 would turn on (or leave on) every light.
    toggle 0,0 through 999,0 would toggle the first line of 1000 lights, turning off the ones that were on, and turning on the ones that were off.
    turn off 499,499 through 500,500 would turn off (or leave off) the middle four lights.

After following the instructions, how many lights are lit?
*/

/*
--- Part Two ---

You just finish implementing your winning light pattern when you realize you mistranslated Santa's message from Ancient Nordic Elvish.

The light grid you bought actually has individual brightness controls; each light can have a brightness of zero or more. The lights all start at zero.

The phrase turn on actually means that you should increase the brightness of those lights by 1.

The phrase turn off actually means that you should decrease the brightness of those lights by 1, to a minimum of zero.

The phrase toggle actually means that you should increase the brightness of those lights by 2.

What is the total brightness of all lights combined after following Santa's instructions?

For example:

    turn on 0,0 through 0,0 would increase the total brightness by 1.
    toggle 0,0 through 999,999 would increase the total brightness by 2000000.


*/

impl Day for D6 {
  fn day(&self) -> usize {
    6
  }

  fn input(&self) -> Option<String> {
    let path = format!("{}/src/inputs/d6.txt", env!("CARGO_MANIFEST_DIR"));
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

    let mut grid = [[false; 1000]; 1000];

    for line in input.lines() {
      let instruction = Self::parse(line);

      for i in (instruction.from.0)..=(instruction.to.0) {
        for j in (instruction.from.1)..=(instruction.to.1) {
          grid[i][j] = match instruction.mode {
            Mode::Off => false,
            Mode::On => true,
            Mode::Toggle => !grid[i][j],
          };
        }
      }
    }

    let mut count = 0;
    for i in 0..1000 {
      for j in 0..1000 {
        if grid[i][j] {
          count += 1;
        }
      }
    }

    Some(count.to_string())
  }

  fn solution_extra(&self) -> Option<String> {
    let Some(input) = self.input() else {
      return None;
    };

    let mut grid = vec![vec![0_u32; 1000]; 1000];

    for line in input.lines() {
      let instruction = Self::parse(line);

      for i in (instruction.from.0)..=(instruction.to.0) {
        for j in (instruction.from.1)..=(instruction.to.1) {
          let before = grid[i][j];
          grid[i][j] = match instruction.mode {
            Mode::Off => before.saturating_sub(1),
            Mode::On => before + 1,
            Mode::Toggle => before + 2,
          };
        }
      }
    }

    let mut count = 0;
    for i in 0..1000 {
      for j in 0..1000 {
        count += grid[i][j];
      }
    }

    Some(count.to_string())
  }
}

struct Point(usize, usize);

enum Mode {
  Off,
  On,
  Toggle,
}

struct Instruction {
  mode: Mode,
  from: Point,
  to: Point,
}

//extras
impl D6 {
  fn parse(line: &str) -> Instruction {
    let coords_str: Vec<&str> = line.split(' ').filter(|str| str.contains(',')).collect();

    let from_str = coords_str[0];
    let to_str = coords_str[1];

    let from_values: Vec<usize> = from_str
      .split(',')
      .map(|value_str| value_str.parse::<usize>().unwrap())
      .collect();
    let to_values: Vec<usize> = to_str
      .split(',')
      .map(|value_str| value_str.parse::<usize>().unwrap())
      .collect();

    let from = Point(from_values[0], from_values[1]);
    let to = Point(to_values[0], to_values[1]);

    let mut mode = Mode::Toggle;
    if line.starts_with("turn on") {
      mode = Mode::On;
    } else if line.starts_with("turn off") {
      mode = Mode::Off;
    }

    Instruction { mode, from, to }
  }
}
