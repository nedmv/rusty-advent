
const OFFSET: i64 = 10_000_000_000_000;

fn calc_price_cramer(k: &[i64; 6]) -> Option<i64> {
  let d = k[0] * k[3] - k[1] * k[2];
  if d == 0 {
    return None;
  }
  let d1 = k[4] * k[3] - k[2] * k[5];
  if d1 % d != 0 {
    return None;
  }
  let a = d1 / d;
  if a < 0 {
    return None;
  }
  let d2 = k[0] * k[5] - k[1] * k[4];
  if d2 % d != 0 {
    return None;
  }
  let b = d2 / d;
  if b < 0 {
    return None;
  }
  Some(3 * a + b)
}

fn get_nums(line: &str) -> (i64, i64) {
  let mut l = line.split(':').skip(1).map(|e| e.split(',')).next().unwrap();
  let first = l.next().unwrap();
  let second = l.next().unwrap();
  (first[3..].parse().unwrap(), second[3..].parse().unwrap())
}

fn solve(input: &str, offset: i64) -> i64 {
  let mut ans = 0;
  let mut k = [0; 6];
  for (id, line) in input.lines().enumerate() {
    let mut pos = id % 4;
    if pos != 3 {
      let nums = get_nums(line);
      pos <<= 1;
      k[pos] = nums.0; k[pos + 1] = nums.1;
      if pos == 4 {
        k[4] += offset; k[5] += offset;
        if let Some(price) = calc_price_cramer(&k) {
          ans += price;
        }
      }
    }
  }
  ans
}

#[aoc(day13, part1)]
pub fn part1(input: &str) -> i64 {
  solve(input, 0)
}



#[aoc(day13, part2)]
pub fn part2(input: &str) -> i64 {
  solve(input, OFFSET)
}

#[cfg(test)]
mod tests {
  use super::*;

  const INPUT: &str = include_str!("../input/2024/day13.txt");
  const ANSWER1: &str = include_str!("../answer/2024/day13p1.txt");
  const ANSWER2: &str = include_str!("../answer/2024/day13p2.txt");

  const EXAMPLE: &str = concat!("Button A: X+94, Y+34\n",
                                "Button B: X+22, Y+67\n",
                                "Prize: X=8400, Y=5400\n\n",
                                "Button A: X+26, Y+66\n",
                                "Button B: X+67, Y+21\n",
                                "Prize: X=12748, Y=12176\n\n",
                                "Button A: X+17, Y+86\n",
                                "Button B: X+84, Y+37\n",
                                "Prize: X=7870, Y=6450\n\n",
                                "Button A: X+69, Y+23\n",
                                "Button B: X+27, Y+71\n",
                                "Prize: X=18641, Y=10279");

  #[test]
  fn part1_local() {
    assert_eq!(part1(INPUT).to_string(), ANSWER1);
  }

  #[test]
  fn part2_local() {
    assert_eq!(part2(INPUT).to_string(), ANSWER2);
  }

  #[test]
  fn part1_example() {
    assert_eq!(part1(EXAMPLE), 480);
  }

  #[test]
  fn part2_example() {
    assert_eq!(part2(EXAMPLE), 875318608908);
  }
}