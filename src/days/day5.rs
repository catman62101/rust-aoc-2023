// https://adventofcode.com/2023/day/5

#[derive(Clone, Copy, Debug)]
pub struct Translation {
  pub source: i64,
  pub destination: i64,
  pub range: i64
}

pub fn load_data(lines: Vec<String>, seeds: &mut Vec<i64>, maps: &mut [Vec<Translation>]) {
  let mut i = 1; // line index, start on the third line

  // get seeds
  let seed_strs = lines[0].split(": ").collect::<Vec<&str>>()[1].split(' ').collect::<Vec<&str>>();
  for seed_str in seed_strs {
    seeds.push(
      String::from_utf8(seed_str.as_bytes().to_vec()).unwrap().parse().unwrap()
    );
  }

  // get 7 maps, seed -> soil -> fertilizer -> water -> light -> temp -> humidity -> location
  let mut map_count = 0;
  while map_count < 7 {
    i += 1;

    while i < lines.len() && lines[i].as_bytes()[0].is_ascii_digit() {
      let nums: Vec<i64> = lines[i].split(' ')
        .collect::<Vec<&str>>().iter()
        .map(|s| s.to_string().parse().unwrap()).collect();

      maps[map_count].push(Translation{
        source: nums[1],
        destination: nums[0],
        range: nums[2],
      });

      i += 1;
    }

    map_count += 1;
  }
}

pub fn part1(seeds: &[i64], maps: &Vec<Vec<Translation>>) -> i64 {
  let mut data = seeds.to_vec();

  for map in maps {
    for val in data.iter_mut() {
      *val += match map.iter().find(|translation| translation.source <= *val && *val < translation.source + translation.range) {
        None => 0,
        Some(translation) => {
          translation.destination - translation.source
        }
      }
    }
  }

  *data.iter().min().unwrap()
}

pub fn part2(seeds: &[i64], maps: &[Vec<Translation>]) -> i64 {
  let mut location = 1;
  let mut maps = maps.to_vec();
  maps.reverse();

  for _ in 0..1_000_000_000 { // just in case
    let mut data = location;

    for map in &maps {
      data -= match map.iter().find(|translation| translation.destination <= data && data < translation.destination + translation.range) {
        None => 0,
        Some(translation) => translation.destination - translation.source
      }
    }

    for i in 0..(seeds.len() / 2) {
      if data >= seeds[2*i] && data < seeds[2*i] + seeds[2*i + 1] {
        return location;
      }
    }

    location += 1;
  }

  -1 // shouldn't get to this line unless something goes horribly wrong
}