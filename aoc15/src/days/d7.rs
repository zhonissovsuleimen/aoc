use std::collections::HashMap;

use aoc_trait::Day;

pub struct D7;

/*
--- Day 7: Some Assembly Required ---

This year, Santa brought little Bobby Tables a set of wires and bitwise logic gates! Unfortunately, little Bobby is a little under the recommended age range, and he needs help assembling the circuit.

Each wire has an identifier (some lowercase letters) and can carry a 16-bit signal (a number from 0 to 65535). A signal is provided to each wire by a gate, another wire, or some specific value. Each wire can only get a signal from one source, but can provide its signal to multiple destinations. A gate provides no signal until all of its inputs have a signal.

The included instructions booklet describes how to connect the parts together: x AND y -> z means to connect wires x and y to an AND gate, and then connect its output to wire z.

For example:

    123 -> x means that the signal 123 is provided to wire x.
    x AND y -> z means that the bitwise AND of wire x and wire y is provided to wire z.
    p LSHIFT 2 -> q means that the value from wire p is left-shifted by 2 and then provided to wire q.
    NOT e -> f means that the bitwise complement of the value from wire e is provided to wire f.

Other possible gates include OR (bitwise OR) and RSHIFT (right-shift). If, for some reason, you'd like to emulate the circuit instead, almost all programming languages (for example, C, JavaScript, or Python) provide operators for these gates.

For example, here is a simple circuit:

123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i

After it is run, these are the signals on the wires:

d: 72
e: 507
f: 492
g: 114
h: 65412
i: 65079
x: 123
y: 456

In little Bobby's kit's instructions booklet (provided as your puzzle input), what signal is ultimately provided to wire a?
*/

/*
--- Part Two ---

Now, take the signal you got on wire a, override wire b to that signal, and reset the other wires (including wire a). What new signal is ultimately provided to wire a?
*/

impl Day for D7 {
  fn day(&self) -> usize {
    7
  }

  fn input(&self) -> Option<String> {
    let path = format!("{}/src/inputs/d7.txt", env!("CARGO_MANIFEST_DIR"));
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

    let mut expressions = HashMap::<String, Expression>::new();
    let mut values = HashMap::<String, u16>::new();

    for line in input.lines() {
      let (key, value) = parse_line(line);
      expressions.insert(key, value);
    }

    let result = solve(&expressions["a"], &expressions, &mut values);

    Some(result.to_string())
  }

  fn solution_extra(&self) -> Option<String> {
    let Some(input) = self.input() else {
      return None;
    };

    let mut expressions = HashMap::<String, Expression>::new();
    let mut values = HashMap::<String, u16>::new();

    for line in input.lines() {
      let (key, value) = parse_line(line);
      expressions.insert(key, value);
    }

    let initial_a_result = solve(&expressions["a"], &expressions, &mut values);

    expressions.insert(String::from("b"), Expression::Value(initial_a_result));
    values.clear();

    let result = solve(&expressions["a"], &expressions, &mut values);
    Some(result.to_string())
  }
}

enum Expression {
  Value(u16),
  Wire(String),
  And(Box<Expression>, Box<Expression>),
  Or(Box<Expression>, Box<Expression>),
  LShift(Box<Expression>, u16),
  RShift(Box<Expression>, u16),
  Not(Box<Expression>),
}

fn parse_expression(token: &str) -> Expression {
  if let Ok(val) = token.parse::<u16>() {
    Expression::Value(val)
  } else {
    Expression::Wire(token.to_string())
  }
}

fn parse_line(line: &str) -> (String, Expression) {
  let split: Vec<&str> = line.split(" -> ").collect();
  let expr = split[0];
  let id = split[1].to_string();

  let tokens: Vec<&str> = expr.split_whitespace().collect();
  let expression = match tokens.as_slice() {
    [a] => parse_expression(a),
    ["NOT", a] => Expression::Not(Box::new(parse_expression(a))),
    [a, "AND", b] => Expression::And(Box::new(parse_expression(a)), Box::new(parse_expression(b))),
    [a, "OR", b] => Expression::Or(Box::new(parse_expression(a)), Box::new(parse_expression(b))),
    [a, "LSHIFT", b] => Expression::LShift(Box::new(parse_expression(a)), b.parse().unwrap()),
    [a, "RSHIFT", b] => Expression::RShift(Box::new(parse_expression(a)), b.parse().unwrap()),
    _ => unreachable!(),
  };

  (id, expression)
}

fn solve(
  expr: &Expression,
  map: &HashMap<String, Expression>,
  cache: &mut HashMap<String, u16>,
) -> u16 {
  match expr {
    Expression::Value(a) => *a,
    Expression::Wire(id) => {
      if let Some(&val) = cache.get(id) {
        val
      } else {
        let val = solve(&map[id], map, cache);
        cache.insert(id.clone(), val);
        val
      }
    },
    Expression::And(a, b) => solve(a, map, cache) & solve(b, map, cache),
    Expression::Or(a, b) => solve(a, map, cache) | solve(b, map, cache),
    Expression::LShift(a, n) => solve(a, map, cache) << n,
    Expression::RShift(a, n) => solve(a, map, cache) >> n,
    Expression::Not(a) => !solve(a, map, cache),
  }
}