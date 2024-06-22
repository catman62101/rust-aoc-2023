use std::{env, time::Instant};

pub mod common;
pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;

fn main() {
    let solutions = [
        || -> () {
            let lines = common::load_lines_from_file("inputs/day1.txt");

            let start = Instant::now();
            println!("PART 1: {} ({:?} elapsed)", day1::part1(&lines), start.elapsed());
            let start = Instant::now();
            println!("PART 2: {} ({:?} elapsed)", day1::part2(&lines), start.elapsed());
        },
        || -> () {
            let lines = common::load_lines_from_file("inputs/day2.txt");
            let start = Instant::now();
            println!("PART 1: {} ({:?} elapsed)", day2::part1(&lines), start.elapsed());
            let start = Instant::now();
            println!("PART 2: {} ({:?} elapsed)", day2::part2(&lines), start.elapsed());
        },
        || -> () {
            let grid: Vec<Vec<u8>> = common::load_lines_from_file("inputs/day3.txt")
                .iter()
                .map(|row| Vec::from(row.as_bytes()))
                .collect();
            let start = Instant::now();
            println!("PART 1: {} ({:?} elapsed)", day3::part1(&grid), start.elapsed());
            let start = Instant::now();
            println!("PART 2: {} ({:?} elapsed)", day3::part2(&grid), start.elapsed());
        },
        || -> () {
            let lines = common::load_lines_from_file("inputs/day4.txt");
            let start = Instant::now();
            println!("PART 1: {} ({:?} elapsed)", day4::part1(&lines), start.elapsed());
            let start = Instant::now();
            println!("PART 2: {} ({:?} elapsed)", day4::part2(&lines), start.elapsed());
        }
    ];

    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => (),
        3 => {
            if args[1] != "--day" {
                panic!("unknown argument provided");
            }

            let day = args[2].parse::<i32>().unwrap() - 1; 
            if day < 0 || day as usize >= solutions.len() {
                panic!("{}", format!("invalid day {day}, please pick a day between 1 and {}", solutions.len()))
            }

            solutions[day as usize]();
            return;
        },
        _ => panic!("invalid number of arguments provided")
    };

    for (i, solution) in solutions.iter().enumerate() {
        println!("============= DAY {} =============", i+1);
        solution();
        println!();
    }
}

