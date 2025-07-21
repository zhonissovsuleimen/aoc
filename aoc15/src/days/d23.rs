use aoc_trait::Day;

pub struct D23;

/*
--- Day 23: Opening the Turing Lock ---

Little Jane Marie just got her very first computer for Christmas from some unknown benefactor. It comes with instructions and an example program, but the computer itself seems to be malfunctioning. She's curious what the program does, and would like you to help her run it.

The manual explains that the computer supports two registers and six instructions (truly, it goes on to remind the reader, a state-of-the-art technology). The registers are named a and b, can hold any non-negative integer, and begin with a value of 0. The instructions are as follows:

    hlf r sets register r to half its current value, then continues with the next instruction.
    tpl r sets register r to triple its current value, then continues with the next instruction.
    inc r increments register r, adding 1 to it, then continues with the next instruction.
    jmp offset is a jump; it continues with the instruction offset away relative to itself.
    jie r, offset is like jmp, but only jumps if register r is even ("jump if even").
    jio r, offset is like jmp, but only jumps if register r is 1 ("jump if one", not odd).

All three jump instructions work with an offset relative to that instruction. The offset is always written with a prefix + or - to indicate the direction of the jump (forward or backward, respectively). For example, jmp +1 would simply continue with the next instruction, while jmp +0 would continuously jump back to itself forever.

The program exits when it tries to run an instruction beyond the ones defined.

For example, this program sets a to 2, because the jio instruction causes it to skip the tpl instruction:

inc a
jio a, +2
tpl a
inc a

What is the value in register b when the program in your puzzle input is finished executing?

*/
impl Day for D23 {
  fn day(&self) -> usize {
    23
  }

  fn input(&self) -> Option<String> {
    let path = format!("{}/src/inputs/d23.txt", env!("CARGO_MANIFEST_DIR"));
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

    let (mut a, mut b) = (0, 0);
    let lines = input.lines().collect::<Vec<&str>>();

    let mut i = 0i32;
    while i >= 0 && i < lines.len() as i32 {
      let tokens = lines[i as usize].split(' ').collect::<Vec<&str>>();
      println!("{:?}", tokens.as_slice());
      match tokens.as_slice() {
        ["hlf", reg] => {
          if *reg == "a" {
            a /= 2;
          } else {
            b /= 2;
          }
        }
        ["tpl", reg] => {
          if *reg == "a" {
            a *= 3;
          } else {
            b *= 3;
          }
        }
        ["inc", reg] => {
          if *reg == "a" {
            a += 1;
          } else {
            b += 1;
          }
        }
        ["jmp", offset] => {
          i += offset.parse::<i32>().unwrap();
          continue;
        }
        ["jie", reg, offset] => {
          if (*reg == "a," && a % 2 == 0) || (*reg == "b," && b % 2 == 0) {
            i += offset.parse::<i32>().unwrap();
            continue;
          }
        }
        ["jio", reg, offset] => {
          println!("beep");
          if (*reg == "a," && a == 1) || (*reg == "b," && b == 1) {
            i += offset.parse::<i32>().unwrap();
            continue;
          }
        }
        _ => unreachable!(),
      }

      i += 1;
    }

    Some(b.to_string())
  }

  fn solution_extra(&self) -> Option<String> {
    let Some(input) = self.input() else {
      return None;
    };

    None
  }
}
