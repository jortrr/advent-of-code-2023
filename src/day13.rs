use core::num;

type Mirrors = (usize, usize);
type Line = String;
type Lines = Vec<Line>;
type Int = i32;

#[derive(Debug)]
struct Pattern {
    rows: Lines,
    columns: Lines,
    number_of_rows: usize,
    number_of_columns: usize,
}

impl Pattern {
    fn from_strings(input: Vec<String>) -> Pattern {
        let number_of_rows = input.len();
        let number_of_columns = input.first().unwrap().len();
        let columns: Lines = (0..number_of_columns)
            .map(|i| {
                input
                    .iter()
                    .map(move |r| r.chars().nth(i as usize).unwrap().clone())
                    .collect::<String>()
            })
            .collect();
        assert_eq!(
            number_of_columns,
            columns.len(),
            "Not enough columns in colomns Vec."
        );
        assert_eq!(
            number_of_rows,
            columns.first().unwrap().len(),
            "Now enough rows in columns Vec."
        );
        Pattern {
            rows: input,
            columns,
            number_of_rows,
            number_of_columns,
        }
    }

    fn from_vec_strings(input: Vec<String>) -> Vec<Pattern> {
        let mut result: Vec<Pattern> = Vec::new();
        let mut current: Vec<String> = Vec::new();
        for line in input {
            if line.is_empty() {
                result.push(Pattern::from_strings(current.clone()));
                current.clear();
            } else {
                current.push(line);
            }
        }

        result
    }

    fn find_mirrors(lines: &Lines) -> Option<Mirrors> {
        for i in 0..lines.len() - 1 {
            let k = i + 1;
            let a = &lines[i];
            let b = &lines[k];
            if a == b {
                let mut found = true;
                for j in 1..lines.len() - k {
                    if i < j {
                        return Some((i, k));
                    }
                    let c = &lines[i - j];
                    let d = &lines[k + j];
                    if c != d {
                        found = false;
                        break;
                    }
                }
                if found {
                    return Some((i, k));
                }
            }
        }
        None
    }

    fn summarize(&self) -> usize {
        let vertical_mirror = Pattern::find_mirrors(&self.columns);
        if let Some(mirror) = vertical_mirror {
            return mirror.1;
        }
        let horizontal_mirror = Pattern::find_mirrors(&self.rows);
        if let Some(mirror) = horizontal_mirror {
            return mirror.1 * 100;
        }
        panic!("Could not find horizontal or vertical mirror rows.")
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
    let example_patterns = Pattern::from_vec_strings(example_input);
    dbg!(&example_patterns);
    test(2, example_patterns.len());
    test(5, example_patterns[0].summarize());
    test(400, example_patterns[1].summarize());
    let sum: usize = example_patterns.iter().map(|p| p.summarize()).sum();
    test(405, sum);

    // Part 1
    let sum: usize = Pattern::from_vec_strings(aoc_input::get(2023, 13))
        .iter()
        .map(|p| p.summarize())
        .sum();
    test(30535, sum);
}
