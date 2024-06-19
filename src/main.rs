pub mod day1;

fn main() {
    println!("============= DAY 1 =============");
    
    let lines = day1::load_data("inputs/day1.txt");

    println!("PART 1: {}", day1::part1(&lines));
    println!("PART 2: {}", day1::part2(&lines));

    println!("\n============= DAY 2 =============");
}

