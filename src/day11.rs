use nalgebra::base::DMatrix;

// Source: https://www.reddit.com/r/adventofcode/comments/1hbtz8w/2024_day_11_every_sequence_converges_to_3947/
const TOTAL_STONES: usize = 3947;

fn get_stones(input: &str) -> Vec<usize> {
  input.trim_end().split(' ').map(|d| d.parse().unwrap()).collect()
}

fn get_next(stone: usize) -> (usize, Option<usize>) {
  if stone == 0 {
    return (1, None);
  }
  let s = stone.to_string();
  let size = s.len();
  if size % 2 == 0 {
    let (left, right) = s.split_at(size/2);
    return (left.parse().unwrap(), Some(right.parse().unwrap()));
  }
  (2024 * stone, None)
}

fn get_transition_matrix(input: &str) -> DMatrix<usize> {
  let mut stones = get_stones(input);
  let mut pos = 0;
  let mut transitions = Vec::new();

  while pos < stones.len() {
    let stone = stones[pos];
    let next = get_next(stone);
    let t = next.0;

    let mut next_pos = stones.len();
    for i in 0..stones.len() {
      if stones[i] == t {
        next_pos = i;
        break;
      }
    }
    if next_pos == stones.len() {
      stones.push(t);
    }
    transitions.push((pos, next_pos));

    if let Some(t) = next.1 {
      let mut next_pos = stones.len();
      for i in 0..stones.len() {
        if stones[i] == t {
          next_pos = i;
          break;
        }
      }
      if next_pos == stones.len() {
        stones.push(t);
      }
      transitions.push((pos, next_pos));
    }
    pos += 1;
  }

  let mut m = DMatrix::<usize>::zeros(stones.len(), stones.len());

  for t in transitions {
    m[t] += 1;
  }
  m
}

#[allow(dead_code)]
// #[aoc(day11, part1, matrix)]
pub fn part1_matrix(input: &str) -> usize {
  let m = get_transition_matrix(input);
  let m = m.pow(25);

  let mut stones = DMatrix::<usize>::zeros(m.nrows(), m.ncols());
  for init in 0..get_stones(input).len() {
    for i in 0..m.nrows() {
      stones[(init, i)] = 1;
    }
  }

  stones.dot(&m)
}

#[allow(dead_code)]
// #[aoc(day11, part2, matrix)]
pub fn part2_matrix(input: &str) -> usize {
  let m = get_transition_matrix(input);
  let m = m.pow(75);

  let mut stones = DMatrix::<usize>::zeros(m.nrows(), m.ncols());
  for init in 0..get_stones(input).len() {
    for i in 0..m.nrows() {
      stones[(init, i)] = 1;
    }
  }

  stones.dot(&m)
}

fn get_transitions(input: &str) -> (usize, usize, Vec<(usize, usize)>) {
  let mut stones = get_stones(input);
  let n = stones.len();
  let mut pos = 0;
  let mut transitions = Vec::new();

  while pos < stones.len() {
    let stone = stones[pos];
    let next = get_next(stone);
    let t = next.0;

    let mut next_pos = stones.len();
    for i in 0..stones.len() {
      if stones[i] == t { 
        next_pos = i;
        break;
      }
    }
    if next_pos == stones.len() {
      stones.push(t);
    }
    transitions.push((pos, next_pos));

    if let Some(t) = next.1 {
      let mut next_pos = stones.len();
      for i in 0..stones.len() {
        if stones[i] == t {
          next_pos = i;
          break;
        }
      }
      if next_pos == stones.len() {
        stones.push(t);
      }
      transitions.push((pos, next_pos));
    }
    pos += 1;
  }
  (n, stones.len(), transitions)
}

fn solve_counter(input: &str, steps: usize) -> usize {
  let (start, total, transitions) = get_transitions(input);
  let mut cur = vec![0; total];
  for i in 0..start {
    cur[i] = 1;
  }
  let mut next = vec![0; total];

  for _ in 0..steps {
    for &(from, to) in &transitions {
      next[to] += cur[from];
    }
    for i in 0..total {
      cur[i] = next[i];
      next[i] = 0;
    }
  }
  cur.iter().sum()
}

#[aoc(day11, part1, counter)]
pub fn part1(input: &str) -> usize {
  solve_counter(input, 25)
}

#[aoc(day11, part2, counter)]
pub fn part2(input: &str) -> usize {
  solve_counter(input, 75)
}

#[cfg(test)]
mod tests {
  use super::*;

  const INPUT: &str = include_str!("../input/2024/day11.txt");
  const ANSWER1: &str = include_str!("../answer/2024/day11p1.txt");
  const ANSWER2: &str = include_str!("../answer/2024/day11p2.txt");

  const EXAMPLE: &str = concat!("125 17");

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
    assert_eq!(part1(EXAMPLE), 55312);
  }
}