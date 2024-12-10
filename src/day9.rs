


fn get_layout(input: &str) -> Vec<(usize, usize)> {
  let s = input.trim_end().as_bytes();
  let n = s.len();
  let mut layout = vec![(0, 0); n];

  let mut id = 0;
  for i in 0..n {
    layout[i].1 = (s[i] - b'0') as usize;
    if i % 2 == 0 {
      layout[i].0 = id;
      id += 1;
    } else {
      layout[i].0 = usize::MAX;
    }
  }
  layout
}

#[aoc(day9, part1, inserts)]
pub fn part1_inserts(input: &str) -> usize {
  let mut layout = get_layout(input);
  let mut l = 1;
  let mut r = layout.len() - 1;

  while l < r {
    let diff = layout[r].1.min(layout[l].1);
    layout[r].1 -= diff;
    layout[l].0 = layout[r].0;
    if layout[r].1 > 0 {
      l += 2;
    } else if layout[l].1 > diff {
      layout.insert(l+1, (usize::MAX, layout[l].1 - diff));
      layout[l].1 = diff;
      r -= 1;
      l += 1;
    } else {
      l += 2;
      r -= 2;
    }
  }

  let mut ans = 0;
  let mut pos = 0;
  for i in 0..=r {
    if layout[i].0 != usize::MAX {
      for _ in 0..layout[i].1 {
        ans += pos * layout[i].0;
        pos += 1;
      }
    }
  }
  ans
}


fn get_data(input: &str) -> Vec<usize> {
  let s = input.trim_end().as_bytes();
  let n = s.len();

  let total = s.iter().fold(0usize, |acc, e| acc + (*e-b'0') as usize );
  let mut data = vec![0; total];
  let mut pos = 0;
  let mut id = 0;
  let mut is_file = true;
  for i in 0..n {
    if is_file {
      for _ in 0..((s[i]-b'0') as usize) {
        data[pos] = id;
        pos += 1;
      }
      id += 1;
    } else {
      for _ in 0..((s[i]-b'0') as usize) {
        data[pos] = usize::MAX;
        pos += 1;
      }
    }
    is_file = !is_file;
  }
  data
}

#[aoc(day9, part1, decompress)]
pub fn part1(input: &str) -> usize {
  let data = get_data(input);
  let mut l = 0;
  let mut r = data.len() - 1;

  let mut ans = 0;
  while l <= r {
    if data[l] != usize::MAX {
      ans += data[l] * l;
      l += 1;
    } else if data[r] == usize::MAX {
      r -= 1;
    } else {
      ans += data[r] * l;
      l += 1;
      r -= 1;
    }
  }
  ans
}

fn find_next_slot(data: &Vec<usize>, l: usize, r: usize, target: usize) -> usize {
  let mut cur = 0;
  for i in l..=r {
    if data[i] == usize::MAX {
      cur += 1;
    } else {
      if cur == target {
        return i - target;
      }
      cur = 0;
    }
  }
  r
}

fn find_best_slot_id(slots: &Vec<usize>, min_size: usize) -> usize {
  let mut min_pos = usize::MAX;
  let mut min_slot = min_size;
  for size in min_size..=9 {
    if slots[size] < min_pos {
      min_pos = slots[size];
      min_slot = size;
    }
  }
  min_slot
}

#[aoc(day9, part2, decompress)]
pub fn part2(input: &str) -> usize {
  let layout = get_layout(input);
  let mut data = get_data(input);

  let mut slots = vec![0; 10];
  for i in 1..=9 {
    slots[i] = find_next_slot(&data, 0, data.len() - 1, i);
  }

  let mut pos = layout.len()-1;
  let mut r = data.len()-1;

  while pos > 0 {
    if layout[pos].0 != usize::MAX {
      let id = find_best_slot_id(&slots, layout[pos].1);
      if slots[id] < r {
        for i in 0..layout[pos].1 {
          data[slots[id] + i] = layout[pos].0;
        }
        for i in 0..layout[pos].1 {
          data[r-i] = usize::MAX;
        }
        let diff = id - layout[pos].1;
        if slots[diff] > slots[id] { // no action in case of diff == 0
          slots[diff] = slots[id]+layout[pos].1;
        }
        slots[id] = find_next_slot(&data, slots[id]+id, r, id);
      }
    }
    r -= layout[pos].1;
    pos -= 1;
  }

  let mut ans = 0;
  for i in 0..data.len() {
    if data[i] != usize::MAX {
      ans += data[i] * i;
    }
  }
  ans
}

#[cfg(test)]
mod tests {
  use super::*;

  const INPUT: &str = include_str!("../input/2024/day9.txt");
  const ANSWER1: &str = include_str!("../answer/2024/day9p1.txt");
  const ANSWER2: &str = include_str!("../answer/2024/day9p2.txt");

  const EXAMPLE: &str = "2333133121414131402";

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
    assert_eq!(part1(EXAMPLE), 1928);
  }

  #[test]
  fn part2_example() {
    assert_eq!(part2(EXAMPLE), 2858);
  }
}