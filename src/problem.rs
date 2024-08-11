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

pub type Answer = Int;
pub type Year = i32;
pub type Day = u32;
pub type Input = String;
pub type ExampleInput = &'static str;

/// Use the newtype pattern to implement `From` and `Into` for `Input` and `Vec<String>`. \
/// `InputLines` is only a wrapper for `Vec<String>`.
///
/// See: https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#using-the-newtype-pattern-to-implement-external-traits-on-external-types
pub struct InputLines(Vec<String>);
pub type Grid<T> = Vec<Vec<T>>;

impl InputLines {
    pub fn filter_empty_lines(self) -> InputLines {
        InputLines(self.0.into_iter().filter(|line| !line.is_empty()).collect())
    }
}

/// Make Input convertible to InputLines(Vec<String>) by lines()
impl From<Input> for InputLines {
    fn from(input: Input) -> Self {
        InputLines(input.lines().map(String::from).collect())
    }
}

/// Make InputLines convertible to Vec<String>
impl Into<Vec<String>> for InputLines {
    fn into(self) -> Vec<String> {
        self.0
    }
}

/// Make InputLines convertible to Grid<char>
impl Into<Grid<char>> for InputLines {
    fn into(self) -> Grid<char> {
        self.0.into_iter().map(|s| s.chars().collect()).collect()
    }
}

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
    const RUN_EXAMPLE: bool = true;

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
            .skip(1) // Skip first
            .take(Self::example_input().lines().count().saturating_sub(2)) // Skip last
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn part_one_example() -> Answer {
        let input = Self::trimmed_example_input();
        let solution = Self::solve_part_one(input, true);
        test!(
            Self::PART_ONE_EXAMPLE_EXPECTED,
            solution,
            "part_one_example"
        );
        solution
    }

    fn part_one() -> Answer {
        let input = aoc::get_string(Self::YEAR, Self::DAY);
        let solution = Self::solve_part_one(input, false);
        test!(Self::PART_ONE_EXPECTED, solution, "part_one");
        solution
    }

    fn part_two_example() -> Answer {
        let input = Self::trimmed_example_input();
        let solution = Self::solve_part_two(input, true);
        test!(
            Self::PART_TWO_EXAMPLE_EXPECTED,
            solution,
            "part_two_example"
        );
        solution
    }

    fn part_two() -> Answer {
        let input = aoc::get_string(Self::YEAR, Self::DAY);
        let solution = Self::solve_part_two(input, false);
        test!(Self::PART_TWO_EXPECTED, solution, "part_two");
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
            if <$problem>::RUN_EXAMPLE {
                benchmark_functions!(
                    <$problem>::part_one_example,
                    <$problem>::part_one,
                    <$problem>::part_two_example,
                    <$problem>::part_two
                );
            } else {
                benchmark_functions!(<$problem>::part_one, <$problem>::part_two);
            }
        }
    };
}

/// Trait to allow a type to be parsed from Problem Input
pub trait Parse {
    fn parse(input: Input) -> Self;
}

/// Parse a single number
pub fn parse_num(input: &str) -> IResult<&str, Int> {
    map_res(digit1, str::parse::<Int>)(input)
}
