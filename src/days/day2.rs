pub fn part1(lines: &Vec<String>) -> i32 {
  let max_red = 12;
  let max_green = 13;
  let max_blue = 14;
  let mut id_sum = 0;

  for line in lines {
    let mut game_id = 0;
    let mut color_counts = Vec::<(i32, i32, i32)>::new();

    extract_info(line, &mut game_id, &mut color_counts);

    let mut valid_game = true;
    for (r, g, b) in color_counts {
      if r > max_red {
        valid_game = false;
        break;
      }
      if g > max_green {
        valid_game = false;
        break;
      }
      if b > max_blue {
        valid_game = false;
        break;
      }
    }

    if valid_game {
      id_sum += game_id;
    }
  }

  id_sum
}

pub fn part2(lines: &Vec<String>) -> i32 {
  let mut power_sum = 0;

  for line in lines {
    let mut max_red = 0;
    let mut max_green = 0;
    let mut max_blue = 0;

    let mut game_id = 0;
    let mut color_counts = Vec::<(i32, i32, i32)>::new();
    extract_info(line, &mut game_id, &mut color_counts);

    for (r, g, b) in color_counts {
      max_red = i32::max(max_red, r);
      max_green = i32::max(max_green, g);
      max_blue = i32::max(max_blue, b);
    }

    power_sum += max_red * max_green * max_blue;
  }

  power_sum
}

fn extract_info(line: &String, case_id: &mut i32, rounds: &mut Vec<(i32, i32, i32)>) {
  let split = line.split(": ").collect::<Vec<&str>>();
  *case_id = split[0].split(" ").collect::<Vec<&str>>()[1].parse().unwrap();

  let rounds_strings = split[1].split("; ").collect::<Vec<&str>>();
  *rounds = rounds_strings.iter().map(|round| {
    let colors = round.split(", ").collect::<Vec<&str>>();
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;

    for color in colors {
      let color_split: Vec<&str> = color.split(" ").collect();

      match color_split[1] {
        "red" => { red = color_split[0].parse().unwrap(); },
        "green" => { green = color_split[0].parse().unwrap(); },
        "blue" => { blue = color_split[0].parse().unwrap(); },
        _ => ()
      }
    }

    (red, green, blue)
  }).collect();
}