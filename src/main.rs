mod grid;
mod line_segment;
mod macros;
mod solution;
mod y2023;

use solution::*;

use clap::Parser;

#[derive(Parser)]
#[command(name = "advent-of-code-2023")]
#[command(about = "Advent of Code 2023 - By jortrr", long_about = None)]
struct Cli {
    /// Day to run, one of {1, ..., 25}
    #[arg(short, long)]
    day: Option<Day>,
}

type SolutionBox = Box<dyn Solution>;

fn main() {
    let instant = Instant::now();
    let cli = Cli::parse();

    let aoc_solutions: Vec<SolutionBox> = vec![
        y2023::d01::Problem::create_box(),
        y2023::d02::Problem::create_box(),
        y2023::d03::Problem::create_box(),
        y2023::d04::Problem::create_box(),
        y2023::d05::Problem::create_box(),
        y2023::d06::Problem::create_box(),
        y2023::d07::Problem::create_box(),
        y2023::d08::Problem::create_box(),
        y2023::d09::Problem::create_box(),
        y2023::d10::Problem::create_box(),
        y2023::d11::Problem::create_box(),
        y2023::d12::Problem::create_box(),
        y2023::d13::Problem::create_box(),
        y2023::d14::Problem::create_box(),
        y2023::d15::Problem::create_box(),
        y2023::d16::Problem::create_box(),
        y2023::d18::Problem::create_box(),
        y2023::d19::Problem::create_box(),
        y2023::d20::Problem::create_box(),
        y2023::d21::Problem::create_box(),
        y2023::d22::Problem::create_box(),
    ];

    let mut test_results: Vec<TestResult> = Vec::new();

    for (i, aoc_solution) in aoc_solutions.iter().enumerate() {
        if let Some(day) = cli.day {
            if day != aoc_solution.day() {
                continue;
            }
        }

        println!(
            "[{}/{}] Running AoC: {}-{:02}",
            i,
            aoc_solutions.len(),
            aoc_solution.year(),
            aoc_solution.day()
        );
        test_results.push(aoc_solution.run());
        println!();
    }

    dbg!(&test_results);
    println!(
        "Ran {} AoC solutions in {:.2?}.",
        test_results.len(),
        instant.elapsed()
    );

    assert!(!test_results.is_empty());
    let all_test_results_succeed = test_results.iter().all(|result| match result.p1 {
        TestStatus::Success(_, _) => true,
        _ => false,
    });
    assert!(all_test_results_succeed);
}
