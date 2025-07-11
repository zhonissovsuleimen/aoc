use aoc_trait::{AOC, Day};
mod days;

pub struct AOC15;

impl AOC for AOC15 {
  const YEAR: usize = 2015;

  fn last() -> impl Day {
    days::d17::D17
  }

  fn all() -> Vec<Box<dyn Day>> {
    vec![
      Box::new(days::d1::D1),
      Box::new(days::d2::D2),
      Box::new(days::d3::D3),
      Box::new(days::d4::D4),
      Box::new(days::d5::D5),
      Box::new(days::d6::D6),
      Box::new(days::d7::D7),
      Box::new(days::d8::D8),
      Box::new(days::d9::D9),
      Box::new(days::d10::D10),
      Box::new(days::d11::D11),
      Box::new(days::d12::D12),
      Box::new(days::d13::D13),
      Box::new(days::d14::D14),
      Box::new(days::d15::D15),
      Box::new(days::d16::D16),
      Box::new(days::d17::D17),
    ]
  }
}
