use std::collections::{VecDeque, HashMap};
use fasthash::sea::Hash64 as Hash;
use std::mem::swap;

#[derive(PartialEq)]
enum Moves{UP = b'^' as isize,
           DOWN = b'v' as isize,
           LEFT = b'<' as isize,
           RIGHT = b'>' as isize}

impl Moves {
    const VALUES: [Self; 4] = [Self::UP, Self::DOWN, Self::LEFT, Self::RIGHT];
}

type Keypad = &'static[&'static[u8]];
const KEYPADS: [Keypad; 2] = [&[&[b'7', b'8', b'9'], &[b'4', b'5', b'6'], &[b'1', b'2', b'3'], &[b'#', b'0', b'A']],
                              &[&[b'#', b'^', b'A'], &[b'<', b'v', b'>']]];

fn number_to_id(num: u8) -> usize {
  match num {
    b'7' => {0}, b'8' => {1}, b'9' => {2},
    b'4' => {3}, b'5' => {4}, b'6' => {5},
    b'1' => {6}, b'2' => {7}, b'3' => {8},
    b'#' => {9}, b'0' => {10},b'A' => {11}
    _ => {panic!("Incorrect symbol on numeric keypad!")}
  }
}

fn direction_to_id(dir: u8) -> usize {
  match dir {
    b'#' => {0}, b'^' => {1}, b'A' => {2},
    b'<' => {3}, b'v' => {4}, b'>' => {5},
    _ => {panic!("Incorrect symbol on directional keypad!")}
  }
}

fn to_char(id: usize, keypad: Keypad) -> u8 {
  keypad[id/3][id%3]
}

fn next_id(id: usize, limit: usize, dir: &Moves) -> Option<usize> {
  match dir {
    Moves::UP => {if id > 2 {Some(id-3)} else {None}},
    Moves::DOWN => {if id + 3 < limit {Some(id+3)} else {None}},
    Moves::LEFT => {if id % 3 > 0 {Some(id-1)} else {None}},
    Moves::RIGHT => {if id % 3 < 2 {Some(id+1)} else {None}}
  }
}

fn get_paths(start_id: usize, end_id: usize, forbidden_id: usize, limit: usize) -> Vec<Vec<u8>> {
  let mut paths = Vec::new();
  let diff = ((start_id / 3) as i32 - (end_id / 3) as i32).abs() +
             ((start_id % 3) as i32 - (end_id % 3) as i32).abs();

  let mut q = VecDeque::new();
  q.push_back((0, start_id, Vec::new()));

  while let Some((d, id, mut path)) = q.pop_front() {
    if d > diff {
      break;
    }
    if d == diff {
      if id == end_id {
        path.push(b'A'); // all paths should end with 'A' on next level
        paths.push(path);
      }
      continue;
    }

    for dir in Moves::VALUES {
      if let Some(next) = next_id(id, limit, &dir) {
        if next == forbidden_id {
          continue;
        }
        let mut new_path = path.clone();
        new_path.push(dir as u8);
        q.push_back((d+1, next, new_path));
      }
    }
  }
  paths
}

fn precalculate_paths(keypad: Keypad) -> Vec<Vec<Vec<Vec<u8>>>> {
  let rows = keypad.len(); let cols = keypad[0].len();
  let limit = rows * cols;
  let mut paths = vec![vec![Vec::new(); limit]; limit];

  let mut forbidden_id = 0;
  for id in 0..limit {
    if to_char(id, keypad) == b'#' {
      forbidden_id = id;
      break;
    }
  }
  for start_id in 0..limit {
    if start_id == forbidden_id {
      continue;
    }
    for end_id in 0..limit {
      if start_id == forbidden_id {
        continue;
      }
      if start_id == end_id {
        paths[start_id][end_id] = vec![vec![b'A']; 1]; // should use 'A' on next level
        continue;
      }
      paths[start_id][end_id] = get_paths(start_id, end_id, forbidden_id, limit);
    }
  }
  paths
}

fn generate_initial_paths(code: &Vec<usize>, keypad_paths: &Vec<Vec<Vec<Vec<u8>>>>) -> Vec<Vec<u8>> {
  let mut paths: Vec<Vec<u8>> = vec![Vec::new(); 1];
  let mut next = Vec::new();
  let mut prev_id = number_to_id(b'A');
  for &id in code {
    let cur_paths = keypad_paths[prev_id][id].clone();
    prev_id = id;
    for p in &paths {
      for cp in &cur_paths {
        let mut cur: Vec<u8> = p.clone();
        cur.extend(cp);
        // cur.push(b'A');
        next.push(cur);
      }
    }
    swap(&mut paths, &mut next);
    next.clear();
  }
  paths
}

fn find_optimal_solution(path: &Vec<u8>, depth: usize, keypad_paths: &Vec<Vec<Vec<Vec<u8>>>>, cache: &mut Vec<HashMap<Vec<u8>, usize, Hash>>) -> usize {
  if depth == 0 {
    return path.len();
  }
  if let Some(&val) = cache[depth].get(path) {
    return val;
  }

  let mut ans = 0;
  let mut prev_id = direction_to_id(b'A');
  for &ch in path {
    let id = direction_to_id(ch);
    let next_paths = keypad_paths[prev_id][id].clone();

    let mut cur_min = usize::MAX;
    for p in &next_paths {
      cur_min = cur_min.min(find_optimal_solution(&p, depth-1, keypad_paths, cache));
    }
    prev_id = id;
    ans += cur_min;
  }
  cache[depth].insert(path.clone(), ans);
  ans
}

fn enter_code(code: &str, depth: usize, keypad_paths: &[Vec<Vec<Vec<Vec<u8>>>>; 2]) -> usize {
  let code: Vec<_> = code.bytes().map(|b| number_to_id(b)).collect();
  let paths = generate_initial_paths(&code, &keypad_paths[0]);

  let mut best = usize::MAX;
  let mut cache = (0..=depth).map(|_| HashMap::with_hasher(Hash)).collect();
  for path in paths {
    best = best.min(find_optimal_solution(&path, depth, &keypad_paths[1], &mut cache));
  }
  best
}

fn solve(input: &str, intermediate_keypads: usize) -> usize {
  let keypad_paths: [Vec<Vec<Vec<Vec<u8>>>>; 2] = [precalculate_paths(KEYPADS[0]), precalculate_paths(KEYPADS[1])];

  let mut ans = 0;
  for code in input.lines() {
    let num: usize = code[..3].parse().unwrap();
    let score = enter_code(code, intermediate_keypads, &keypad_paths);
    ans += num * score;
  }
  ans
}

#[aoc(day21, part1)]
pub fn part1(input: &str) -> usize {
  solve(input, 2)
}

#[aoc(day21, part2)]
pub fn part2(input: &str) -> usize {
  solve(input, 25)
}

#[cfg(test)]
mod tests {
  use super::*;
  use indoc::indoc;

  const INPUT: &str = include_str!("../input/2024/day21.txt");
  const ANSWER1: &str = include_str!("../answer/2024/day21p1.txt");
  const ANSWER2: &str = include_str!("../answer/2024/day21p2.txt");

  const EXAMPLE: &str = indoc!("029A
                                980A
                                179A
                                456A
                                379A");


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
    assert_eq!(part1(EXAMPLE), 126384);
  }

  #[test]
  fn part2_example() {
    assert_eq!(part2(EXAMPLE), 154115708116294);
  }
}