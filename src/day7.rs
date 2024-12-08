
fn parse_input(input: &str) -> Vec<(usize, Vec<usize>)> {
  let mut data = Vec::new();
  for line in input.lines() {
    let mut s = line.split(':');

    let target: usize = s.next().unwrap()
                  .parse().unwrap();
    let nums: Vec<usize> = s.next().unwrap().trim()
                .split(' ').map(|d| d.parse().unwrap())
                .collect();
    data.push((target, nums));
  }
  data
}

fn check(target: usize, nums: &Vec<usize>, pos: usize, cur: usize) -> bool {
  if cur > target {
    return false;
  }
  if pos == nums.len() {
    return cur == target;
  }
  check(target, nums, pos+1, cur + nums[pos]) ||
  check(target, nums, pos+1, cur * nums[pos])
}

fn concatenate(first: usize, second: usize) -> usize {
  let mut t = second;
  let mut count = 0;
  while t > 0 {
    t /= 10;
    count += 1;
  }
  first * 10_usize.pow(count) + second
}

fn check2(target: usize, nums: &Vec<usize>, pos: usize, cur: usize) -> bool {
  if cur > target {
    return false;
  }
  if pos == nums.len() {
    return cur == target;
  }
  check2(target, nums, pos+1, cur + nums[pos]) ||
  check2(target, nums, pos+1, cur * nums[pos]) ||
  check2(target, nums, pos+1, concatenate(cur, nums[pos]))
}

fn solve(input: &str, f: &dyn Fn(usize, &Vec<usize>, usize, usize) -> bool) -> usize {
  let data = parse_input(input);
  let mut ans= 0;
  for (target, nums) in data {
    if f(target, &nums, 1, nums[0]) {
      ans += target;
    }
  }
  ans
}

#[aoc(day7, part1)]
pub fn part1(input: &str) -> usize {
  solve(input, &check)
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> usize {
  solve(input, &check2)
}