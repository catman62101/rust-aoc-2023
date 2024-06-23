use std::collections::HashSet;

pub fn part1(cards: &Vec<String>) -> i32 {
  let mut score = 0;

  for card in cards {
    let split = card.split(": ").collect::<Vec<&str>>()[1]
      .split(" | ").collect::<Vec<&str>>();

    let matches = count_matches(
      extract_numbers(split[0].as_bytes()),
      extract_numbers(split[1].as_bytes())
    );

    if matches > 0 {
      score += i32::pow(2, matches as u32 - 1)
    }
  }

  score
}

pub fn part2(cards: &Vec<String>) -> i32 {
  let mut card_counts = vec![1; cards.len()];
  let mut card_overflow = 0;

  for (i, card) in cards.iter().enumerate() {
    let split = card.split(": ").collect::<Vec<&str>>()[1]
      .split(" | ").collect::<Vec<&str>>();

    let matches = count_matches(
      extract_numbers(split[0].as_bytes()),
      extract_numbers(split[1].as_bytes())
    );

    for j in 0..matches {
      let idx = j as usize + i + 1;
      if idx >= card_counts.len() {
        card_overflow += (idx - card_counts.len() + 1) as i32;
        break;
      }
      card_counts[idx] += card_counts[i];
    }
  }
  
  card_overflow + card_counts.iter().sum::<i32>()
}

fn count_matches(numbers: Vec<i32>, winning: Vec<i32>) -> i32 {
  let winning_set = &HashSet::<i32>::from_iter(winning);
  let numbers_set = &HashSet::<i32>::from_iter(numbers);

  HashSet::intersection(
    winning_set,
    numbers_set
  ).collect::<HashSet<&i32>>().len() as i32
}

fn extract_numbers(line: &[u8]) -> Vec<i32> {
  let mut numbers = Vec::new();  
  let mut start = 0;
  let mut end = 0;

  while end < line.len() {
    if line[end as usize] == b' ' {
      if start < end {
        numbers.push(
          String::from_utf8(line[start..end].to_vec()).unwrap().parse().unwrap()
        );
      }
      start = end + 1;
    }
    end += 1;
  }

  if start < end {
    numbers.push(
      String::from_utf8(line[start..end].to_vec()).unwrap().parse().unwrap()
    );
  }

  numbers
}