const MOD: usize = 16777216;
const MAX_ITERATIONS: usize = 2000;
const MAX_PATTERNS: usize = 19*19*19*19;
const 

fn calc(mut num: usize) -> usize {
  num ^= num * 64;  
  num %= MOD;
  num ^= num / 32;
  num %= MOD;
  num ^= num * 2048;
  num %= MOD;
  num
}


#[aoc(day22, part1)]
pub fn part1(input: &str) -> usize {
  let mut ans = 0;
  for mut num in input.lines().map(|s| s.parse().unwrap()) {
    for _ in 0..MAX_ITERATIONS {
      num = calc(num);
    }
    ans += num;
  }
  ans
}

#[aoc(day22, part2)]
pub fn part2(input: &str) -> usize {
  let mut cur_cost;
  let mut prev_cost;
  let mut pattern = [0; 4];
  let mut data = [[[[0; 19]; 19]; 19]; 19];
  let mut seen = [[[[false; 19]; 19]; 19]; 19];
  for mut num in input.lines().map(|s| s.parse().unwrap()) {
    cur_cost = num % 10;
    for i in 1..4 {
      prev_cost = cur_cost;
      num = calc(num);
      cur_cost = num % 10;
      pattern[i] = 9 + cur_cost - prev_cost;
    }
    for _ in 3..MAX_ITERATIONS {
      for i in 0..3 {
        pattern[i] = pattern[i+1];
      }
      prev_cost = cur_cost;
      num = calc(num);
      cur_cost = num % 10;
      pattern[3] = 9 + cur_cost - prev_cost;
      if !seen[pattern[0]][pattern[1]][pattern[2]][pattern[3]] {
        seen[pattern[0]][pattern[1]][pattern[2]][pattern[3]] = true;
        data[pattern[0]][pattern[1]][pattern[2]][pattern[3]] += cur_cost as u16;
      }
    }

    for i in 0..19 {
      for j in 0..19 {
        for k in 0..19 {
          for l in 0..19 {
            seen[i][j][k][l] = false;
          }
        }
      }
    }
  }

  let mut ans = 0;
  for i in 0..19 {
    for j in 0..19 {
      for k in 0..19 {
        for l in 0..19 {
          ans = ans.max(data[i][j][k][l]);
        }
      }
    }
  }
  ans as usize
}

#[cfg(test)]
mod tests {
  use super::*;
  use indoc::indoc;

  const INPUT: &str = include_str!("../input/2024/day22.txt");
  const ANSWER1: &str = include_str!("../answer/2024/day22p1.txt");
  const ANSWER2: &str = include_str!("../answer/2024/day22p2.txt");

  const EXAMPLE_1: &str = indoc!("1
                                  10
                                  100
                                  2024");

  const EXAMPLE_2: &str = indoc!("1
                                  2
                                  3
                                  2024");

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
    assert_eq!(part1(EXAMPLE_1), 37327623);
  }

  #[test]
  fn part2_example2() {
    assert_eq!(part2(EXAMPLE_2), 23);
  }
}