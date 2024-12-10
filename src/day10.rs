use crate::utils::get_map;

const MOVES: [i32; 5] = [1, 0, -1, 0, 1];

#[aoc(day10, part1)]
pub fn part1(input: &str) -> usize {
  let mut m = get_map(input);
  let rows = m.len();
  let cols = m[0].len();
  let mut ans = 0;
  let mut q = Vec::with_capacity(361);
  for row in 0..rows {
    for col in 0..cols {
      if m[row][col] != b'0' {
        continue;
      }

      q.push((row, col));
      m[row][col] += 11; // mark as seen

      while !q.is_empty() {
        let (r, c) = q.pop().unwrap();
        for i in 0..4 {
          let (nr, nc) = (r as i32 + MOVES[i], c as i32 + MOVES[i+1]);
          if nr < 0 || nc < 0 {
            continue;
          }
          let (nr, nc) = (nr as usize, nc as usize);
          if nr >= rows || nc >= cols {
            continue;
          }
          if m[nr][nc] != m[r][c] - 10 { // + 1
            continue;
          }
          if m[nr][nc] == b'9' {
            ans += 1;
          } else if m[nr][nc] < b'9' {
            q.push((nr, nc));
          }
          m[nr][nc] += 11;
        }
      }

      //restore seen
      q.push((row, col));
      m[row][col] -= 11;

      while !q.is_empty() {
        let (r, c) = q.pop().unwrap();
        for i in 0..4 {
          let (nr, nc) = (r as i32 + MOVES[i], c as i32 + MOVES[i+1]);
          if nr < 0 || nc < 0 {
            continue;
          }
          let (nr, nc) = (nr as usize, nc as usize);
          if nr >= rows || nc >= cols {
            continue;
          }
          if m[nr][nc] > b'9' {
            m[nr][nc] -= 11;
            q.push((nr, nc));
          }
        }
      }
    }
  }
  ans
}

#[aoc(day10, part2)]
pub fn part2(input: &str) -> usize {
  let m = get_map(input);
  let rows = m.len();
  let cols = m[0].len();
  let mut ans = 0;
  let mut q = Vec::with_capacity(361);
  for row in 0..rows {
    for col in 0..cols {
      if m[row][col] != b'0' {
        continue;
      }
      q.push((row, col));

      while !q.is_empty() {
        let (r, c) = q.pop().unwrap();
        for i in 0..4 {
          let (nr, nc) = (r as i32 + MOVES[i], c as i32 + MOVES[i+1]);
          if nr < 0 || nc < 0 {
            continue;
          }
          let (nr, nc) = (nr as usize, nc as usize);
          if nr >= rows || nc >= cols {
            continue;
          }
          if m[nr][nc] != m[r][c] + 1 {
            continue;
          }
          if m[nr][nc] == b'9' {
            ans += 1;
          } else {
            q.push((nr, nc));
          }
        }
      }
    }
  }
  ans
}

#[cfg(test)]
mod tests {
  use super::*;

  const INPUT: &str = include_str!("../input/2024/day10.txt");
  const ANSWER1: &str = include_str!("../answer/2024/day10p1.txt");
  const ANSWER2: &str = include_str!("../answer/2024/day10p2.txt");

  const EXAMPLE: &str = concat!("89010123\n",
                                "78121874\n",
                                "87430965\n",
                                "96549874\n",
                                "45678903\n",
                                "32019012\n",
                                "01329801\n",
                                "10456732");

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
    assert_eq!(part1(EXAMPLE), 36);
  }

  #[test]
  fn part2_example() {
    assert_eq!(part2(EXAMPLE), 81);
  }
}