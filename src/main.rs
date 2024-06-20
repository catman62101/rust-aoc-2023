use std::env;

pub mod common;
pub mod day1;
pub mod day2;
pub mod day3;

fn main() {
    let solutions = [
        || -> () {
            println!("============= DAY 1 =============");
            let lines = common::load_lines_from_file("inputs/day1.txt");
            println!("PART 1: {}", day1::part1(&lines));
            println!("PART 2: {}", day1::part2(&lines));
        },
        || -> () {
            println!("============= DAY 2 =============");
            let lines = common::load_lines_from_file("inputs/day2.txt");
            println!("PART 1: {}", day2::part1(&lines));
            println!("PART 2: {}", day2::part2(&lines));
        },
        || -> () {
            println!("============= DAY 3 =============");
            let grid: Vec<Vec<u8>> = common::load_lines_from_file("inputs/day3.txt")
                .iter()
                .map(|row| Vec::from(row.as_bytes()))
                .collect();
            println!("PART 1: {}", day3::part1(&grid));
            println!("PART 2: {}", day3::part2(&grid));
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

    for solution in solutions {
        solution();
        println!();
    }
}

