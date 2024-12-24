use std::collections::{HashSet, HashMap};
use fasthash::sea::Hash64 as Hash;

const MAX_ELEMENTS: usize = 26 * 26;


fn parse_adj(input: &str) -> Vec<Vec<usize>> {
  let mut adj = vec![Vec::new(); MAX_ELEMENTS];
  for s in input.lines().map(|s| s.as_bytes()) {
    let a = (s[0]-b'a') as usize * 26 + (s[1]-b'a') as usize;
    let b = (s[3]-b'a') as usize * 26 + (s[4]-b'a') as usize;
    adj[a].push(b);
    adj[b].push(a);
  }

  for a in &mut adj {
    a.sort_unstable();
  }
  adj
}

#[inline(always)]
fn starts_with_t(id: usize) -> bool {
  (id / 26) == 19
}

#[aoc(day23, part1)]
pub fn part1(input: &str) -> usize {
  let adj = parse_adj(input);

  let mut ans = 0;
  for a in 0..MAX_ELEMENTS {
    let a_ok = starts_with_t(a);
    let n = adj[a].len();
    if n < 2 {
      continue;
    }
    for i in 0..n {
      if adj[a][i] < a {
        continue;
      }
      let b = adj[a][i];
      let b_ok = starts_with_t(b);

      for j in i+1..n {
        let c = adj[a][j];
        let c_ok = starts_with_t(c);
        if !a_ok && !b_ok && !c_ok {
          continue;
        }
        for &k in &adj[b] {
          if k == c {
            ans += 1;
            break;
          }
        }
      }
    }
  }
  ans
}

fn rec(adj: &Vec<Vec<usize>>, start: usize, stack: &mut Vec<usize>, seen: &mut HashSet<usize, Hash>) -> Vec<usize> {
  seen.insert(start);
  for &e in stack.iter() {
    if e != start {
      let mut has_e = false;
      for &i in &adj[start] {
        if i == e {
          has_e = true;
          break;
        } else if i > e {
          break;
        }
      }
      if !has_e {
        let mut copy = stack.clone();
        copy.pop();
        return copy;
      }
    }
  }

  let mut ans = stack.clone();
  for &i in &adj[start] {
    if seen.contains(&i) {
      continue;
    }
    stack.push(i);
    let cur = rec(adj, i, stack, seen);
    stack.pop();
    if cur.len() > ans.len() {
      ans = cur.clone();
    }
  }
  ans
}

#[aoc(day23, part2)]
pub fn part2(input: &str) -> String {
  let adj = parse_adj(input);

  let mut best = Vec::new();
  let mut cur;
  let mut stack = Vec::new();
  let mut seen = HashSet::with_hasher(Hash);

  for i in 0..MAX_ELEMENTS {
    if adj[i].len() < best.len() {
      continue;
    }
    cur = rec(&adj, i, &mut stack, &mut seen);
    if best.len() < cur.len() {
      best = cur.clone();
    }
    cur.clear();
    stack.clear();
    seen.clear();
  }

  best.sort_unstable();

  let mut ans = Vec::new();
  for num in best {
    ans.push((num / 26) as u8 + b'a');
    ans.push((num % 26) as u8 + b'a');
    ans.push(b',');
  }
  ans.pop();
  String::from_utf8(ans).unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;
  use indoc::indoc;

  const INPUT: &str = include_str!("../input/2024/day23.txt");
  const ANSWER1: &str = include_str!("../answer/2024/day23p1.txt");
  const ANSWER2: &str = include_str!("../answer/2024/day23p2.txt");

  const EXAMPLE: &str = indoc!("kh-tc
                                qp-kh
                                de-cg
                                ka-co
                                yn-aq
                                qp-ub
                                cg-tb
                                vc-aq
                                tb-ka
                                wh-tc
                                yn-cg
                                kh-ub
                                ta-co
                                de-co
                                tc-td
                                tb-wq
                                wh-td
                                ta-ka
                                td-qp
                                aq-cg
                                wq-ub
                                ub-vc
                                de-ta
                                wq-aq
                                wq-vc
                                wh-yn
                                ka-de
                                kh-ta
                                co-tc
                                wh-qp
                                tb-vc
                                td-yn");

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
    assert_eq!(part1(EXAMPLE), 7);
  }

  #[test]
  fn part2_example2() {
    assert_eq!(part2(EXAMPLE), "co,de,ka,ta");
  }
}