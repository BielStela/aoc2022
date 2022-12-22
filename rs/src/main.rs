use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = &args[1].trim().parse().unwrap();
    let data_path = format!("../data/day{:02}.txt", day);
    println!("Data path: {}", data_path);
    println!("Day: {}", day);
    let input = fs::read_to_string(&data_path);

    if let Ok(input) = input {
        let day_func = match day {
            1 => aoc2022::day01::run,
            2 => aoc2022::day02::run,
            3 => aoc2022::day03::run,
            _ => unreachable!(),
        };
        day_func(input.trim_end())
    } else {
        eprint!("No data found")
    }
}
