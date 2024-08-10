pub use aoc::*;

pub use nom::branch::alt;
pub use nom::bytes::complete::tag;
pub use nom::character::complete::{alpha1, digit1, one_of};
pub use nom::combinator::{map, map_res};
pub use nom::multi::separated_list1;
pub use nom::sequence::{preceded, terminated, tuple};
pub use nom::IResult;

pub use std::collections::HashMap;
pub use std::fmt::Debug;
pub use std::iter::once;
pub use std::time::Instant;

/// Default Integer type
pub type Int = i64;
pub type Answer = Int;
pub type Year = i32;
pub type Day = u32;
pub type Input = String;
pub type ExampleInput = &'static str;

/// Trait for implementing an Advent of Code problem
pub trait Problem {
    // Advent of Code year and day, used to fetch AoC input
    const YEAR: Year;
    const DAY: Day;

    // Expected values for the inputs, will be tested
    const PART_ONE_EXAMPLE_EXPECTED: Answer;
    const PART_ONE_EXPECTED: Answer;
    const PART_TWO_EXAMPLE_EXPECTED: Answer;
    const PART_TWO_EXPECTED: Answer;

    /// Solve AoC(`YEAR`, `DAY`) part one
    fn solve_part_one(input: Input, is_example: bool) -> Answer;

    /// Solve AoC(`YEAR`, `DAY`) part two
    fn solve_part_two(input: Input, is_example: bool) -> Answer;

    /// The Advent of Code example input
    fn example_input() -> ExampleInput;

    /// Trim example_input, remove preceding spaces from all lines, remove first \n, keep empty lines intact
    fn trimmed_example_input() -> Input {
        Self::example_input()
            .lines()
            .map(|line| {
                if line.trim().is_empty() {
                    line // Keep empty lines intact
                } else {
                    line.trim_start() // Trim leading spaces from non-empty lines
                }
            })
            .collect::<Vec<_>>()[1..]
            .join("\n")
    }

    fn part_one_example() -> Answer {
        let input = Self::trimmed_example_input();
        let solution = Self::solve_part_one(input, true);
        test!(Self::PART_ONE_EXAMPLE_EXPECTED, solution);
        solution
    }

    fn part_one() -> Answer {
        let input = aoc::get_string(Self::YEAR, Self::DAY);
        let solution = Self::solve_part_one(input, false);
        test!(Self::PART_ONE_EXPECTED, solution);
        solution
    }

    fn part_two_example() -> Answer {
        let input = Self::trimmed_example_input();
        let solution = Self::solve_part_two(input, true);
        test!(Self::PART_TWO_EXAMPLE_EXPECTED, solution);
        solution
    }

    fn part_two() -> Answer {
        let input = aoc::get_string(Self::YEAR, Self::DAY);
        let solution = Self::solve_part_two(input, false);
        test!(Self::PART_TWO_EXPECTED, solution);
        solution
    }
}

/// Benchmark and run all parts of an Advent of Code problem
#[macro_export]
macro_rules! run {
    ($problem:ty) => {
        fn main() {
            // Ensure that the type implements the Problem trait
            fn assert_impl_problem<T: Problem>() {}

            // Assert at compile-time that $problem implements Problem
            assert_impl_problem::<$problem>();

            // Use the benchmark_functions macro to benchmark all parts
            benchmark_functions!(
                <$problem>::part_one_example,
                <$problem>::part_one,
                <$problem>::part_two_example,
                <$problem>::part_two
            );
        }
    };
}
