use super::utils::get_map;
use std::collections::VecDeque;

const MOVES: [i32;5] = [-1, 0, 1, 0, -1]; // NESW

fn flood_fill(m: &Vec<Vec<u8>>, start: (usize, usize)) -> Vec<Vec<u32>> {
  let rows = m.len(); let cols = m[0].len();
  let mut distance = vec![vec![0; cols]; rows];
  distance[start.0][start.1] = u32::MAX; // temporary make start unreachable

  let mut q = VecDeque::new();
  q.push_back((0, start.0 as i32, start.1 as i32));

  while let Some((cost, row, col)) = q.pop_front() {
    for i in 0..4 {
      let r = row + MOVES[i];
      let c = col + MOVES[i+1];

      if m[r as usize][c as usize] != b'#' && distance[r as usize][c as usize] == 0 {
        distance[r as usize][c as usize] = cost + 1;
        q.push_back((cost+1, r, c));
      }
    }
  }
  distance[start.0][start.1] = 0;
  distance
}

fn solve(input: &str, cheat_limit: usize) -> usize {
  let m = get_map(input);
  let rows = m.len();
  let cols = m[0].len();
  let mut start = (0, 0);
  let mut end = (0, 0);
  for row in 1..rows-1 {
    for col in 1..cols-1 {
      match m[row][col] {
        b'S' => {start = (row, col); if end != (0, 0) {break}},
        b'E' => {end = (row, col); if start != (0, 0) {break}},
        _ => {}
      }
    }
  }
  let distance_from_start = flood_fill(&m, start);
  let distance_from_end = flood_fill(&m, end);

  let record_without_cheating = distance_from_start[end.0][end.1];
  let mut ans = 0;

  for start_row in 1..rows-1 {
    for start_col in 1..cols-1 {
      if m[start_row][start_col] == b'#' {
        continue;
      }
      for end_row in 1.max(start_row.saturating_sub(cheat_limit))..=(rows-1).min(start_row + cheat_limit) {
        for end_col in 1.max(start_col.saturating_sub(cheat_limit))..=(cols-1).min(start_col + cheat_limit) {
          if m[end_row][end_col] == b'#' {
            continue;
          }
          let d = start_row.max(end_row) - start_row.min(end_row) + start_col.max(end_col) - start_col.min(end_col);
          if d == 0 || d > cheat_limit {
            continue;
          }
          let cheat_distance = distance_from_start[start_row][start_col] + distance_from_end[end_row][end_col] + d as u32;
          if cheat_distance + 100 <= record_without_cheating {
            ans += 1;
          }
        }
      }
    }
  }
  ans
}


#[aoc(day20, part1)]
pub fn part1(input: &str) -> usize {
  solve(input, 2)
}

#[aoc(day20, part2)]
pub fn part2(input: &str) -> usize {
  solve(input, 20)
}

#[cfg(test)]
mod tests {
  use super::*;
  // use indoc::indoc;

  const INPUT: &str = include_str!("../input/2024/day20.txt");
  const ANSWER1: &str = include_str!("../answer/2024/day20p1.txt");
  const ANSWER2: &str = include_str!("../answer/2024/day20p2.txt");

  // const EXAMPLE: &str = indoc!("###############
  //                               #...#...#.....#
  //                               #.#.#.#.#.###.#
  //                               #S#...#.#.#...#
  //                               #######.#.#.###
  //                               #######.#.#...#
  //                               #######.#.###.#
  //                               ###..E#...#...#
  //                               ###.#######.###
  //                               #...###...#...#
  //                               #.#####.#.###.#
  //                               #.#...#.#.#...#
  //                               #.#.#.#.#.#.###
  //                               #...#...#...###
  //                               ###############");


  #[test]
  fn part1_local() {
    assert_eq!(part1(INPUT).to_string(), ANSWER1);
  }

  #[test]
  fn part2_local() {
    assert_eq!(part2(INPUT).to_string(), ANSWER2);
  }
}