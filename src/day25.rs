
fn parse_lock(m: &Vec<&[u8]>) -> [usize; 5] {
  let mut ans = [0; 5];
  for col in 0..5 {
    for row in 1..7 {
      if m[row][col] == b'.' {
        ans[col] = row-1;
        break;
      }
    }
  }
  ans
}

fn parse_key(m: &Vec<&[u8]>) -> [usize; 5] {
  let mut ans = [0; 5];
  for col in 0..5 {
    for row in (0..6).rev() {
      if m[row][col] == b'.' {
        ans[col] = 5-row;
        break;
      }
    }
  }
  ans
}

#[aoc(day25, part1)]
pub fn part1(input: &str) -> usize {
  let mut keys = Vec::new();
  let mut locks = Vec::new();
  for lines in input.split("\n\n").map(|s| s.lines().map(|l| l.as_bytes()).collect::<Vec<_>>()) {
    if lines[0][0] == b'#' {
      locks.push(parse_lock(&lines));
    } else {
      keys.push(parse_key(&lines));
    }
  }

  let mut ans = 0;
  for lock in &locks {
    'outer: for key in &keys {
      for i in 0..5 {
        if lock[i] + key[i] > 5 {
          continue 'outer;
        }
      }
      ans += 1;
    }
  }
  ans
}

#[aoc(day25, part2)]
pub fn part2(_input: &str) -> usize {
  0
}

#[cfg(test)]
mod tests {
  use super::*;
  use indoc::indoc;

  const INPUT: &str = include_str!("../input/2024/day25.txt");
  const ANSWER1: &str = include_str!("../answer/2024/day25p1.txt");
  const ANSWER2: &str = include_str!("../answer/2024/day25p2.txt");

  const EXAMPLE: &str = indoc!("#####
                                .####
                                .####
                                .####
                                .#.#.
                                .#...
                                .....

                                #####
                                ##.##
                                .#.##
                                ...##
                                ...#.
                                ...#.
                                .....

                                .....
                                #....
                                #....
                                #...#
                                #.#.#
                                #.###
                                #####

                                .....
                                .....
                                #.#..
                                ###..
                                ###.#
                                ###.#
                                #####

                                .....
                                .....
                                .....
                                #....
                                #.#..
                                #.#.#
                                #####");

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
    assert_eq!(part1(EXAMPLE), 3);
  }
}