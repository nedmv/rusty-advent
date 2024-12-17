use crate::utils::get_map;
use std::collections::BinaryHeap;
const MOVES: [i32; 5] = [-1, 0, 1, 0, -1]; //NESW

fn parse_input(input: &str) -> (Vec<Vec<u8>>, (usize, usize), (usize, usize)) {
  let m = get_map(input);
  let start = (m.len()-2, 1);
  let fin = (1, m[0].len()-2);
  (m, start, fin)
}

#[aoc(day16, part1)]
pub fn part1(input: &str) -> i32 {
  let (m, start, fin) = parse_input(input);
  let rows = m.len(); let cols = m[0].len();
  type E = (i32, usize, usize, usize); //score, row, col, dir
  let mut pq: BinaryHeap<E> = BinaryHeap::new();
  pq.push((0, start.0, start.1, 1));

  let mut d = vec![vec![[i32::MIN; 4]; cols]; rows];
  let mut r;
  let mut c;
  let mut sc; 

  while let Some((score, row, col, dir)) = pq.pop() {
    if (row, col) == fin {
      return -score;
    }

    for new_dir in [dir, (dir+1) % 4, (dir+3)%4] {
      sc = if dir == new_dir {score-1} else {score-1001};
      r = (row as i32 + MOVES[new_dir]) as usize;
      c = (col as i32 + MOVES[new_dir+1]) as usize;
      if m[r][c] != b'#' && d[r][c][new_dir] < sc {
        d[r][c][new_dir] = sc;
        pq.push((sc, r, c, new_dir));
      }
    }
  }
  0
}


#[aoc(day16, part2)]
pub fn part2(input: &str) -> i32 {
  let (m, start, fin) = parse_input(input);
  let rows = m.len(); let cols = m[0].len();
  type E = (i32, usize, usize, usize, Vec::<(usize, usize)>); //score, row, col, dir, path
  let mut pq: BinaryHeap<E> = BinaryHeap::new();
  pq.push((0, start.0, start.1, 1, vec![start]));

  let mut d = vec![vec![[i32::MIN; 4]; cols]; rows];
  let mut r;
  let mut c;
  let mut sc;
  let mut limit = i32::MIN;
  let mut best = vec![vec![false; cols]; rows];

  while let Some((score, row, col, dir, path)) = pq.pop() {
    if score < limit {
      break;
    }
    if (row, col) == fin {
      limit = score;
      for p in path {
        best[p.0][p.1] = true;
      }
      continue;
    }
    if score == limit {
      continue;
    }

    for new_dir in [dir, (dir+1) % 4, (dir+3)%4] {
      sc = if dir == new_dir {score-1} else {score-1001};
      r = (row as i32 + MOVES[new_dir]) as usize;
      c = (col as i32 + MOVES[new_dir+1]) as usize;
      if m[r][c] != b'#' && d[r][c][new_dir] < sc + 1000 {
        if d[r][c][new_dir] < sc {
          d[r][c][new_dir] = sc;
        }
        let mut new_path = path.clone();
        new_path.push((r, c));
        pq.push((sc, r, c, new_dir, new_path));
      }
    }
  }
  let mut ans = 0;
  for row in 1..rows-1 {
    for col in 1..cols-1 {
      if best[row][col] {
        ans += 1;
      }
    }
  }
  ans
}

#[cfg(test)]
mod tests {
  use super::*;
  use indoc::indoc;

  const INPUT: &str = include_str!("../input/2024/day16.txt");
  const ANSWER1: &str = include_str!("../answer/2024/day16p1.txt");
  const ANSWER2: &str = include_str!("../answer/2024/day16p2.txt");

  const EXAMPLE_1: &str = indoc!("###############
                                  #.......#....E#
                                  #.#.###.#.###.#
                                  #.....#.#...#.#
                                  #.###.#####.#.#
                                  #.#.#.......#.#
                                  #.#.#####.###.#
                                  #...........#.#
                                  ###.#.#####.#.#
                                  #...#.....#.#.#
                                  #.#.#.###.#.#.#
                                  #.....#...#.#.#
                                  #.###.#.#.#.#.#
                                  #S..#.....#...#
                                  ###############");

  const EXAMPLE_2: &str = indoc!("#################
                                  #...#...#...#..E#
                                  #.#.#.#.#.#.#.#.#
                                  #.#.#.#...#...#.#
                                  #.#.#.#.###.#.#.#
                                  #...#.#.#.....#.#
                                  #.#.#.#.#.#####.#
                                  #.#...#.#.#.....#
                                  #.#.#####.#.###.#
                                  #.#.#.......#...#
                                  #.#.###.#####.###
                                  #.#.#...#.....#.#
                                  #.#.#.#####.###.#
                                  #.#.#.........#.#
                                  #.#.#.#########.#
                                  #S#.............#
                                  #################");

  #[test]
  fn part1_local() {
    assert_eq!(part1(INPUT).to_string(), ANSWER1);
  }

  #[test]
  fn part2_local() {
    assert_eq!(part2(INPUT).to_string(), ANSWER2);
  }

  #[test]
  fn part1_example1() {
    assert_eq!(part1(EXAMPLE_1), 7036);
  }

  #[test]
  fn part2_example1() {
    assert_eq!(part2(EXAMPLE_1), 45);
  }

  #[test]
  fn part1_example2() {
    assert_eq!(part1(EXAMPLE_2), 11048);
  }

  #[test]
  fn part2_example2() {
    assert_eq!(part2(EXAMPLE_2), 64);
  }
}