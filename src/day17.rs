use std::mem::swap;

#[allow(non_snake_case)]
#[derive(Debug, Default)]
struct Register {
  A: u64,
  B: u64,
  C: u64,
}

fn get_combo_operand(op: u8, registers: &Register) -> u64 {
  let val;
  match op {
    0..=3 => {val = op as u64},
    4 => {val = registers.A},
    5 => {val = registers.B},
    6 => {val = registers.C},
    _ => {panic!("Invalid combo operand {}!", op)}
  }
  val
}

fn get_new_val(op: u8, registers: &Register) -> u64 {
  registers.A / 2_u64.pow(get_combo_operand(op, registers) as u32)
}

fn run_once(program: &Vec<u8>, registers: &mut Register) -> u8 {
  let mut printed_num = 0;
  let mut op;
  for i in (0..program.len()).step_by(2) {
    op = program[i+1];
    match program[i] {
      0 => {
        registers.A = get_new_val(op, registers);
      },
      1 => {
        registers.B ^= op as u64;
      },
      2 => {
        registers.B = get_combo_operand(op, registers) % 8;
      },
      3 => {}, // using input property to ignore proper jump implementation
      4 => {
        registers.B ^= registers.C;
      },
      5 => {
        printed_num = get_combo_operand(op, registers) % 8;
      }
      6 => {
        registers.B = get_new_val(op, registers);
      },
      7 => {
        registers.C = get_new_val(op, registers);
      },
      _ => {panic!("Invalid operation {}!", program[i])}
    }
  }

  printed_num as u8
}

fn parse_input(input: &str) -> (Vec<u8>, u64) {
  let lines: Vec<_> = input.lines().collect();
  (lines[4].bytes().skip(9).step_by(2).map(|b| b - b'0').collect(), 
   lines[0].split(": ").nth(1).unwrap().parse().unwrap()) // assume that B=C=0
}

#[aoc(day17, part1)]
pub fn part1(input: &str) -> String {
  let (program, a) = parse_input(input);
  let mut registers = Register{A: a, B: 0, C: 0};
  let mut output = Vec::new();
  loop {
    output.push(b'0' + run_once(&program, &mut registers));
    if registers.A == 0 {
      break;
    }
    output.push(b',');
  }
  String::from_utf8(output).unwrap()
}


#[aoc(day17, part2)]
pub fn part2(input: &str) -> u64 {
  let (program, _) = parse_input(input);
  let mut registers = Register::default();
  let mut candidates = Vec::new();
  candidates.push(0);
  let mut next = Vec::new();
  let mut cur;
  for &target in program.iter().rev() {
    for cand in &candidates {
      for i in 0..8 {
        cur = cand*8 + i;
        registers.A = cur;
        if run_once(&program, &mut registers) == target {
          next.push(cur);
        }
      }
    }
    swap(&mut candidates, &mut next);
    next.clear();
  }
  candidates[0] // candidates are always sorted
}

#[cfg(test)]
mod tests {
  use super::*;
  use indoc::indoc;

  const INPUT: &str = include_str!("../input/2024/day17.txt");
  const ANSWER1: &str = include_str!("../answer/2024/day17p1.txt");
  const ANSWER2: &str = include_str!("../answer/2024/day17p2.txt");

  const EXAMPLE_1: &str = indoc!("
                      Register A: 729
                      Register B: 0
                      Register C: 0

                      Program: 0,1,5,4,3,0");

  const EXAMPLE_2: &str = indoc!("
                      Register A: 2024
                      Register B: 0
                      Register C: 0

                      Program: 0,3,5,4,3,0");

  //Source: https://www.reddit.com/r/adventofcode/comments/1hggduo/2024_day_17_part_2_a_challenging_test_case/
  const CHALLENGING_INPUT: &str = indoc!("
                      Register A: 12345678
                      Register B: 0
                      Register C: 0

                      Program: 2,4,1,0,7,5,1,5,0,3,4,5,5,5,3,0");

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
    assert_eq!(part1(EXAMPLE_1), "4,6,3,5,6,3,5,2,1,0");
  }

  #[test]
  fn part2_example2() {
    assert_eq!(part2(EXAMPLE_2), 117440);
  }

  #[test]
  fn part1_challenging() {
    assert_eq!(part1(CHALLENGING_INPUT), "6,0,4,5,4,5,2,0");
  }

  #[test]
  fn part2_challenging() {
    assert_eq!(part2(CHALLENGING_INPUT), 202797954918051);
  }

}