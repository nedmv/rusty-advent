use regex::Regex;

#[aoc(day3, part1)]
pub fn part1(input: &str) -> u32 {
  let muls = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
  let mut ans = 0;
  for (_, [first, second]) in muls.captures_iter(input).map(|c| c.extract()) {
    ans += first.parse::<u32>().unwrap() * second.parse::<u32>().unwrap();
  }
  ans
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> u32 {
  let muls = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
  let mut ans = 0;
  let mut enabled = true;
  for captures in muls.captures_iter(input) {
    match captures.get(0).unwrap().as_str() {
      "do()" => enabled = true,
      "don't()" => enabled = false,
      _ => {
        if enabled {
          let first = captures.get(1).unwrap().as_str().parse::<u32>().unwrap();
          let second = captures.get(2).unwrap().as_str().parse::<u32>().unwrap();
          ans += first * second;
        }
      }
    }
  }
  ans
}