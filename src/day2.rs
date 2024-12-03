
#[derive(PartialEq)]
enum Order {None, Decreasing, Increasing, Any}

pub fn input_generator(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|l| {
            l.trim().split(' ').map(|d| d.parse().unwrap())
            .collect()
        }).collect()
}


fn get_pair_order(left: i32, right: i32) -> Order {
  let diff = right - left;
  let abs = diff.abs();
  if abs < 1 || abs > 3 {
    Order::None
  } else if diff < 0 {
    Order::Decreasing
  } else {
    Order::Increasing
  }
}

fn get_list_order(list: &[i32]) -> Order {
  let mut order = Order::Any;
  for i in 1..list.len() {
    let new_order = get_pair_order(list[i-1], list[i]);
    if order == Order::Any {
      order = new_order;
    } else if order != new_order {
      order = Order::None;
    }
    if order == Order::None {
      break;
    }
  }
  order
}


#[aoc(day2, part1)]
pub fn part1(input: &str) -> u32 {
  let input = input_generator(input);
  let mut ans = 0;
  for list in input {
    if get_list_order(&list) != Order::None {
      ans += 1;
    }
  }
  ans
}

fn is_ordered_without_pos(list: &[i32], pos: usize) -> bool {
  if pos == 0 {
    return get_list_order(&list[1..]) != Order::None;
  } else if pos == list.len()-1 {
    return true;
  }
  let order1 = get_list_order(&list[..pos]);
  let order2 = get_list_order(&list[pos+1..]);
  let pair_order = get_pair_order(list[pos-1], list[pos+1]);

  if order1 == Order::None || order2 == Order::None || pair_order == Order::None {
    return false;
  }

  if order1 != Order::Any && order1 != pair_order {
    return false;
  }

  if order2 != Order::Any && order2 != pair_order {
    return false;
  }

  return true;
}


fn get_list_order2(list: &[i32]) -> Order {
  let mut order = Order::Any;
  let mut pos = 0;
  for i in 1..list.len() {
    let new_order = get_pair_order(list[i-1], list[i]);
    if order == Order::Any {
      order = new_order;
    } else if order != new_order {
      order = Order::None;
    }
    if order == Order::None {
      pos = i;
      break;
    }
  }
  if order != Order::None {
    return order;
  }

  if is_ordered_without_pos(list, pos) ||
     is_ordered_without_pos(list, pos-1) ||
     (pos == 2 && is_ordered_without_pos(list, 0)) {
    // println!("{:?}, {}", list, pos);
    return Order::Any;
  }
  Order::None
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> u32 {
  let input = input_generator(input);
  let mut ans = 0;
  for list in input {
    if get_list_order2(&list) != Order::None {
      ans += 1;
    }
  }
  ans
}