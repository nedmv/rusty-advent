
use crate::utils::get_map;

fn get_antennas(m: &Vec<Vec<u8>>) -> Vec<Vec<(i32, i32)>> {
  let rows = m.len();
  let cols = m[0].len();
  let mut antennas = vec![Vec::new(); 256];
  for row in 0..rows {
    for col in 0..cols {
      if m[row][col] != b'.' {
        antennas[m[row][col] as usize].push((row as i32, col as i32));
      }
    }
  }
  antennas
}

fn calc_antinodes(m: &Vec<Vec<u8>>) -> u32 {
  let rows = m.len();
  let cols = m[0].len();
  let mut ans = 0;
  for row in 0..rows {
    for col in 0..cols {
      if m[row][col] == 0 {
        ans += 1;
      }
    }
  }
  ans
}

#[aoc(day8, part1)]
pub fn part1(input: &str) -> u32 {
  let mut m = get_map(input);
  let rows = m.len() as i32;
  let cols = m[0].len() as i32;
  for ant in get_antennas(&m) {
    if ant.len() < 2 {
      continue;
    }
    for &a in &ant {
      for &b in &ant {
        if a == b {
          continue;
        }
        let (dx, dy) = (a.0 - b.0, a.1 - b.1);
        let (x, y) = (a.0 + dx, a.1 + dy);

        if x >= 0 && x < rows && y >= 0 && y < cols {
          m[x as usize][y as usize] = 0;
        }
      }
    }
  }
  calc_antinodes(&m)
}

#[aoc(day8, part2)]
pub fn part2(input: &str) -> u32 {
  let mut m = get_map(input);
  let rows = m.len() as i32;
  let cols = m[0].len() as i32;
  for ant in get_antennas(&m) {
    if ant.len() < 2 {
      continue;
    }
    for &a in &ant {
      for &b in &ant {
        if a == b {
          continue;
        }
        let (dx, dy) = (a.0 - b.0, a.1 - b.1);
        let (mut x, mut y) = a;

        while x >= 0 && x < rows && y >= 0 && y < cols {
          m[x as usize][y as usize] = 0;
          x += dx;
          y += dy;
        }
      }
    }
  }
  calc_antinodes(&m)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input/2024/day8.txt");
    const ANSWER1: &str = include_str!("../answer/2024/day8p1.txt");
    const ANSWER2: &str = include_str!("../answer/2024/day8p2.txt");

    #[test]
    fn part1_local() {
        assert_eq!(part1(INPUT).to_string(), ANSWER1);
    }

    #[test]
    fn part2_local() {
        assert_eq!(part2(INPUT).to_string(), ANSWER2);
    }
}