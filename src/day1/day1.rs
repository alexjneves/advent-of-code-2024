use crate::day::{Day, InputType, read_day_input, Part};

const DAY_ID: u8 = 1;

pub struct Day1 {}

impl Day for Day1 {
    fn run(&self, part: Part, input_type: InputType) -> i32 {
        // let input = read_day_input(DAY_ID, &part, &input_type);
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1_part1_example_input() {
        const EXPECTED_ANSWER: i32 = 0;

        let day1 = Day1 {};
        let answer = day1.run(Part::One, InputType::Example);

        assert!(answer == EXPECTED_ANSWER);
    }

    #[test]
    fn day1_part1_custom_input() {
        const EXPECTED_ANSWER: i32 = 0;

        let day1 = Day1 {};
        let answer = day1.run(Part::One,InputType::Custom);

        assert!(answer == EXPECTED_ANSWER);
    }

    #[test]
    fn day1_part2_example_input() {
        const EXPECTED_ANSWER: i32 = 0;

        let day1 = Day1 {};
        let answer = day1.run(Part::Two, InputType::Example);

        assert!(answer == EXPECTED_ANSWER);
    }

    #[test]
    fn day1_part2_custom_input() {
        const EXPECTED_ANSWER: i32 = 0;

        let day1 = Day1 {};
        let answer = day1.run(Part::Two,InputType::Custom);

        assert!(answer == EXPECTED_ANSWER);
    }
}