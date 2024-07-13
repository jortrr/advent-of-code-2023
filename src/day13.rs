use std::fmt::Debug;

static RUN_PART_2: bool = true;

type Grid<T> = Vec<Vec<T>>;
type Point = (usize, usize);

#[derive(Clone)]
struct Pattern {
    grid: Grid<char>,
    rows: usize,
    columns: usize,
}

impl Pattern {
    fn from_strings(input: Vec<String>) -> Pattern {
        let rows = input.len();
        let columns = input.first().unwrap().len();
        let grid: Grid<char> = input.iter().map(|s| s.chars().collect()).collect();
        Pattern {
            grid,
            rows,
            columns,
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

    fn compare(a: &Vec<char>, b: &Vec<char>) -> usize {
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

    fn find_reflection(&self, smudges: usize) -> Option<Point> {
        let columns: Grid<char> = (0..self.columns).map(|i| self.get_column(i)).collect();

        // Check whether any 2 rows form a reflection, with exactly smudges amount of imperfections
        for i in 0..self.rows - 1 {
            let a = self.get_row(i);
            let b = self.get_row(i + 1);
            let mut diff = Pattern::compare(a, b);
            for j in 1..self.rows - 1 - i {
                if diff > smudges || i < j {
                    break;
                }
                let c = self.get_row(i - j);
                let d = self.get_row(i + 1 + j);
                diff += Pattern::compare(&c, &d);
            }
            if diff == smudges {
                return Some((i, 0));
            }
        }

        // Check whether any 2 columns form a reflection, with exactly smudges amount of imperfections
        for i in 0..self.columns - 1 {
            let a = &columns[i];
            let b = &columns[i + 1];
            let mut diff = Pattern::compare(&a, &b);
            for j in 1..self.columns - 1 - i {
                if diff > smudges || i < j {
                    break;
                }
                let c = &columns[i - j];
                let d = &columns[i + 1 + j];
                diff += Pattern::compare(&c, &d);
            }
            if diff == smudges {
                return Some((0, i));
            }
        }

        // No reflections found, return None
        None
    }

    fn summarize(&self, smudges: usize) -> usize {
        let reflection = self.find_reflection(smudges);
        match reflection {
            Some((i, 0)) => return (i + 1) * 100,
            Some((0, i)) => return i + 1,
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
    let mut example_patterns = Pattern::from_vec_strings(example_input);
    //dbg!(&example_patterns);
    test(2, example_patterns.len());
    test(5, example_patterns[0].summarize(0));
    test(400, example_patterns[1].summarize(0));
    let sum: usize = example_patterns.iter_mut().map(|p| p.summarize(0)).sum();
    test(405, sum);

    // Part 2 - Example
    /*let mut smudges_0 = example_patterns[0].find_valid_smudges();
    test(300, smudges_0.1[0].summarize().unwrap());

    dbg!(smudges_0);

    let mut smudges_1 = example_patterns[1].find_valid_smudges();
    test(100, smudges_1.1[0].summarize().unwrap());

    // Part 1
    let sum: usize = Pattern::from_vec_strings(aoc_input::get(2023, 13))
        .iter_mut()
        .map(|p| p.summarize().unwrap())
        .sum();
    test(30535, sum);

    // Part 2
    if RUN_PART_2 {
        let x: Vec<_> = Pattern::from_vec_strings(aoc_input::get(2023, 13))
            .iter_mut()
            .map(|p| p.find_valid_smudges())
            .collect();
        dbg!(x);
    }*/
}
