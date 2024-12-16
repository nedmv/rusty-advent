use crate::utils::get_map;
use std::collections::HashSet;
use fasthash::sea::Hash64 as Hash;

fn parse_input(input: &str) -> (Vec<Vec<u8>>, Vec<u8>) {
  let mut input = input.trim().split("\n\n");
  let m = get_map(input.next().unwrap());
  let seq = input.next().unwrap().split('\n').fold("".to_string(), |acc, e| acc + e);
  (m, seq.into_bytes())
}

fn find_start(m: &mut Vec<Vec<u8>>) -> (usize, usize) {
  for row in 1..m.len()-1 {
    for col in 1..m[0].len()-1 {
      if m[row][col] == b'@' {
        m[row][col] = b'.'; //ignore @ placement from now on
        return (row, col);
      }
    }
  }
  (0, 0)
}

fn get_step(ch: u8) -> (i32, i32) {
  match ch {
    b'^' => {(-1, 0)},
    b'>' => {(0, 1)},
    b'v' => {(1, 0)},
    b'<' => {(0, -1)}, 
    _ => {(0, 0)}
  }
}

fn get_next_pos(pos: (usize, usize), d: (i32, i32)) -> (usize, usize) {
  ((pos.0 as i32 + d.0) as usize, (pos.1 as i32 + d.1) as usize)
}

fn calc_score(m: &Vec<Vec<u8>>, ch: u8) -> usize {
  let mut score = 0;
  for row in 1..m.len()-1 {
    for col in 1..m[0].len()-1 {
      if m[row][col] == ch {
        score += 100 * row + col;
      }
    }
  }
  score
}

#[aoc(day15, part1)]
pub fn part1(input: &str) -> usize {
  let (mut m, seq) = parse_input(input);
  let mut pos = find_start(&mut m);

  let mut d;
  let mut robot_next;

  'outer: for ch in seq {
    d = get_step(ch);
    robot_next = get_next_pos(pos, d);
    let mut next = robot_next;
    loop {
      match m[next.0][next.1] {
         b'#' => {continue 'outer;},
         b'O' => {
           next = get_next_pos(next, d);
         },
         _ => {break;}
      }
    }
    pos = robot_next;
    if pos != next { // moved some boxes
      m[next.0][next.1] = b'O';
      m[pos.0][pos.1] = b'.';
    }
  }

  calc_score(&m, b'O')
}

fn expand(m: &Vec<Vec<u8>>) -> (Vec<Vec<u8>>, (usize, usize)) {
  let (rows, cols) = (m.len(), m[0].len());
  let mut new_m = vec![vec![b'.'; 2 * cols]; rows];
  let mut start = (0, 0);
  for row in 0..rows {
    for col in 0..cols {
      match m[row][col] {
        b'#' => {
          let c = col << 1;
          new_m[row][c] = b'#';
          new_m[row][c+1] = b'#';
        },
        b'O' => {
          let c = col << 1;
          new_m[row][c] = b'[';
          new_m[row][c+1] = b']';
        },
        b'@' => {
          let c = col << 1;
          new_m[row][c] = b'.'; // ignore '@' placement
          new_m[row][c+1] = b'.';
          start = (row, c);
        },
        _ => {}
      }
    }
  }
  (new_m, start)
}

#[aoc(day15, part2)]
pub fn part2(input: &str) -> usize {
  let (m, seq) = parse_input(input);
  let (mut m, mut pos) = expand(&m);

  let left = (0, -1);
  let right = (0, 1);

  let mut q = Vec::new();
  let mut seen = HashSet::with_hasher(Hash);
  let mut from = Vec::new();
  let mut to = Vec::new();

  let mut d ;
  let mut robot_next;
  let mut next;

  'outer: for ch in seq {
    q.clear();
    seen.clear();
    from.clear();
    to.clear();
    
    d = get_step(ch);
    robot_next = get_next_pos(pos, d);
    from.push(pos);
    to.push((robot_next, b'.'));
    q.push(robot_next);

    while let Some(cur) = q.pop() {
      match m[cur.0][cur.1] {
        b'[' => {
          next = get_next_pos(cur, d);
          if seen.insert(next) {
            q.push(next);
          }
          from.push(cur);
          to.push((next, b'['));
          next = get_next_pos(cur, right);
          if seen.insert(next) {
            q.push(next);
          }
        },
        b']' => {
          next = get_next_pos(cur, d);
          if seen.insert(next) {
            q.push(next);
          }
          from.push(cur);
          to.push((next, b']'));
          next = get_next_pos(cur, left);
          if seen.insert(next) {
            q.push(next);
          }
        },
        b'#' => {
          continue 'outer;
        },
        _ => {}
      }
    }
    pos = robot_next;
    for &(r, c) in &from {
      m[r][c] = b'.';
    }
    for &((r, c), ch) in &to {
      m[r][c] = ch;
    }
  }
  calc_score(&m, b'[')
}

#[cfg(test)]
mod tests {
  use super::*;

  const INPUT: &str = include_str!("../input/2024/day15.txt");
  const ANSWER1: &str = include_str!("../answer/2024/day15p1.txt");
  const ANSWER2: &str = include_str!("../answer/2024/day15p2.txt");

  const EXAMPLE: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

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
    assert_eq!(part1(EXAMPLE), 10092);
  }

  #[test]
  fn part2_example() {
    assert_eq!(part2(EXAMPLE), 9021);
  }
}