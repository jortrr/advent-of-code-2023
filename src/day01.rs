use crate::*;

fn get_calibration_value(input: &String) -> Int {
    let digits: Vec<_> = input.chars().filter(|c| c.is_digit(10)).collect();
    format!("{}{}", digits.first().unwrap(), digits.last().unwrap())
        .parse::<Int>()
        .unwrap()
}

pub struct DayOne {}

impl Problem for DayOne {
    fn year(&self) -> Year {
        2023
    }

    fn day(&self) -> Day {
        1
    }

    fn expect_part_one(&self) -> Answer {
        55386
    }

    fn expect_part_two(&self) -> Answer {
        54824
    }

    fn solve_part_one(&self, input: Input, _is_example: bool) -> Answer {
        let input: Vec<String> = InputLines::from(input).filter_empty_lines().into();
        let solution = input.iter().map(|line| get_calibration_value(line)).sum();
        solution
    }

    fn solve_part_two(&self, input: Input, _is_example: bool) -> Answer {
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
