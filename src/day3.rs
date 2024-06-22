pub fn part1(grid: &Vec<Vec<u8>>) -> i32 {
  let directions = [-1, 0, 1, 0, -1, -1, 1, 1, -1];
  let width = grid[0].len() as i32;
  let height = grid.len() as i32;

  let mut number = Vec::<u8>::new();
  let mut is_part_num = false;
  let mut part_num_sum = 0;

  for y in 0..height {
    for x in 0..width {
      match grid[y as usize][x as usize] {
        b'0'..=b'9' => {
          number.push(grid[y as usize][x as usize]);

          for i in 0..directions.len()-1 {
            let dx = x as i32 + directions[i];
            let dy = y as i32 + directions[i+1];
    
            if dx < 0 || dx >= width || dy < 0 || dy >= height {
              continue;
            }

            match grid[dy as usize][dx as usize] {
              b'0'..=b'9'|b'.' => (),
              _ => is_part_num = true
            };
          }
        }
        _ => {
          if is_part_num {
            part_num_sum += String::from_utf8(number.clone())
              .unwrap().parse::<i32>().unwrap();
          }
          number.clear();
          is_part_num = false;
        }
      };
    }
  }
  
  part_num_sum
}

pub fn part2(grid: &Vec<Vec<u8>>) -> i32 {
  let height = grid.len();
  let width = grid[0].len();
  let mut gear_ratio_sum = 0;

  for y in 0..height {
    for x in 0..width {
      if grid[y][x] == b'*' {
        let mut numbers = Vec::<i32>::new();

        if y > 0 {
          numbers.append(&mut extract_numbers(&grid[y-1], x));
        }
        if y < height + 1 {
          numbers.append(&mut extract_numbers(&grid[y+1], x));
        }
        numbers.append(&mut extract_numbers(&grid[y], x));
        if numbers.len() != 2 { continue; }

        gear_ratio_sum += numbers[0] * numbers[1];
      }
    }
  }

  gear_ratio_sum
}

fn is_digit(cell: u8) -> bool {
  match cell {
    b'0'..=b'9' => true,
    _ => false
  }
}


fn extract_numbers(line: &Vec<u8>, i: usize) -> Vec<i32> {
  let mut numbers = Vec::<i32>::new();

  if is_digit(line[i]) {
    if let Some(num) = extract_number(line, i) {
      numbers.push(num);
    }
  } else {
    if i > 0 {
      if let Some(num) = extract_number(line, i-1) {
        numbers.push(num);
      }
    }
    if i < line.len()-1 {
      if let Some(num) = extract_number(line, i+1) {
        numbers.push(num);
      }
    }
  }

  numbers
}

fn extract_number(line: &Vec<u8>, i: usize) -> Option<i32> {
  let mut start = i as i32;
  let mut end = i as i32;

  while start >= 0 && is_digit(line[(start) as usize]) {
    start -= 1;
  }
  start += 1;
  while (end as usize) < line.len() && is_digit(line[end as usize]) {
    end += 1;
  }

  return match start < end {
    true => Some(
      String::from_utf8(line[(start as usize)..(end as usize)].to_vec()).unwrap()
        .parse().unwrap()
    ),
    false => None
  };
}