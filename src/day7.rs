
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

fn count_digits(num: usize) -> u32 {
  let mut count = 0;
  let mut t = num;
  while t > 0 {
    t /= 10;
    count += 1;
  }
  count as u32
}

#[allow(dead_code)]
fn check(target: usize, nums: &Vec<usize>, pos: usize, cur: usize) -> bool {
  if pos == nums.len() {
    return cur == target;
  }
  check(target, nums, pos+1, cur + nums[pos]) ||
  check(target, nums, pos+1, cur * nums[pos])
}

#[allow(dead_code)]
fn concatenate(first: usize, second: usize) -> usize {
  first * 10_usize.pow(count_digits(second)) + second
}

#[allow(dead_code)]
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

#[allow(dead_code)]
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

fn check_from_tail(target: usize, nums: &Vec<usize>, pos: usize) -> bool {
  if pos == nums.len() {
    return target == 0;
  }
  if target == 0 {
    return false;
  }
  let id = nums.len() - pos - 1;
  (target >= nums[id] && check_from_tail(target - nums[id], nums, pos+1)) ||
  ((target % nums[id] == 0) && check_from_tail(target / nums[id], nums, pos+1))
}

fn check_from_tail2(target: usize, nums: &Vec<usize>, pos: usize) -> bool {
  if pos == nums.len() {
    return target == 0;
  }
  if target == 0 {
    return false;
  }
  let id = nums.len() - pos - 1;
  (target >= nums[id] && check_from_tail2(target - nums[id], nums, pos+1)) ||
  ((target % nums[id] == 0) && check_from_tail2(target / nums[id], nums, pos+1)) ||
  {
    let power = 10_usize.pow(count_digits(nums[id]));
    ((target-nums[id]) % power == 0) && check_from_tail2(target / power, nums, pos+1)
  }
}

fn solve_from_tail(input: &str, f: &dyn Fn(usize, &Vec<usize>, usize) -> bool) -> usize {
  let data = parse_input(input);
  let mut ans= 0;
  for (target, nums) in data {
    if f(target, &nums, 0) {
      ans += target;
    }
  }
  ans
}

#[aoc(day7, part1)]
pub fn part1(input: &str) -> usize {
  // solve(input, &check)
  solve_from_tail(input, &check_from_tail)
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> usize {
  // solve(input, &check2)
  solve_from_tail(input, &check_from_tail2)
}