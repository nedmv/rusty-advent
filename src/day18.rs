use std::collections::VecDeque;

const TARGET_ROW: i32 = 70;
const TARGET_COL: i32 = 70;
const TARGET_LIMIT: usize = 1024;

#[derive(PartialEq, Clone)]
enum State {Free, Visited, Blocked}
use State::{Free, Visited, Blocked};

const MOVES: [i32; 5] = [1, 0, -1, 0, 1];

fn bfs(obstacles: &Vec<(usize, usize)>, rows: i32, cols: i32, limit: usize) -> u32 {
  let mut seen = vec![vec![false; (rows as usize)+1]; (cols as usize)+1];

  for i in 0..limit {
    seen[obstacles[i].0][obstacles[i].1] = true;
  }

  let mut q = VecDeque::new();
  q.push_back((0, 0, 0));

  while let Some((cost, row, col)) = q.pop_front() {
    for i in 0..4 {
      let r = row + MOVES[i];
      let c = col + MOVES[i+1];
      if 0 <= r && r <= rows && 0 <= c && c <= cols && !seen[r as usize][c as usize] {
        if (r, c) == (rows, cols) {
          return cost + 1;
        }
        seen[r as usize][c as usize] = true;
        q.push_back((cost+1, r, c));
      }
    }
  }
  u32::MAX
}


fn parse_input(input: &str) -> Vec<(usize, usize)> {
  let mut obstacles = Vec::new();
  let mut it;
  for line in input.lines() {
    it = line.split(',');
    obstacles.push(
    (it.next().unwrap().parse().unwrap(),
     it.next().unwrap().parse().unwrap()));
  }
  obstacles
}

fn solve_part1(input: &str, rows: i32, cols: i32, limit: usize) -> u32 {
  let obstacles = parse_input(input);
  bfs(&obstacles, rows, cols, limit)
}

#[aoc(day18, part1)]
pub fn part1(input: &str) -> u32 {
  solve_part1(input, TARGET_ROW, TARGET_COL, TARGET_LIMIT)
}

fn bfs_from_end(obstacles: &Vec<(usize, usize)>, rows: i32, cols: i32, limit: usize) -> (usize, usize) {
  let mut seen = vec![vec![Free; (rows as usize)+1]; (cols as usize)+1];

  for i in 0..obstacles.len() {
    seen[obstacles[i].0][obstacles[i].1] = Blocked;
  }

  let mut q = VecDeque::new();
  q.push_back((0, 0));

  while let Some((row, col)) = q.pop_front() {
    for i in 0..4 {
      let r = row + MOVES[i];
      let c = col + MOVES[i+1];
      if 0 <= r && r <= rows && 0 <= c && c <= cols && seen[r as usize][c as usize] == Free {
        //no success check based on puzzle condition
        seen[r as usize][c as usize] = Visited;
        q.push_back((r, c));
      }
    }
  }

  for pos in (limit..obstacles.len()).rev() {
    let (row, col) = obstacles[pos];
    seen[row][col] = Free;
    let mut connected = false;
    for i in 0..4 {
      let r = (row as i32) + MOVES[i];
      let c = (col as i32) + MOVES[i+1];
      if 0 <= r && r <= rows && 0 <= c && c <= cols && seen[r as usize][c as usize] == Visited {
        connected = true;
        break;
      }
    }
    if !connected {
      continue;
    }

    q.push_back((row as i32, col as i32));
    let mut finished = false;
    'queue: while let Some((row, col)) = q.pop_front() {
      for i in 0..4 {
        let r = row + MOVES[i];
        let c = col + MOVES[i+1];
        if 0 <= r && r <= rows && 0 <= c && c <= cols && seen[r as usize][c as usize] == Free {
          if r == rows && c == cols {
            finished = true;
            break 'queue;
          }
          seen[r as usize][c as usize] = Visited;
          q.push_back((r, c));
        }
      }
    }
    if finished {
      return (row, col);
    }
  }
  (0, 0)
}

fn solve_part2(input: &str, rows: i32, cols: i32, limit: usize) -> String {
  let obstacles = parse_input(input);
  let pos= bfs_from_end(&obstacles, rows, cols, limit);
  pos.0.to_string() + "," + &pos.1.to_string()
}

#[aoc(day18, part2)]
pub fn part2(input: &str) -> String {
  solve_part2(input, TARGET_ROW, TARGET_COL, TARGET_LIMIT)
}

#[cfg(test)]
mod tests {
  use super::*;
  use indoc::indoc;

  const INPUT: &str = include_str!("../input/2024/day18.txt");
  const ANSWER1: &str = include_str!("../answer/2024/day18p1.txt");
  const ANSWER2: &str = include_str!("../answer/2024/day18p2.txt");

  const EXAMPLE: &str = indoc!("5,4
                                4,2
                                4,5
                                3,0
                                2,1
                                6,3
                                2,4
                                1,5
                                0,6
                                3,3
                                2,6
                                5,1
                                1,2
                                5,5
                                2,5
                                6,5
                                1,4
                                0,4
                                6,4
                                1,1
                                6,1
                                1,0
                                0,5
                                1,6
                                2,0");

  const EXAMPLE_ROW: i32 = 6;
  const EXAMPLE_COL: i32 = 6;
  const EXAMPLE_LIMIT: usize = 12;

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
    assert_eq!(solve_part1(EXAMPLE, EXAMPLE_ROW, EXAMPLE_COL, EXAMPLE_LIMIT), 22);
  }

  #[test]
  fn part2_example() {
    assert_eq!(solve_part2(EXAMPLE, EXAMPLE_ROW, EXAMPLE_COL, EXAMPLE_LIMIT), "6,1");
  }
}