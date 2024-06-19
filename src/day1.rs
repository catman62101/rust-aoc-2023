use std::{collections::HashMap, fs::File, io::{prelude::*, BufReader}};

pub fn load_data(filepath: &str) -> Vec<String> {
  let file = match File::open(&filepath) {
    Err(why) => panic!("{}", why),
    Ok(file) => file,
  };

  let reader = BufReader::new(file);
  let mut lines = Vec::new();

  for line in reader.lines() {
    let line_str = line.unwrap();
    if line_str.len() > 0 {
      lines.push(line_str);
    }
  }

  lines
}

pub fn part1(lines: &Vec<String>) -> i32 {
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
    let mut digit1 = -1;
    let mut digit2 = -1;

    for i in 0..line.len() {
      match trie.search(line.clone(), i) {
        None => (),
        Some(value) => {
          digit2 = value;
          if digit1 == -1 {
            digit1 = digit2;
          }
        }
      }
    }

    calibration_sum += 10 * digit1 + digit2;
  }

  calibration_sum
}

pub fn part2(lines: &Vec<String>) -> i32 {
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

  let mut calibration_sum = 0;

  for line in lines {
    let mut digit1 = -1;
    let mut digit2 = -1;

    for i in 0..line.len() {
      match trie.search(line.clone(), i) {
        None => (),
        Some(value) => {
          digit2 = value;
          if digit1 == -1 {
            digit1 = digit2;
          }
        }
      }
    }

    calibration_sum += 10 * digit1 + digit2;
  }

  calibration_sum
}

#[derive(Debug)]
struct TrieNode {
  value: Option<i32>,
  children: HashMap<u8, Box<TrieNode>>
}

impl TrieNode {
  fn new() -> Self {
    Self{
      value: None,
      children: HashMap::new()
    }
  }

  fn insert(&mut self, word: String, value: i32) {
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

  fn search(&self, line: String, idx: usize) -> Option<i32> {
    let mut curr = self;

    for letter in line[idx..line.len()].as_bytes() {
      curr = match curr.children.get(letter) {
        Some(child) => child.as_ref(),
        None => return None
      };

      match curr.value {
        Some(val) => return Some(val),
        None => ()
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
      assert_eq!(trie.search(word.to_string(), 0).unwrap(), *val);
    }

    for (word, val) in word_to_num {
      println!("{word} {val}");
      println!("{}", format!("  {}  ", word.to_string()));
      assert_eq!(trie.search(format!("  {}  ", word.to_string()), 2).unwrap(), val);
    }
  }
}