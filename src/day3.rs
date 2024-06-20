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
  let directions = [
    (-1, 1), (0, 1), (1, 1), (-1, -1), (0, -1), (1, -1)
  ];
  let width = grid[0].len() as i32;
  let height = grid.len() as i32;

  let mut part_num_sum = 0;

  for y in 0..height {
    for x in 0..width {
      let mut top_index: Option<usize> = None;
      let mut bottom_index: Option<usize> = None;

      match grid[y as usize][x as usize] {
        b'*' => {
          for (sx, sy) in directions.iter() {
            let dx = x + sx;
            let dy = y + sy;

            if dx < 0 || dx >= width || dy < 0 || dy >= height {
              continue;
            }

            match grid[dy as usize][dx as usize] {
              b'0'..=b'9' => match *sy > 0 {
                true => bottom_index = Some(dx as usize),
                false => top_index = Some(dx as usize),
              },
              _ => ()
            }
          }
        },
        _ => ()
      };

      if top_index == None || bottom_index == None {
        continue;
      }

      let mut start = top_index.unwrap() as i32;
      let mut end = top_index.unwrap() as i32;

      while start >= 0 {
        match grid[(y-1) as usize][start as usize] {
          b'0'..=b'9' => (),
          _ => break
        };

        start -= 1;
      }
      start += 1;
      while end < width {
        match grid[(y-1) as usize][end as usize] {
          b'0'..=b'9' => (),
          _ => break
        };

        end += 1;
      }

      let num1: i32 = String::from_utf8(
        grid[(y-1) as usize][(start as usize)..(end as usize)].to_vec()
      ).unwrap().parse().unwrap();

      start = bottom_index.unwrap() as i32;
      end = bottom_index.unwrap() as i32;
      while start >= 0 {
        match grid[(y+1) as usize][start as usize] {
          b'0'..=b'9' => (),
          _ => break
        };

        start -= 1;
      }
      start += 1;
      while end < width {
        match grid[(y+1) as usize][end as usize] {
          b'0'..=b'9' => (),
          _ => break
        };

        end += 1;
      }

      // println!("{}", String::from_utf8(
      //   grid[(y+1) as usize][(start as usize)..(end as usize)].to_vec()
      // ).unwrap());
      let num2: i32 = String::from_utf8(
        grid[(y+1) as usize][(start as usize)..(end as usize)].to_vec()
      ).unwrap().parse().unwrap();
      // println!("{} {}", num1, num2);
      
      part_num_sum += num1 * num2;
    }
  }
  
  part_num_sum
}