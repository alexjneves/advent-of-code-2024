use crate::day::{read_day_input, Day, InputType, Part};

const DAY_ID: u8 = 5;

pub struct Day5 {}

#[derive(Debug)]
struct PageOrderingRule {
    first: i32,
    second: i32
}

impl Day for Day5 {
    fn run(&self, part: Part, input_type: InputType) -> i32 {
        let input = read_day_input(DAY_ID, &part, &input_type);

        match part {
            Part::One => part1(&input),
            Part::Two => part2(&input)
        }
    }
}

fn part1(input: &Vec<String>) -> i32 {
    let (rules, pages) = parse_input(input);

    println!("{:?}", rules);
    println!("{:?}", pages);

    get_valid_pages(&rules, &pages).iter()
        .map(|page| get_middle_page_number(page))
        .sum()
}

fn part2(input: &Vec<String>) -> i32 {
    0
}

fn parse_input(input: &Vec<String>) -> (Vec<PageOrderingRule>, Vec<Vec<i32>>) {
    let mut page_ordering_rules: Vec<PageOrderingRule> = vec![];
    let mut pages: Vec<Vec<i32>> = vec![];

    let mut parse_pages = false;

    for line in input.iter() {
        if line.is_empty() {
            parse_pages = true;
            continue;
        }

        if parse_pages {
            pages.push(parse_page(line));
        } else {
            page_ordering_rules.push(parse_page_order_rule(line));
        }
    }

    (page_ordering_rules, pages)
}

fn parse_page_order_rule(input: &String) -> PageOrderingRule {
    let split: Vec<&str> = input.split('|').collect();
    let first = split.get(0).unwrap();
    let second = split.get(1).unwrap();

    PageOrderingRule {
        first: first.parse().unwrap(),
        second: second.parse().unwrap()
    }
}

fn parse_page(input: &String) -> Vec<i32> {
    input
        .split(",")
        .map(|n| n.parse::<i32>().unwrap())
        .collect()
}

fn get_valid_pages<'a>(rules: &Vec<PageOrderingRule>, pages: &'a Vec<Vec<i32>>) -> Vec<&'a Vec<i32>> {
    let mut valid_pages: Vec<&Vec<i32>> = vec![];

    for page in pages {
        let mut is_valid = true;

        for rule in rules {
            if let Some(first_index) = page.iter().position(|i| *i == rule.first) {
                if let Some(second_index) = page.iter().position(|i| *i == rule.second) {
                    if first_index > second_index {
                        is_valid = false;
                        break;
                    }
                }
            }
        }

        if is_valid {
            valid_pages.push(page);
        }
    }

    valid_pages
}

// fn get_middle_page_number<'a>(pages: &'a Vec<Vec<i32>>) -> Vec<i32> {
//     let mut middle_numbers: Vec<i32> = vec![];

//     for page in pages {
//         let middle_index = page.len() / 2;
//         middle_numbers.push(page.get(middle_index).unwrap().to_owned());
//     }

//     middle_numbers
// }

fn get_middle_page_number(page: &Vec<i32>) -> i32 {
    let middle_index = page.len() / 2;
    page.get(middle_index).unwrap().to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day5_part1_example_input() {
        const EXPECTED_ANSWER: i32 = 143;

        let day5 = Day5 {};
        let answer = day5.run(Part::One, InputType::Example);

        assert!(answer == EXPECTED_ANSWER);
    }

    #[test]
    fn day5_part1_custom_input() {
        const EXPECTED_ANSWER: i32 = 6949;

        let day5 = Day5 {};
        let answer = day5.run(Part::One,InputType::Custom);

        assert!(answer == EXPECTED_ANSWER);
    }

    #[test]
    fn day5_part2_example_input() {
        const EXPECTED_ANSWER: i32 = 0;

        let day5 = Day5 {};
        let answer = day5.run(Part::Two, InputType::Example);

        assert!(answer == EXPECTED_ANSWER);
    }

    #[test]
    fn day5_part2_custom_input() {
        const EXPECTED_ANSWER: i32 = 0;

        let day5 = Day5 {};
        let answer = day5.run(Part::Two,InputType::Custom);
        
        assert!(answer == EXPECTED_ANSWER);
    }
}