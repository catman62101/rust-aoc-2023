use std::{fs::File, io::{prelude::*, BufReader}};

pub fn load_lines_from_file(filepath: &str) -> Vec<String> {
  let file = match File::open(filepath) {
    Err(why) => panic!("{}", why),
    Ok(file) => file,
  };

  let reader = BufReader::new(file);
  let mut lines = Vec::new();

  for line in reader.lines() {
    let line_str = line.unwrap();
    if !line_str.is_empty() {
      lines.push(line_str);
    }
  }

  lines
}