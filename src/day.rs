use std::fs;

pub trait Day {
    fn run(&self, part: Part, input: InputType) -> i32;
}

pub enum Part {
    One,
    Two,
}

pub enum InputType {
    Example,
    Custom,
}

pub fn read_day_input(day: u8, part: &Part, input_type: &InputType) -> Vec<String> {
    let contents = read_day_input_string(day, part, input_type);

    contents
        .split('\n')
        .map(|line| line.to_owned())
        .collect()
}

pub fn read_day_input_string(day: u8, part: &Part, input_type: &InputType) -> String {
    let path = match input_type {
        InputType::Example => format!(
            "src/day{}/files/part{}_example_input.txt",
            day,
            part_to_int(part)
        ),
        InputType::Custom => format!(
            "src/day{}/files/custom_input.txt",
            day,
        )
    };

    fs::read_to_string(path).unwrap()
}

fn part_to_int(part: &Part) -> i32 {
    match part {
        Part::One => 1,
        Part::Two => 2,
    }
}
