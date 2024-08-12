#![allow(unused_imports, dead_code)]
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
use std::iter;
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

impl InputLines {
    pub fn filter_empty_lines(self) -> InputLines {
        InputLines(self.0.into_iter().filter(|line| !line.is_empty()).collect())
    }
}

impl Debug for InputLines {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self.0)
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

/// Trim example_input, remove preceding spaces from all lines, remove first \n, keep empty lines intact
fn trim_example_input(input: ExampleInput) -> Input {
    input
        .lines()
        .map(|line| {
            if line.trim().is_empty() {
                line // Keep empty lines intact
            } else {
                line.trim_start() // Trim leading spaces from non-empty lines
            }
        })
        .skip(1) // Skip first
        .take(input.lines().count().saturating_sub(2)) // Skip last
        .collect::<Vec<_>>()
        .join("\n")
}

/// Trait for implementing an Advent of Code problem
pub trait Problem {
    // Advent of Code year and day, used to fetch AoC input
    const YEAR: Year;
    const DAY: Day;

    // Expected values for the inputs, will be tested
    const PART_ONE_EXPECTED: Answer;
    const PART_TWO_EXPECTED: Answer;

    /// Solve AoC(`YEAR`, `DAY`) part one
    fn solve_part_one(input: Input, is_example: bool) -> Answer;

    /// Solve AoC(`YEAR`, `DAY`) part two
    fn solve_part_two(input: Input, is_example: bool) -> Answer;

    /// Define Advent of Code examples
    fn define_examples() -> Vec<Example> {
        Vec::new()
    }

    fn part_one() -> Answer {
        let input = aoc::get(Self::YEAR, Self::DAY);
        let solution = Self::solve_part_one(input, false);
        test!(Self::PART_ONE_EXPECTED, solution, "part_one");
        solution
    }

    fn part_two() -> Answer {
        let input = aoc::get(Self::YEAR, Self::DAY);
        let solution = Self::solve_part_two(input, false);
        test!(Self::PART_TWO_EXPECTED, solution, "part_two");
        solution
    }

    /// Run all given examples
    fn run_examples() -> bool {
        static NAME_ONE: &str = "example_part_one()";
        static NAME_TWO: &str = "example_part_two()";
        let format = |part: &str, i: usize| {
            format!("{} [{}/{}]", part, i + 1, Self::define_examples().len(),)
        };

        for (i, example) in Self::define_examples().iter().enumerate() {
            let input = trim_example_input(example.input);
            match example.expect {
                Expect::PartOne(one) => {
                    test!(one, Self::solve_part_one(input, true), format(NAME_ONE, i));
                }
                Expect::PartTwo(two) => {
                    test!(two, Self::solve_part_two(input, true), format(NAME_TWO, i));
                }
                Expect::PartsOneAndTwo(one, two) => {
                    test!(
                        one,
                        Self::solve_part_one(input.clone(), true),
                        format(NAME_ONE, i)
                    );
                    test!(two, Self::solve_part_two(input, true), format(NAME_TWO, i));
                }
                Expect::Any => (),
            }
        }
        true
    }
}

/// Benchmark and run all parts of an Advent of Code problem
/// Also runs all examples specified inside the run_examples function or define_examples() macro
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
                <$problem>::run_examples,
                <$problem>::part_one,
                <$problem>::part_two
            );
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

/// Advent of Code ExampleInput expectation for Problem part one, part two, or both
pub enum Expect {
    PartOne(Answer),
    PartTwo(Answer),
    PartsOneAndTwo(Answer, Answer),
    Any,
}

/// Advent of Code ExampleInput and expectation
pub struct Example {
    pub input: ExampleInput,
    pub expect: Expect,
}

impl Example {
    pub fn get_input(&self) -> Input {
        trim_example_input(self.input)
    }
}

/// Define Advent of Code Examples
#[macro_export]
macro_rules! define_examples {
    (
        $(
            (
                $input:expr,
                $expect:expr,
            )
        ),* $(,)?
    ) => {
        fn define_examples() -> Vec<Example> {
            vec![
                $(
                    Example {
                        input: $input,
                        expect: $expect,
                    },
                )*
            ]
        }
    };
}
