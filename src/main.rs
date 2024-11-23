mod day;
mod day1;

use std::process::exit;

use clap::Parser;
use day::{Day, InputType, Part};
use day1::day1::Day1;

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
        _ => Err("Invalid day provided".to_owned())
    };

    match answer {
        Ok(answer) => println!("Result: {}", answer),
        Err(error)=> println!("Error: {}", error.as_str()) 
    }
}
