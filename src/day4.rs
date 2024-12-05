
fn get_map(input: &str) -> Vec<Vec<u8>> {
  input
      .lines()
      .map(|l| {
          l.bytes().collect()
      }).collect()
}

const DIRECTIONS: [[i32; 2]; 8] = [[1, 0], [-1, 0], [0, 1], [0, -1], [1, 1], [1, -1], [-1, 1], [-1, -1]];
const PATTERN: [u8; 3] = [b'M', b'A', b'S'];

fn count_xmas(m: &Vec<Vec<u8>>, row: usize, col: usize) -> u32 {
  let mut ans = 0;
  let rows = m.len() as i32;
  let cols = m[0].len() as i32;
  for dir in DIRECTIONS {
    let mut r = row as i32;
    let mut c = col as i32;
    let r_lim = r + 3 * dir[0];
    if r_lim < 0 || r_lim >= rows {
      continue;
    }
    let c_lim = c + 3 * dir[1];
    if c_lim < 0 || c_lim >= cols {
      continue;
    }
    let mut found = true;
    for i in 0..3 {
      r += dir[0];
      c += dir[1];
      if m[r as usize][c as usize] != PATTERN[i] {
        found = false;
        break;
      }
    }
    if found {
      ans += 1;
    }
  }
  ans
}

#[aoc(day4, part1)]
pub fn part1(input: &str) -> u32 {
  let m = get_map(input);
  let rows = m.len();
  let cols = m[0].len();

  let mut ans = 0;
  for row in 0..rows {
    for col in 0..cols {
      if m[row][col] == b'X' {
        ans += count_xmas(&m, row, col);
      }
    }
  }
  ans
}

fn check_x_mas(m: &Vec<Vec<u8>>, r: usize, c: usize) -> bool {
  (m[r][c] == b'A') &&
  ((m[r-1][c-1] == b'M' && m[r+1][c+1] == b'S') || (m[r-1][c-1] == b'S' && m[r+1][c+1] == b'M')) &&
  ((m[r-1][c+1] == b'M' && m[r+1][c-1] == b'S') || (m[r-1][c+1] == b'S' && m[r+1][c-1] == b'M'))
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> u32 {
  let m = get_map(input);
  let rows = m.len();
  let cols = m[0].len();

  let mut ans = 0;
  for row in 1..rows-1 {
    for col in 1..cols-1 {
      if check_x_mas(&m, row, col) {
        ans += 1;
      }
    }
  }
  ans
}