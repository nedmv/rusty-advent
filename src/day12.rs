use crate::utils::get_map;

const MOVES: [i32; 5] = [1, 0, -1, 0, 1];

#[aoc(day12, part1)]
pub fn part1(input: &str) -> usize {
  let m = get_map(input);
  let rows = m.len();
  let cols = m[0].len();
  let mut seen = vec![vec![false; cols]; rows];
  let mut ans = 0;
  let mut q = Vec::new();
  for row in 0..rows {
    for col in 0..cols {
      if !seen[row][col] {
        seen[row][col] = true;
        let target = m[row][col];


        q.push((row, col));
        let mut square = 0;
        let mut perim = 0;
        while !q.is_empty() {
          let (r, c) = q.pop().unwrap();
          square += 1;
          perim += 4;
          for i in 0..4 {
            let (nr, nc) = (r as i32 + MOVES[i], c as i32 + MOVES[i+1]);
            if nr < 0 || nc < 0 {
              continue;
            }
            let (nr, nc) = (nr as usize, nc as usize);
            if nr >= rows || nc >= cols {
              continue;
            }
            if m[nr][nc] == target {
              perim -= 1;
              if !seen[nr][nc] {
                seen[nr][nc] = true;
                q.push((nr, nc));
              }
            }
          }
        }
        ans += square * perim;
      }
    }
  }
  ans
}

#[aoc(day12, part2)]
pub fn part2(input: &str) -> usize {
  let m = get_map(input);
  let rows = m.len();
  let cols = m[0].len();
  let mut seen = vec![vec![false; cols]; rows];
  let mut ans = 0;
  let mut q = Vec::new();
  for row in 0..rows {
    for col in 0..cols {
      if !seen[row][col] {
        seen[row][col] = true;
        let target = m[row][col];


        q.push((row, col));
        let mut square = 0;
        let mut borders = 0;
        while !q.is_empty() {
          let (r, c) = q.pop().unwrap();
          square += 1;
          for i in 0..4 {
            let (nr, nc) = (r as i32 + MOVES[i], c as i32 + MOVES[i+1]);
            let mut has_border = false;
            if nr < 0 || nc < 0 {
              has_border = true;
            }
            let (nr, nc) = (nr as usize, nc as usize);
            if nr >= rows || nc >= cols {
              has_border = true;
            }
            if has_border || m[nr][nc] != target {
              borders += 1;
              let prev_row = r as i32 + MOVES[(i+3) % 4];
              let prev_col = c as i32 + MOVES[i];

              if prev_row < 0 || prev_col < 0 {
                continue;
              }
              let (prev_row, prev_col) = (prev_row as usize, prev_col as usize);
              if prev_row >= rows || prev_col >= cols {
                continue;
              }
              if m[prev_row][prev_col] != target {
                continue;
              }

              let mut prev_has_border = false;
              let prev_neigh_r = prev_row as i32 + MOVES[i];
              let prev_neigh_c = prev_col as i32 + MOVES[i+1];
              if prev_neigh_r < 0 || prev_neigh_c < 0 {
                prev_has_border = true;
              }
              let (prev_neigh_r, prev_neigh_c) = (prev_neigh_r as usize, prev_neigh_c as usize);
              if prev_neigh_r >= rows || prev_neigh_c >= cols {
                prev_has_border = true;
              }
              if prev_has_border || m[prev_neigh_r][prev_neigh_c] != target {
                borders -= 1;
              }
            } else {
              if !seen[nr][nc] {
                seen[nr][nc] = true;
                q.push((nr, nc));
              }
            }
          }
        }
        ans += square * borders;
      }
    }
  }
  ans
}

#[cfg(test)]
mod tests {
  use super::*;

  const INPUT: &str = include_str!("../input/2024/day12.txt");
  const ANSWER1: &str = include_str!("../answer/2024/day12p1.txt");
  const ANSWER2: &str = include_str!("../answer/2024/day12p2.txt");

  const EXAMPLE: &str = concat!("RRRRIICCFF\n",
                                "RRRRIICCCF\n",
                                "VVRRRCCFFF\n",
                                "VVRCCCJFFF\n",
                                "VVVVCJJCFE\n",
                                "VVIVCCJJEE\n",
                                "VVIIICJJEE\n",
                                "MIIIIIJJEE\n",
                                "MIIISIJEEE\n",
                                "MMMISSJEEE");

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
    assert_eq!(part1(EXAMPLE), 1930);
  }

  #[test]
  fn part2_example() {
    assert_eq!(part2(EXAMPLE), 1206);
  }
}