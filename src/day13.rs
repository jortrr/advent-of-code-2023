use colored::Colorize;
use std::fmt::Debug;

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
    fn from_strings(input: Vec<String>, smudges: Int) -> Pattern {
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

    fn from_vec_strings(input: Vec<String>, smudges: Int) -> Vec<Pattern> {
        let mut result: Vec<Pattern> = Vec::new();
        let mut current: Vec<String> = Vec::new();
        for line in input {
            if line.is_empty() {
                result.push(Pattern::from_strings(current.clone(), smudges));
                current.clear();
            } else {
                current.push(line);
            }
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

fn test<T: std::cmp::PartialEq + std::fmt::Debug>(expected: T, actual: T) {
    dbg!(&actual);
    assert_eq!(
        expected, actual,
        "Test case failed: this value should always equal '{:?}'.",
        expected
    );
}

fn main() {
    // Part 1 - Example
    let example_input: Vec<String> = vec![
        "#.##..##.",
        "..#.##.#.",
        "##......#",
        "##......#",
        "..#.##.#.",
        "..##..##.",
        "#.#.##.#.",
        "",
        "#...##..#",
        "#....#..#",
        "..##..###",
        "#####.##.",
        "#####.##.",
        "..##..###",
        "#....#..#",
        "",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
    let mut example_patterns = Pattern::from_vec_strings(example_input.clone(), 0);
    //dbg!(&example_patterns);
    test(2, example_patterns.len());
    test(5, example_patterns[0].summary.unwrap());
    test(400, example_patterns[1].summary.unwrap());
    let sum: Int = example_patterns
        .iter_mut()
        .map(|p| p.summary.unwrap())
        .sum();
    test(405, sum);
    dbg!(example_patterns);

    #[rustfmt::skip]
    let test_a_input: Vec<String> = vec![
        "###..##",
        "##.#.#.",
        "..#...#",
        "##..#..",
        "#####.#",
        "###..#.",
        "###....",
    ].iter().map(|s|s.to_string()).collect();
    let test_a_patterns = Pattern::from_strings(test_a_input, 0);
    dbg!(test_a_patterns);

    // Part 2 - Example
    let example_patterns_with_smudge = Pattern::from_vec_strings(example_input, 1);
    test(300, example_patterns_with_smudge[0].summary.unwrap());
    test(100, example_patterns_with_smudge[1].summary.unwrap());

    // Part 1
    let part_1_patterns = Pattern::from_vec_strings(aoc_input::get(2023, 13), 0);
    //dbg!(&part_1_patterns);
    let sum = part_1_patterns.iter().map(|p| p.summary.unwrap()).sum();
    test(30535, sum);

    // Part 2
    let part_2_patterns = Pattern::from_vec_strings(aoc_input::get(2023, 13), 1);
    let sum: Int = part_2_patterns.iter().map(|p| p.summary.unwrap()).sum();
    test(30844, sum);
}
