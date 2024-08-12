mod problem;
use problem::*;

fn get_calibration_value(input: &String) -> Int {
    let digits: Vec<_> = input.chars().filter(|c| c.is_digit(10)).collect();
    format!("{}{}", digits.first().unwrap(), digits.last().unwrap())
        .parse::<Int>()
        .unwrap()
}

struct DayOne {}

impl Problem for DayOne {
    const YEAR: Year = 2023;
    const DAY: Day = 1;
    const PART_ONE_EXPECTED: Answer = 55386;
    const PART_TWO_EXPECTED: Answer = 54824;

    fn solve_part_one(input: Input, _is_example: bool) -> Answer {
        let input: Vec<String> = InputLines::from(input).filter_empty_lines().into();
        let solution = input.iter().map(|line| get_calibration_value(line)).sum();
        solution
    }

    fn solve_part_two(input: Input, _is_example: bool) -> Answer {
        let input: Vec<String> = InputLines::from(input).filter_empty_lines().into();
        let solution = input
            .iter()
            .map(|line| {
                line.replace("one", "o1e")
                    .replace("two", "t2o")
                    .replace("three", "t3e")
                    .replace("four", "f4r")
                    .replace("five", "f5e")
                    .replace("six", "s6x")
                    .replace("seven", "s7n")
                    .replace("eight", "e8t")
                    .replace("nine", "n9e")
            })
            .map(|line| get_calibration_value(&line))
            .sum();
        solution
    }
}

run!(DayOne);
