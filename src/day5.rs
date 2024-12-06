const LIMIT: usize = 100;

fn get_rules_and_reports(input: &str) -> ([[bool; LIMIT]; LIMIT], Vec<Vec<usize>>) {
  let mut rules = [[false; LIMIT]; LIMIT];
  let mut reports = Vec::new();

  let mut parse_rule = true;
  for line in input.lines() {
    if line.len() == 0 {
      parse_rule = false;
      continue;
    }
    if parse_rule {
      let mut pair = line.trim().split('|')
                                                    .map(|d| d.parse::<usize>().unwrap());
      let left = pair.next().unwrap();
      let right = pair.next().unwrap();
      rules[left][right] = true;
    } else {
      reports.push(line.trim().split(',')
                       .map(|d| d.parse().unwrap())
                       .collect());
    }
  }
  (rules, reports)
}

#[aoc(day5, part1)]
pub fn part1(input: &str) -> usize {
  let (rules, reports) = get_rules_and_reports(input);
  let mut ans = 0;
  'outer: for report in reports {
    let n = report.len();
    for left in 0..n {
      for right in left+1..n {
        if rules[report[right]][report[left]] {
          continue 'outer;
        }
      }
    }
    ans += report[n / 2];
  }
  ans
}

fn get_middle_in_ordered(rules: &[[bool; LIMIT]; LIMIT], report: &Vec<usize>) -> usize {
  let n = report.len();
  let mut pos = vec![0; n];

  for left in 0..n {
    for right in left+1..n {
      if rules[report[left]][report[right]] {
        pos[right] += 1;
      } else {
        pos[left] += 1;
      }
    }
  }

  for i in 0..n {
    if pos[i] == n / 2 {
      return report[i];
    }
  }
  0
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> usize {
  let (rules, reports) = get_rules_and_reports(input);
  let mut ans = 0;
  'outer: for report in reports {
    let n = report.len();
    for left in 0..n {
      for right in left+1..n {
        if rules[report[right]][report[left]] {
          ans += get_middle_in_ordered(&rules, &report);
          continue 'outer;
        }
      }
    }
  }
  ans
}

