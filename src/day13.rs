type Rows = (usize, usize);

enum Mirror {
    Row(Rows),
    Columns(Rows),
}

#[derive(Clone, PartialEq)]
enum Smudge {
    Row(Rows),
    Columns(Rows),
}

impl std::fmt::Debug for Smudge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Row(arg0) => write!(f, "Smudge::Row({},{})", arg0.0, arg0.1),
            Self::Columns(arg0) => write!(f, "Smudge::Column({},{})", arg0.0, arg0.1),
        }
    }
}

type SmudgeResult = (Smudge, usize);
type Line = Vec<char>;
type Lines = Vec<Line>;
type Int = i32;

#[derive(Debug, Clone)]
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
                    .collect::<Line>()
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
        let rows: Lines = input.iter().map(|s| s.chars().collect::<Line>()).collect();
        Pattern {
            rows,
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

    fn find_mirror_rows(lines: &Lines) -> Option<Rows> {
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

    fn find_smudge_rows(lines: &Lines) -> Vec<Rows> {
        let mut result: Vec<Rows> = Vec::new();
        for i in 0..lines.len() - 1 {
            for j in i + 1..lines.len() {
                let a = &lines[i];
                let b = &lines[j];
                let mut diff = 0;
                for k in 0..a.len() {
                    if a[k] != b[k] {
                        diff += 1;
                    }
                }
                if diff == 1 {
                    result.push((i, j));
                }
            }
        }
        result
    }

    fn find_smudges(&self) -> Vec<Smudge> {
        let smudge_rows: Vec<Smudge> = Pattern::find_smudge_rows(&self.rows)
            .iter()
            .map(|s| Smudge::Row(*s))
            .collect();
        let mut smudge_columns: Vec<Smudge> = Pattern::find_smudge_rows(&self.columns)
            .iter()
            .map(|s| Smudge::Columns(*s))
            .collect();
        let mut result = smudge_rows;
        result.append(&mut smudge_columns);
        result
    }

    fn desmudge(&self, smudge: &Smudge) -> Pattern {
        match smudge {
            Smudge::Row(rows) => {
                let mut new_pattern = self.clone();
                new_pattern.rows[rows.0] = new_pattern.rows[rows.1].clone();
                for i in 0..new_pattern.number_of_rows {
                    for j in 0..new_pattern.number_of_columns {
                        new_pattern.columns[j][i] = new_pattern.rows[i][j];
                    }
                }
                return new_pattern;
            }
            Smudge::Columns(rows) => {
                let mut new_pattern = self.clone();
                new_pattern.columns[rows.0] = new_pattern.columns[rows.1].clone();
                for i in 0..new_pattern.number_of_rows {
                    for j in 0..new_pattern.number_of_columns {
                        new_pattern.rows[i][j] = new_pattern.columns[j][i];
                    }
                }
                return new_pattern;
            }
            _ => panic!("Invalid smudge on pattern."),
        }
    }

    fn find_valid_smudges(&self) -> Vec<SmudgeResult> {
        let smudges: Vec<SmudgeResult> = self
            .find_smudges()
            .iter()
            .map(|s| (s, self.desmudge(s).summarize()))
            .filter(|(_, o)| o.is_some())
            .map(|(s, o)| (s.clone(), o.unwrap()))
            .collect();
        smudges
    }

    fn get_mirror(&self) -> Option<Mirror> {
        let horizontal_mirror = Pattern::find_mirror_rows(&self.rows);
        if let Some(mirror) = horizontal_mirror {
            return Some(Mirror::Row(mirror));
        }
        let vertical_mirror = Pattern::find_mirror_rows(&self.columns);
        if let Some(mirror) = vertical_mirror {
            return Some(Mirror::Columns(mirror));
        }
        None
    }

    fn summarize(&self) -> Option<usize> {
        let mirror = self.get_mirror();
        if let Some(Mirror::Row(mirror)) = mirror {
            return Some(mirror.1 * 100);
        }
        if let Some(Mirror::Columns(mirror)) = mirror {
            return Some(mirror.1);
        }
        None
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
    //dbg!(&example_patterns);
    test(2, example_patterns.len());
    test(5, example_patterns[0].summarize().unwrap());
    test(400, example_patterns[1].summarize().unwrap());
    let sum: usize = example_patterns
        .iter()
        .map(|p| p.summarize().unwrap())
        .sum();
    test(405, sum);

    // Part 2 - Example
    let smudges_0 = example_patterns[0].find_valid_smudges();
    test((Smudge::Row((0, 5)), 300), smudges_0[0].clone());

    let smudges_1 = example_patterns[1].find_valid_smudges();
    test((Smudge::Row((0, 1)), 100), smudges_1[0].clone());

    // Part 1
    let sum: usize = Pattern::from_vec_strings(aoc_input::get(2023, 13))
        .iter()
        .map(|p| p.summarize().unwrap())
        .sum();
    test(30535, sum);
}
