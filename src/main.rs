pub mod common;
pub mod day1;
pub mod day2;

fn main() {
    println!("============= DAY 1 =============");
    
    let lines = common::load_lines_from_file("inputs/day1.txt");
    println!("PART 1: {}", day1::part1(&lines));
    println!("PART 2: {}", day1::part2(&lines));
    
    println!("\n============= DAY 2 =============");
    
    let lines = common::load_lines_from_file("inputs/day2.txt");
    println!("PART 1: {}", day2::part1(&lines));
    println!("PART 2: {}", day2::part2(&lines));
}

