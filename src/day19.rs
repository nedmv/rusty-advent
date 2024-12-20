use std::collections::HashMap;
use fasthash::city::Hash128 as Hash;


#[derive(PartialEq)]
struct Trie {
  children: [Option<Box<Trie>>;5],
  exists: bool
}

impl Trie {
  fn new() -> Self {
    Trie {children: [const {None}; 5], exists: false}
  }
  fn add(&mut self, s: &str) {
    if s.len() == 0 {
      self.exists = true;
      return;
    }
    let id = Self::get_id(s.bytes().next().unwrap());
    if self.children[id] == None {
      self.children[id] = Some(Box::<Trie>::new(Trie::new()));
    }
    self.children[id].as_mut().unwrap().add(&s[1..]);
  }

  fn get_offsets(&self, s: &str, depth: usize, offsets: &mut Vec<usize>) {
    if self.exists {
      offsets.push(depth);
    }
    if s.len() == 0 {
      return;
    }
    let id = Self::get_id(s.bytes().next().unwrap());
    if self.children[id] == None {
      return;
    }
    self.children[id].as_ref().unwrap().get_offsets(&s[1..], depth+1, offsets);
  }

  fn get_id(ch: u8) -> usize {
    match ch {
      b'w' => 0,
      b'u' => 1,
      b'b' => 2,
      b'r' => 3,
      b'g' => 4,
      _ => {panic!("Unexpected character {}", ch)}
    }
  }
}

fn parse(input: &str) -> (Vec<&str>, Vec<&str>) {
  let mut lines = input.lines();
  let patterns = lines.next().unwrap().split(", ").collect();
  let designs = lines.skip(1).collect();
  (patterns, designs)
}

fn rec<'a>(design: &'a str, patterns: &Trie, dp: &mut HashMap<&'a str, usize, Hash>) -> usize {
  if design.len() == 0 { // performing this check via hash map is significantly slower
    return 1;
  }
  if let Some(val) = dp.get(design) {
    return *val;
  }
  let mut ans = 0;
  let mut offsets = Vec::new();
  patterns.get_offsets(design, 0, &mut offsets);
  
  for offset in offsets {
    ans += rec(&design[offset..], patterns, dp);
  }
  dp.insert(design, ans);
  ans
}

#[aoc(day19, part1)]
pub fn part1(input: &str) -> u32 {
  let (patterns, designs) = parse(input);
  let mut trie = Trie::new();
  for p in patterns {
    trie.add(p);
  }
  let mut dp = HashMap::with_hasher(Hash);
  let mut ans = 0;
  for d in designs {
    if rec(d, &trie, &mut dp) > 0 {
      ans += 1;
    }
  }
  ans
}

#[aoc(day19, part2)]
pub fn part2(input: &str) -> usize {
  let (patterns, designs) = parse(input);
  let mut trie = Trie::new();
  for p in patterns {
    trie.add(p);
  }
  let mut dp = HashMap::with_hasher(Hash);
  let mut ans = 0;
  for d in designs {
    ans += rec(d, &trie, &mut dp);
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