// https://adventofcode.com/2023/day/1

use std::collections::HashMap;

pub fn part1(lines: &Vec<String>) -> u16 {
  let word_to_num = vec![
    ("0", 0), ("1", 1), ("2", 2), ("3", 3), ("4", 4),
    ("5", 5), ("6", 6), ("7", 7), ("8", 8), ("9", 9),
  ];
  let mut trie = TrieNode::new();

  for (word, val) in word_to_num {
    trie.insert(word.to_string(), val)
  }

  let mut calibration_sum = 0;

  for line in lines {
    let mut digit1 = 0;
    let mut digit2 = 0;

    for i in 0..line.len() {
      if let Some(value) = trie.search(line.as_str(), i) {
        digit1 = value;
        break;
      }
    }

    for i in (0..line.len()).rev() {
      if let Some(value) = trie.search(line.as_str(), i) {
        digit2 = value;
        break;
      }
    }

    calibration_sum += 10 * digit1 + digit2;
  }

  calibration_sum
}

pub fn part2(lines: &Vec<String>) -> u16 {
  let word_to_num = vec![
    ("0", 0), ("1", 1), ("2", 2), ("3", 3), ("4", 4),
    ("5", 5), ("6", 6), ("7", 7), ("8", 8), ("9", 9),
    ("zero", 0), ("one", 1), ("two", 2),   ("three", 3), ("four", 4),
    ("five", 5), ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9),
  ];
  let mut trie = TrieNode::new();
  
  for (word, val) in word_to_num {
    trie.insert(word.to_string(), val)
  }

  let mut digit_sum1 = 0;
  let mut digit_sum2 = 0;

  for line in lines {
    let mut digit1 = 0;
    let mut digit2 = 0;

    for i in 0..(line.len()) {
      if let Some(value) = trie.search(line, i) {
        digit1 = value;
        break;
      }
    }

    for i in (0..(line.len())).rev() {
      if let Some(value) = trie.search(line, i) {
        digit2 = value;
        break;
      }
    }

    digit_sum1 += digit1;
    digit_sum2 += digit2;
  }

  digit_sum1 * 10 + digit_sum2
}

#[derive(Debug)]
struct TrieNode {
  value: Option<u16>,
  children: HashMap<u8, Box<TrieNode>>
}

impl TrieNode {
  fn new() -> Self {
    Self{
      value: None,
      children: HashMap::new()
    }
  }

  fn insert(&mut self, word: String, value: u16) {
    let mut curr = self;

    for letter in word.as_bytes() {
      if !curr.children.contains_key(letter) {
        curr.children.insert(*letter, Box::new(
          Self{
            value: None,
            children: HashMap::new(),
          }
        ));
      }
      
      curr = curr.children.get_mut(letter).unwrap();
    }

    curr.value = Some(value);
  }

  fn search(&self, line: &str, idx: usize) -> Option<u16> {
    let mut curr = self;

    for letter in line[idx..line.len()].as_bytes() {
      curr = match curr.children.get(letter) {
        Some(child) => child.as_ref(),
        None => return None
      };

      if let Some(val) = curr.value {
        return Some(val);
      }
    }

    None
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_trie() {
    let word_to_num = vec![
      ("0", 0), ("1", 1), ("2", 2), ("3", 3), ("4", 4),
      ("5", 5), ("6", 6), ("7", 7), ("8", 8), ("9", 9),
      ("zero", 0), ("one", 1), ("two", 2),   ("three", 3), ("four", 4),
      ("five", 5), ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9),
    ];
    let mut trie = TrieNode::new();

    for (word, val) in word_to_num.iter() {
      trie.insert(word.to_string(), *val)
    }

    for (word, val) in word_to_num.iter() {
      println!("{word} {val}");
      assert_eq!(trie.search(word, 0).unwrap(), *val);
    }

    for (word, val) in word_to_num {
      println!("{word} {val}");
      println!("  {}  ", word);
      assert_eq!(trie.search(&format!("  {}  ", word), 2).unwrap(), val);
    }
  }
}