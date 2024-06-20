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
    // top
    (-1, 1), (0, 1), (1, 1),
    // bottom
    (-1, -1), (0, -1), (-1, 1),
    // mid
    (-1, 0), (1, 0)
  ];
  let width = grid[0].len() as i32;
  let height = grid.len() as i32;

  let mut part_num_sum = 0;

  for y in 0..height {
    for x in 0..width {
      let mut top_index1: Option<usize> = None;
      let mut top_index2: Option<usize> = None;
      let mut bottom_index1: Option<usize> = None;
      let mut bottom_index2: Option<usize> = None;
      let mut indices = Vec::<(usize, usize)>::new();

      match grid[y as usize][x as usize] {
        b'*' => {
          for (sx, sy) in directions.iter() {
            let dx = x + sx;
            let dy = y + sy;

            if dx < 0 || dx >= width || dy < 0 || dy >= height {
              continue;
            }

            match grid[dy as usize][dx as usize] {
              b'0'..=b'9' => match *sy {
                 1 => {
                  if let Some(idx) = bottom_index1 {
                    if idx as i32 == dx-1  {
                      bottom_index1 = Some(dx as usize);
                    } else {
                      bottom_index2 = Some(dx as usize);
                    }
                  } else {
                    bottom_index1 = Some(dx as usize);
                  }
                },
                -1 => {
                  if let Some(idx) = top_index1 {
                    if idx as i32 == dx-1  {
                      top_index1 = Some(dx as usize);
                    } else {
                      top_index2 = Some(dx as usize);
                    }
                  } else {
                    top_index1 = Some(dx as usize);
                  }
                },
                 0 => indices.push((dx as usize, dy as usize)),
                _ => ()
              },
              _ => ()
            }
          }
        },
        _ => ()
      };

      if let Some(idx) = top_index1 {
        indices.push((idx, (y-1) as usize));
      }
      if let Some(idx) = top_index2 {
        indices.push((idx, (y-1) as usize));
      }
      if let Some(idx) = bottom_index1 {
        indices.push((idx, (y+1) as usize));
      }
      if let Some(idx) = bottom_index2 {
        indices.push((idx, (y+1) as usize));
      }
      
      
      if indices.len() != 2 {
        continue;
      }

      // println!("{:?}", indices);

      let mut gear_ratio = 1;
      
      for (num_x, num_y) in indices {
        let mut start = num_x as i32;
        let mut end = num_x;
  
        while start >= 0 {
          match grid[num_y][start as usize] {
            b'0'..=b'9' => (),
            _ => break
          };
          start -= 1;
        }
        start += 1;
        while end < width as usize {
          match grid[num_y][end] {
            b'0'..=b'9' => (),
            _ => break
          };
          end += 1;
        }

        let start = start as usize;
        let num: i32 = String::from_utf8(grid[num_y][start..end].to_vec()).unwrap().parse().unwrap();
        // println!("{}", num);
        gear_ratio *= num;
      }

      part_num_sum += gear_ratio;
    }
  }
  
  part_num_sum
}