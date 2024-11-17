mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod grid;
mod line_segment;
mod macros;
mod problem;

use day01::DayOne;
use day02::DayTwo;
use day03::DayThree;
use day04::DayFour;
use day05::DayFive;
use day06::DaySix;
use day07::DaySeven;
use day08::DayEight;
use day09::DayNine;
use day10::DayTen;
use day11::DayEleven;
use day12::DayTwelve;
use day13::DayThirteen;
use day14::DayFourteen;
use day15::DayFifteen;
use day16::DaySixteen;
use day18::DayEighteen;
use day19::DayNineteen;
use day20::DayTwenty;
use day21::DayTwentyOne;
use day22::DayTwentyTwo;
use problem::*;

type ProblemPtr = Box<dyn Problem>;

fn main() {
    let problems: Vec<ProblemPtr> = problems!(
        DayOne,
        DayTwo,
        DayThree,
        DayFour,
        DayFive,
        DaySix,
        DaySeven,
        DayEight,
        DayNine,
        DayTen,
        DayEleven,
        DayTwelve,
        DayThirteen,
        DayFourteen,
        DayFifteen,
        DaySixteen,
        DayEighteen,
        DayNineteen,
        DayTwenty,
        DayTwentyOne,
        DayTwentyTwo
    );
    for (i, problem) in problems.iter().enumerate() {
        println!(
            "[{}/{}] Running AoC: {}-{:02}",
            i,
            problems.len(),
            problem.year(),
            problem.day()
        );
        problem.run();
        println!();
    }
}
