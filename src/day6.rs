
use crate::utils::get_map;
use std::collections::HashSet;
use fasthash::sea::Hash64 as Hash;

const MOVES: [i32;5] = [-1, 0, 1, 0, -1]; // NESW

fn find_start(m: &Vec<Vec<u8>>) -> (usize, usize) {
  for row in 0..m.len() {
    for col in 0..m[0].len() {
      if m[row][col] == b'^' {
        return (row, col);
      }
    }
  }
  (0, 0)
}

fn get_seen(m: &Vec<Vec<u8>>, start: (usize, usize), dir: usize) -> HashSet<(usize, usize), Hash> {
  let mut seen = HashSet::with_hasher(Hash);
  let mut cur = (start.0, start.1, dir);
  let rows = m.len() as i32;
  let cols = m[0].len() as i32;
  loop {
    seen.insert((cur.0, cur.1));

    for dir in cur.2..cur.2+4 {
      let r = cur.0 as i32 + MOVES[dir % 4];
      let c = cur.1 as i32 + MOVES[(dir+1) % 4];
      if r < 0 || r >= rows || c < 0 || c >= cols {
        return seen;
      }
      let r = r as usize;
      let c = c as usize;
      if m[r][c] != b'#' {
        cur = (r, c, dir);
        break;
      }
    }
  }
}

#[aoc(day6, part1)]
pub fn part1(input: &str) -> usize {
  let m = get_map(input);
  let start = find_start(&m);
  let seen = get_seen(&m, start, 0);
  seen.len()
}

fn is_loop(m: &mut Vec<Vec<u8>>, start: (usize, usize), dir: usize, obstacle: (usize, usize)) -> bool {
  m[obstacle.0][obstacle.1] = b'#';

  let mut seen = HashSet::with_hasher(Hash);
  let mut cur = (start.0, start.1, dir);
  let rows = m.len() as i32;
  let cols = m[0].len() as i32;
  let mut looped = false;

  'outer: loop {
    if !seen.insert(cur) {
      looped = true;
      break;
    }

    for dir in cur.2..cur.2+4 {
      let r = cur.0 as i32 + MOVES[dir % 4];
      let c = cur.1 as i32 + MOVES[(dir+1) % 4];
      if r < 0 || r >= rows || c < 0 || c >= cols {
        break 'outer;
      }
      let r = r as usize;
      let c = c as usize;
      if m[r][c] != b'#' {
        cur = (r, c, dir % 4);
        break;
      }
    }
  }

  m[obstacle.0][obstacle.1] = b'.';
  looped
}


#[aoc(day6, part2)]
pub fn part2(input: &str) -> usize {
  let mut m = get_map(input);
  let start = find_start(&m);
  let mut seen = get_seen(&m, start, 0);
  seen.remove(&start);
  let mut ans = 0;
  for obstacle in seen {
    if is_loop(&mut m, start, 0, obstacle) {
      ans += 1;
    }
  }
  ans
}

