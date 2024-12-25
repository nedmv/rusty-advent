use std::collections::HashMap;
use fasthash::sea::Hash64 as Hash;
type Values<'a> = HashMap::<&'a str, bool, Hash>;
type Gate<'a> = (&'a str, &'a str, &'a str, &'a str);

fn parse(input: &str) -> (Values, Vec<Gate>) {
  let mut values = HashMap::with_hasher(Hash);
  let mut gates = Vec::new();
  let mut parse_gates = false;
  for line in input.trim().lines() {
    if line.len() == 0 {
      parse_gates = true;
      continue;
    }
    if parse_gates {
      let line: Vec<_> = line.split(' ').collect();
      gates.push((line[0], line[1], line[2], line[4]));
    } else {
      let flag = if line.as_bytes()[5] == b'1' {true} else {false};
      values.insert(&line[..3], flag);
    }
  }
  (values, gates)
}

fn from_binary(binary: &Vec<bool>) -> usize {
  let mut ans = 0;
  let mut mul = 1;
  for &byte in binary.iter() {
    if byte {
      ans += mul;
    }
    mul *= 2;
  }
  ans
}

#[aoc(day24, part1)]
pub fn part1(input: &str) -> usize {
  let (mut values, gates) = parse(input);
  let limit = values.len() / 2;
  let mut z = vec![false; limit+1];

  loop {
    let mut changed = false;

    for &(a, op, b, out) in gates.iter() {
      if values.contains_key(&out) {
        continue;
      }
      let val_a = values.get(&a);
      if val_a == None {
        continue;
      }
      let val_a = *val_a.unwrap();
      let val_b = values.get(&b);
      if val_b == None {
        continue;
      }
      let val_b = *val_b.unwrap();
      let val_out = match op {
        "AND" => {
          val_a && val_b
        },
        "OR" => {
          val_a || val_b
        },
        "XOR" => {
          val_a ^ val_b
        },
        _ => {panic!("Unexpected operation {:?}", op)}
      };
      if out.as_bytes()[0] == b'z' {
        let id: usize = out[1..].parse().unwrap();
        z[id] = val_out;
      }
      changed = true;
      values.insert(&out, val_out);
    }
    if !changed {
      break;
    }
  }
  from_binary(&z)
}

#[aoc(day24, part2)]
pub fn part2(input: &str) -> String {
  let (values, gates) = parse(input);
  let limit = values.len() / 2;
  let mut swaps = Vec::with_capacity(8);

  'outer: for &(a, op, b, out) in gates.iter() {
    if out.as_bytes()[0] == b'z' && op != "XOR" && out[1..].parse::<usize>().unwrap() != limit {
      swaps.push(out);
      continue 'outer;
    } 
    match op {
      "XOR" => {
        if (out.as_bytes()[0] < b'x') &&
           (a.as_bytes()[0] < b'x') &&
           (b.as_bytes()[0] < b'x') {
          swaps.push(out);
          continue 'outer;
        }
        for &(inner_a, inner_op, inner_b, _) in gates.iter() {
          if (inner_op == "OR") && (out == inner_a || out == inner_b) {
            swaps.push(out);
            continue 'outer;
          }
        }
      }, 
      "AND" => {
        if a == "x00" || b == "x00" { // half adder
          continue 'outer;
        }
        for &(inner_a, inner_op, inner_b, _) in gates.iter() {
          if (inner_op != "OR") && (out == inner_a || out == inner_b) {
            swaps.push(out);
            continue 'outer;
          }
        }
      },
      _ => {}
    };
  }
  swaps.sort_unstable();
  let mut ans = Vec::new();
  for swap in swaps {
    ans.extend(swap.bytes());
    ans.push(b',');
  }
  ans.pop();
  String::from_utf8(ans).unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;
  use indoc::indoc;

  const INPUT: &str = include_str!("../input/2024/day24.txt");
  const ANSWER1: &str = include_str!("../answer/2024/day24p1.txt");
  const ANSWER2: &str = include_str!("../answer/2024/day24p2.txt");

  const EXAMPLE_1: &str = indoc!("x00: 1
                                  x01: 1
                                  x02: 1
                                  y00: 0
                                  y01: 1
                                  y02: 0

                                  x00 AND y00 -> z00
                                  x01 XOR y01 -> z01
                                  x02 OR y02 -> z02");

  #[test]
  fn part1_local() {
    assert_eq!(part1(INPUT).to_string(), ANSWER1);
  }

  #[test]
  fn part2_local() {
    assert_eq!(part2(INPUT).to_string(), ANSWER2);
  }

  #[test]
  fn part1_example1() {
    assert_eq!(part1(EXAMPLE_1), 4);
  }
}