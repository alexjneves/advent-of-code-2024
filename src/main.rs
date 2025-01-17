mod day;
mod day1;
mod day2;
mod day3;
mod day5;
mod day6;
mod day7;
mod day8;

use std::process::exit;

use clap::Parser;
use day::{Day, InputType, Part};
use day1::day1::Day1;
use day2::day2::Day2;
use day3::day3::Day3;
use day5::day5::Day5;
use day6::day6::Day6;
use day7::day7::Day7;
use day8::day8::Day8;

#[derive(Parser)]
struct Cli {
    day: String,
    part: u8,
    input: char
}

fn main() {
    let args: Cli = Cli::parse();

    let part = match args.part {
        1 => Part::One,
        2 => Part::Two,
        _ => exit(-1)
    };

    let input = match args.input {
        'e' => InputType::Example,
        'c' => InputType::Custom,
        _ => exit(-1)
    };

    let answer: Result<i32, String> = match args.day.as_str() {
        "day1" => Ok(Day1 {}.run(part, input)),
        "day2" => Ok(Day2 {}.run(part, input)),
        "day3" => Ok(Day3 {}.run(part, input)),
        "day5" => Ok(Day5 {}.run(part, input)),
        "day6" => Ok(Day6 {}.run(part, input)),
        "day7" => Ok(Day7 {}.run(part, input)),
        "day8" => Ok(Day8 {}.run(part, input)),
        _ => Err("Invalid day provided".to_owned())
    };

    match answer {
        Ok(answer) => println!("Result: {}", answer),
        Err(error)=> println!("Error: {}", error.as_str()) 
    }
}
