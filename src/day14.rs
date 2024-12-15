const ROWS: usize = 103;
const COLS: usize = 101;
const MAGIC: usize = 51; // (1 / COLS) mod ROWS

fn get_robots(input: &str) -> Vec::<(i32, i32, i32, i32)> {
  let mut robots = Vec::new();
  for line in input.trim().lines() {
    let mut line = line.split(' ');
    let pos = &mut line.next().unwrap()[2..].split(',').map(|e| e.parse().unwrap());
    let vel = &mut line.next().unwrap()[2..].split(',').map(|e| e.parse().unwrap());
    robots.push((pos.next().unwrap(), pos.next().unwrap(), vel.next().unwrap(), vel.next().unwrap()));
  }
  robots
}

fn solve_part1(input: &str, rows: usize, cols: usize, turns: usize) -> usize {
  let rows = rows as i32;
  let cols = cols as i32;
  let turns = turns as i32;
  let mid_r = rows / 2;
  let mid_c = cols / 2;
  let robots = get_robots(input);
  let mut quadrants = [[0; 2]; 2];
  for mut r in robots {
    r.0 += r.2 * turns;
    r.0 %= cols;
    if r.0 < 0 {r.0 += cols};
    if r.0 == mid_c {
      continue;
    }
    r.1 += r.3 * turns;
    r.1 %= rows;
    if r.1 < 0 {r.1 += rows};
    if r.1 == mid_r {
      continue;
    }
    quadrants[if r.0 < mid_c {0} else {1}][if r.1 < mid_r {0} else {1}] += 1;
  }
  // println!("{:?}", quadrants);
  quadrants[0][0] * quadrants[0][1] * quadrants[1][0] * quadrants[1][1]
}

#[aoc(day14, part1)]
pub fn part1(input: &str) -> usize {
  solve_part1(input, ROWS, COLS, 100)
}

#[aoc(day14, part2)]
pub fn part2(input: &str) -> usize {
  let mut robots = get_robots(input);
  let n = robots.len();
  let mut mean_x;
  let mut mean_y;
  let mut variance_x;
  let mut min_variance_x = i32::MAX;
  let mut min_t_x = 0;
  let mut min_t_y = 0;
  let mut variance_y;
  let mut min_variance_y = i32::MAX;
  let mut values_x = vec![0; n];
  let mut values_y = vec![0; n]; 

  let rows = ROWS as i32;
  let cols = COLS as i32;

  for t in 1..=COLS {
    for i in 0..n {
      robots[i].0 += robots[i].2;
      robots[i].0 %= cols;
      if robots[i].0 < 0 {robots[i].0 += cols};

      values_x[i] = robots[i].0;
    }
    mean_x = values_x.iter().sum::<i32>() / (n as i32);
    variance_x = values_x.iter().fold(0, |acc, e| acc + (mean_x - *e).pow(2));
    if variance_x < min_variance_x {
      min_variance_x = variance_x;
      min_t_x = t;
    }
  }

  for t in 1..=ROWS {
    for i in 0..n {
      robots[i].1 += robots[i].3;
      robots[i].1 %= rows;
      if robots[i].1 < 0 {robots[i].1 += rows};

      values_y[i] = robots[i].1;
    }
    mean_y = values_y.iter().sum::<i32>() / (n as i32);
    variance_y = values_y.iter().fold(0, |acc, e| acc + (mean_y - *e).pow(2));
    if variance_y < min_variance_y {
      min_variance_y = variance_y;
      min_t_y = t;
    }
  }
  // println!("{} {}", min_t_x, min_t_y);
  min_t_x + ((MAGIC * (ROWS + min_t_y - min_t_x)) % ROWS) * COLS
}

#[cfg(test)]
mod tests {
  use super::*;

  const INPUT: &str = include_str!("../input/2024/day14.txt");
  const ANSWER1: &str = include_str!("../answer/2024/day14p1.txt");
  const ANSWER2: &str = include_str!("../answer/2024/day14p2.txt");

  const EXAMPLE: &str = concat!("p=0,4 v=3,-3\n",
                                "p=6,3 v=-1,-3\n",
                                "p=10,3 v=-1,2\n",
                                "p=2,0 v=2,-1\n",
                                "p=0,0 v=1,3\n",
                                "p=3,0 v=-2,-2\n",
                                "p=7,6 v=-1,-3\n",
                                "p=3,0 v=-1,-2\n",
                                "p=9,3 v=2,3\n",
                                "p=7,3 v=-1,2\n",
                                "p=2,4 v=2,-3\n",
                                "p=9,5 v=-3,-3");

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
    assert_eq!(solve_part1(EXAMPLE, 11, 7, 100), 12);
  }
}