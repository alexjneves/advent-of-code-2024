use crate::day::{read_day_input, Day, InputType, Part};

const DAY_ID: u8 = 0;

pub struct Day0 {}

impl Day for Day0 {
    fn run(&self, part: Part, input_type: InputType) -> i32 {
        let input = read_day_input(DAY_ID, &part, &input_type);

        match part {
            Part::One => part1(&input),
            Part::Two => part2(&input)
        }
    }
}

fn part1(input: &Vec<String>) -> i32 {
    0
}

fn part2(input: &Vec<String>) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day0_part1_example_input() {
        const EXPECTED_ANSWER: i32 = 0;

        let day0 = Day0 {};
        let answer = day0.run(Part::One, InputType::Example);

        assert!(answer == EXPECTED_ANSWER);
    }

    #[test]
    fn day0_part1_custom_input() {
        const EXPECTED_ANSWER: i32 = 0;

        let day0 = Day0 {};
        let answer = day0.run(Part::One,InputType::Custom);

        assert!(answer == EXPECTED_ANSWER);
    }

    #[test]
    fn day0_part2_example_input() {
        const EXPECTED_ANSWER: i32 = 0;

        let day0 = Day0 {};
        let answer = day0.run(Part::Two, InputType::Example);

        assert!(answer == EXPECTED_ANSWER);
    }

    #[test]
    fn day0_part2_custom_input() {
        const EXPECTED_ANSWER: i32 = 0;

        let day0 = Day0 {};
        let answer = day0.run(Part::Two,InputType::Custom);
        
        assert!(answer == EXPECTED_ANSWER);
    }
}