use std::collections::HashMap;
use fasthash::city::Hash128 as Hash;

fn parse(input: &str) -> (Vec<&str>, Vec<&str>) {
  let mut lines = input.lines();
  let patterns = lines.next().unwrap().split(", ").collect();
  let designs = lines.skip(1).collect();
  (patterns, designs)
}

fn rec<'a>(design: &'a str, patterns: &Vec<&str>, dp: &mut HashMap<&'a str, usize, Hash>) -> usize {
  if design.len() == 0 { // performing this check via hash map is significantly slower
    return 1;
  }
  if let Some(val) = dp.get(design) {
    return *val;
  }
  let mut ans = 0;
  for &pat in patterns.iter() {
    if design.starts_with(pat) {
      ans += rec(&design[pat.len()..], patterns, dp);
    }
  }
  dp.insert(design, ans);
  ans
}

#[aoc(day19, part1)]
pub fn part1(input: &str) -> u32 {
  let (patterns, designs) = parse(input);
  let mut dp = HashMap::with_hasher(Hash);
  let mut ans = 0;
  for d in designs {
    if rec(d, &patterns, &mut dp) > 0 {
      ans += 1;
    }
  }
  ans
}

#[aoc(day19, part2)]
pub fn part2(input: &str) -> usize {
  let (patterns, designs) = parse(input);
  let mut dp = HashMap::with_hasher(Hash);
  let mut ans = 0;
  for d in designs {
    ans += rec(d, &patterns, &mut dp);
  }
  ans
}

#[cfg(test)]
mod tests {
  use super::*;
  use indoc::indoc;

  const INPUT: &str = include_str!("../input/2024/day19.txt");
  const ANSWER1: &str = include_str!("../answer/2024/day19p1.txt");
  const ANSWER2: &str = include_str!("../answer/2024/day19p2.txt");

  const EXAMPLE: &str = indoc!("r, wr, b, g, bwu, rb, gb, br

                                brwrr
                                bggr
                                gbbr
                                rrbgbr
                                ubwu
                                bwurrg
                                brgr
                                bbrgwb");


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
    assert_eq!(part1(EXAMPLE), 6);
  }

  #[test]
  fn part2_example() {
    assert_eq!(part2(EXAMPLE), 16);
  }
}