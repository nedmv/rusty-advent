use counter::Counter;
use std::fmt::Display;
type Pair = (i32, i32);

// #[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<Pair> {
    input
        .lines()
        .map(|l| {
            let mut pair = l.trim().split("   ").map(|d| d.parse().unwrap());
            (
                pair.next().unwrap(),
                pair.next().unwrap(),
            )
        }).collect()
}


// #[aoc(day1, part1)]
pub fn part1(input: &str) -> impl Display {
  let input = input_generator(input);
  let mut l1 = input.iter().map(|p| p.0 ).collect::<Vec<i32>>();
  let mut l2 = input.iter().map(|p| p.1 ).collect::<Vec<i32>>();
  
  l1.sort_unstable();
  l2.sort_unstable();

  let mut ans = 0;
  for i in 0..l1.len() {
    ans += (l1[i] - l2[i]).abs();
  }
  ans
}

// #[aoc(day1, part2)]
pub fn part2(input: &str) -> impl Display {
  let input = input_generator(input);
  let l1 = input.iter().map(|p| p.0 ).collect::<Vec<i32>>();
  let l2 = input.iter().map(|p| p.1 ).collect::<Counter<i32>>();

  let mut ans = 0;
  for i in 0..l1.len() {
    ans += l1[i] * l2[&l1[i]] as i32;
  }
  ans
}