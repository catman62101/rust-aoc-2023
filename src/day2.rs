use regex::Regex;

pub fn part1(lines: &Vec<String>) -> i32 {
  let max_red = 12;
  let max_green = 13;
  let max_blue = 14;
  let mut id_sum = 0;
  let game_re = Regex::new(r"Game (\d+)").unwrap();
  let round_re = Regex::new(r"(\d+ \w+(?:, \d+ \w+)?+);?").unwrap();
  let red_re = Regex::new(r"(\w+) red").unwrap();
  let blue_re = Regex::new(r"(\w+) blue").unwrap();
  let green_re = Regex::new(r"(\w+) green").unwrap();

  for line in lines {
    let game_id: i32 = game_re
      .captures(line).unwrap().get(1).unwrap().as_str().parse().unwrap();

    let rounds = round_re.captures_iter(line)
      .filter_map(|m| m.get(1).map(|s| s.as_str().to_string()))
      .collect::<Vec<String>>();

    let mut game_valid = true;
    for round in rounds {
      let red_count = match red_re.captures(&round) {
        Some(cap) => cap[1].parse().unwrap(),
        None => 0
      };
      let blue_count = match blue_re.captures(&round) {
        Some(cap) => cap[1].parse().unwrap(),
        None => 0
      };
      let green_count = match green_re.captures(&round) {
        Some(cap) => cap[1].parse().unwrap(),
        None => 0
      };

      if red_count > max_red || blue_count > max_blue || green_count > max_green {
        game_valid = false;
        break;
      }
    }

    if game_valid {
      id_sum += game_id;
    }
  }

  id_sum
}

pub fn part2(lines: &Vec<String>) -> i32 {
  let mut power_sum = 0;
  let round_re = Regex::new(r"(\d+ \w+(?:, \d+ \w+)?+);?").unwrap();
  let red_re = Regex::new(r"(\w+) red").unwrap();
  let blue_re = Regex::new(r"(\w+) blue").unwrap();
  let green_re = Regex::new(r"(\w+) green").unwrap();

  for line in lines {
    let mut max_red = 0;
    let mut max_green = 0;
    let mut max_blue = 0;

    let rounds = round_re.captures_iter(line)
      .filter_map(|m| m.get(1).map(|s| s.as_str().to_string()))
      .collect::<Vec<String>>();

    for round in rounds {
      let red_count = match red_re.captures(&round) {
        Some(cap) => cap[1].parse().unwrap(),
        None => 0
      };
      let blue_count = match blue_re.captures(&round) {
        Some(cap) => cap[1].parse().unwrap(),
        None => 0
      };
      let green_count = match green_re.captures(&round) {
        Some(cap) => cap[1].parse().unwrap(),
        None => 0
      };

      max_red = i32::max(max_red, red_count);
      max_green = i32::max(max_green, green_count);
      max_blue = i32::max(max_blue, blue_count);
    }

    power_sum += max_red * max_green * max_blue;
  }

  power_sum
}