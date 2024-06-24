// https://adventofcode.com/2023/day/6

pub fn part1(times: &[i32], distances: &[i32]) -> i32 {
  let mut ways_product = 1;
  
  for i in 0..times.len() {
    let mut l = 1;
    let mut r = times[i]-1;

    while l <= r {
      let m = (l+r) / 2;
      let my_distance = (times[i]-m) * m;

      if my_distance < distances[i] {
        l = m + 1;
      } else {
        r = m - 1;
      }
    }

    ways_product *= times[i] - 2*l + 1;
  }

  ways_product
}

pub fn part2(times: &[i32], distances: &[i32]) -> i64 {
  let mut time: i64 = 0;
  let mut distance: i64 = 0;

  for t in times {
    let mut power = 1;
    while power < *t {
      power *= 10;
      time *= 10;
    }

    time += *t as i64;
  }

  for d in distances {
    let mut power = 1;
    while power < *d {
      power *= 10;
      distance *= 10;
    }

    distance += *d as i64;
  }

  let mut l = 1;
  let mut r = time-1;

  while l <= r {
    let m = (l+r) / 2;
    let my_distance = (time-m) * m;

    if my_distance < distance {
      l = m + 1;
    } else {
      r = m - 1;
    }
  }

  time - 2*l + 1
}

pub fn load_data(lines: Vec<String>, times: &mut Vec<i32>, distances: &mut Vec<i32>) {
  let mut start = 0;
  // skip to first digit
  while !lines[0].as_bytes()[start].is_ascii_digit() {
    start += 1;
  }

  for end in start..lines[0].len() {
    if lines[0].as_bytes()[end] == b' ' {
      if start < end {
        times.push(
          String::from_utf8(lines[0].as_bytes()[start..end].to_vec()).unwrap().parse().unwrap()
        );
      }
      start = end + 1;
    }
  }
  if start < lines[0].len() {
    times.push(
      String::from_utf8(lines[0].as_bytes()[start..lines[0].len()].to_vec()).unwrap().parse().unwrap()
    );
  }
  
  start = 0;
  while !lines[1].as_bytes()[start].is_ascii_digit() {
    start += 1;
  }
  for end in start..lines[1].len() {
    if lines[1].as_bytes()[end] == b' ' {
      if start < end {
        distances.push(
          String::from_utf8(lines[1].as_bytes()[start..end].to_vec()).unwrap().parse().unwrap()
        );
      }
      start = end + 1;
    }
  }
  if start < lines[1].len() {
    distances.push(
      String::from_utf8(lines[1].as_bytes()[start..lines[1].len()].to_vec()).unwrap().parse().unwrap()
    );
  }
}