use colored::Colorize;

mod problem;
use problem::*;

type Int = i32;
type Grid<T> = Vec<Vec<T>>;

#[derive(Debug, Clone)]
enum Line {
    Row(Int),
    Column(Int),
}

impl Line {
    fn in_line(&self, row: Int, column: Int) -> bool {
        use Line::*;
        match &self {
            Row(k) => *k == row,
            Column(k) => *k == column,
        }
    }
}

#[derive(Clone)]
struct Pattern {
    grid: Grid<char>,
    rows: usize,
    columns: usize,
    smudges: Int,
    summary: Option<Int>,
    reflection: Option<Line>,
}

impl Debug for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut grid_string: String = String::new();
        let reflection = self.reflection.as_ref().unwrap();
        let summary = self.summary.as_ref().unwrap();
        for i in 0..self.rows {
            grid_string.push_str("  ");
            for j in 0..self.columns {
                let element = if reflection.in_line(i as Int, j as Int)
                    || reflection.in_line(i as Int - 1, j as Int - 1)
                {
                    self.grid[i][j].to_string().red()
                } else {
                    self.grid[i][j].to_string().normal()
                };

                grid_string.push_str(&element.to_string());
            }
            grid_string.push_str("\n");
        }

        write!(
            f,
            "\nPattern({}x{}): (smudges: {}, summary: {})\n{}",
            self.rows, self.columns, self.smudges, summary, grid_string
        )
    }
}

impl Pattern {
    fn parse(input: Input, smudges: Int) -> Vec<Pattern> {
        Pattern::parse_patterns(&input.lines().map(|s| s.to_string()).collect(), smudges)
    }

    /// Parse a single Pattern
    fn parse_pattern(input: Vec<String>, smudges: Int) -> Pattern {
        let rows = input.len();
        let columns = input.first().unwrap().len();
        let grid: Grid<char> = input.iter().map(|s| s.chars().collect()).collect();
        let mut result = Pattern {
            grid,
            rows,
            columns,
            smudges,
            summary: None,
            reflection: None,
        };
        result.reflection = result.find_reflection();
        result.summary = Some(result.summarize());

        result
    }

    /// Parse Patterns separated by empty lines
    fn parse_patterns(input: &Vec<String>, smudges: Int) -> Vec<Pattern> {
        let mut result: Vec<Pattern> = Vec::new();
        let mut current: Vec<String> = Vec::new();
        for line in input {
            if line.is_empty() {
                result.push(Pattern::parse_pattern(current.clone(), smudges));
                current.clear();
            } else {
                current.push(line.clone());
            }
        }
        if !current.is_empty() {
            result.push(Pattern::parse_pattern(current.clone(), smudges));
            current.clear();
        }

        result
    }

    fn compare(a: &Vec<char>, b: &Vec<char>) -> Int {
        assert!(a.len() == b.len());
        let mut differences = 0;
        for i in 0..a.len() {
            if a[i] != b[i] {
                differences += 1;
            }
        }
        differences
    }

    fn get_row(&self, index: usize) -> &Vec<char> {
        assert!(index < self.rows);
        &self.grid[index]
    }

    fn get_column(&self, index: usize) -> Vec<char> {
        assert!(index < self.columns);
        let mut result: Vec<char> = Vec::new();
        for i in 0..self.rows {
            result.push(self.grid[i][index]);
        }
        result
    }

    fn find_reflection(&self) -> Option<Line> {
        let columns: Grid<char> = (0..self.columns).map(|i| self.get_column(i)).collect();
        let mut result = None;

        // Check whether any 2 rows form a reflection, with exactly smudges amount of imperfections
        for i in 0..self.rows - 1 {
            let a = self.get_row(i);
            let b = self.get_row(i + 1);
            let mut diff = Pattern::compare(a, b);
            for j in 1..self.rows - 1 - i {
                if diff > self.smudges || i < j {
                    break;
                }
                let c = self.get_row(i - j);
                let d = self.get_row(i + 1 + j);
                diff += Pattern::compare(&c, &d);
            }
            if diff == self.smudges {
                result = Some(Line::Row(i as Int));
                break;
            }
        }

        if result.is_none() {
            // Check whether any 2 columns form a reflection, with exactly smudges amount of imperfections
            for i in 0..self.columns - 1 {
                let a = &columns[i];
                let b = &columns[i + 1];
                let mut diff = Pattern::compare(&a, &b);
                for j in 1..self.columns - 1 - i {
                    if diff > self.smudges || i < j {
                        break;
                    }
                    let c = &columns[i - j];
                    let d = &columns[i + 1 + j];
                    diff += Pattern::compare(&c, &d);
                }
                if diff == self.smudges {
                    result = Some(Line::Column(i as Int));
                    break;
                }
            }
        }

        result
    }

    fn summarize(&self) -> Int {
        let reflection = self.find_reflection();
        match reflection {
            Some(Line::Row(i)) => return (i + 1) * 100,
            Some(Line::Column(i)) => return i + 1,
            _ => panic!("No valid relection found: '{:?}'.", reflection),
        }
    }
}

struct DayThirteen {}

impl Problem for DayThirteen {
    const YEAR: Year = 2023;
    const DAY: Day = 13;
    const PART_ONE_EXAMPLE_EXPECTED: Answer = 405;
    const PART_ONE_EXPECTED: Answer = 30535;
    const PART_TWO_EXAMPLE_EXPECTED: Answer = 400;
    const PART_TWO_EXPECTED: Answer = 30844;

    fn example_input() -> ExampleInput {
        "
        #.##..##.
        ..#.##.#.
        ##......#
        ##......#
        ..#.##.#.
        ..##..##.
        #.#.##.#.

        #...##..#
        #....#..#
        ..##..###
        #####.##.
        #####.##.
        ..##..###
        #....#..#

        "
    }

    fn solve_part_one(input: Input, is_example: bool) -> Answer {
        debug!(is_example, &input);
        let part_1_patterns = Pattern::parse(input, 0);
        debug!(is_example, &part_1_patterns);
        let sum: Int = part_1_patterns.iter().map(|p| p.summary.unwrap()).sum();
        sum as Answer
    }

    fn solve_part_two(input: Input, _is_example: bool) -> Answer {
        let part_2_patterns = Pattern::parse(input, 1);
        let sum: Int = part_2_patterns.iter().map(|p| p.summary.unwrap()).sum();
        sum as Answer
    }
}

run!(DayThirteen);
