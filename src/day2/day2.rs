use crate::day::{Day, InputType, read_day_input, Part};

const DAY_ID: u8 = 2;

pub struct Day2 {}

impl Day for Day2 {
    fn run(&self, part: Part, input_type: InputType) -> i32 {
        let input = read_day_input(DAY_ID, &part, &input_type);

        match part {
            Part::One => part1(&input),
            Part::Two => part2(&input)
        }
    }
}

fn part1(input: &Vec<String>) -> i32 {
    input.iter()
        .map(parse_report)
        .map(|report| is_level_safe(report, false))
        .filter(|is_safe| *is_safe)
        .count() as i32
}

fn part2(input: &Vec<String>) -> i32 {
    input.iter()
        .map(parse_report)
        .map(|report| is_level_safe(report, true))
        .filter(|is_safe| *is_safe)
        .count() as i32
}

fn parse_report(input: &String) -> Vec<u32> {
    input.split_whitespace().map(|i| i.parse::<u32>().unwrap()).collect()
}

fn is_level_safe(level: Vec<u32>, problem_dampener: bool) -> bool {
    let mut asc: Option<bool> = Option::None;

    for window in level.windows(2) {
        if let [x, y] = window {
            if !is_diff_in_range(x, y) {
                return false;
            }

            if asc.is_none() {
                asc = Option::Some(x < y);
            }

            if asc.unwrap() {
                if x > y {
                    return false;
                }
            } else {
                if x < y {
                    return false;
                }
            }
        }
    }

    true
}

fn is_diff_in_range(x: &u32, y: &u32) -> bool {
    let diff = x.abs_diff(*y);
    diff >= 1 && diff <= 3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day2_part1_example_input() {
        const EXPECTED_ANSWER: i32 = 2;

        let day2 = Day2 {};
        let answer = day2.run(Part::One, InputType::Example);

        assert!(answer == EXPECTED_ANSWER);
    }

    #[test]
    fn day2_part1_custom_input() {
        const EXPECTED_ANSWER: i32 = 321;

        let day2 = Day2 {};
        let answer = day2.run(Part::One,InputType::Custom);

        assert!(answer == EXPECTED_ANSWER);
    }

    #[test]
    fn day2_part2_example_input() {
        const EXPECTED_ANSWER: i32 = 4;

        let day2 = Day2 {};
        let answer = day2.run(Part::Two, InputType::Example);

        assert!(answer == EXPECTED_ANSWER);
    }

    #[test]
    fn day2_part2_custom_input() {
        const EXPECTED_ANSWER: i32 = 0;

        let day2 = Day2 {};
        let answer = day2.run(Part::Two,InputType::Custom);

        assert!(answer == EXPECTED_ANSWER);
    }
}