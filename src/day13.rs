use std::fmt::Debug;

static RUN_PART_2: bool = true;

type Rows = (usize, usize);

#[derive(Clone)]
enum Mirror {
    Row(Rows),
    Columns(Rows),
}

impl std::fmt::Debug for Mirror {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Row(arg0) => write!(f, "Mirror::Row({},{})", arg0.0, arg0.1),
            Self::Columns(arg0) => write!(f, "Mirror::Column({},{})", arg0.0, arg0.1),
        }
    }
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

type Line = Vec<char>;
type Lines = Vec<Line>;
type Int = i32;

#[derive(Clone)]
struct Pattern {
    rows: Lines,
    columns: Lines,
    number_of_rows: usize,
    number_of_columns: usize,
    mirror: Option<Mirror>,
    smudge: Option<Smudge>,
    summary: Option<usize>,
}

impl Debug for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut rows: Vec<String> = self
            .rows
            .iter()
            .map(|v| v.iter().collect::<String>())
            .collect();
        match self.mirror {
            Some(Mirror::Row(i)) => {
                for j in 0..self.number_of_rows {
                    if j == i.0 || j == i.1 {
                        rows[j].push('M');
                    } else {
                        rows[j].push(' ');
                    }
                }
            }
            Some(Mirror::Columns(i)) => {
                let mut new_row: Vec<char> = vec![' '; self.number_of_columns];
                new_row[i.0] = 'M';
                new_row[i.1] = 'M';
                rows.push(new_row.iter().collect());
            }
            _ => (),
        }
        match self.smudge {
            Some(Smudge::Row(i)) => {
                for j in 0..self.number_of_rows {
                    if j == i.0 || j == i.1 {
                        rows[j].push('S');
                    } else {
                        rows[j].push(' ');
                    }
                }
            }
            Some(Smudge::Columns(i)) => {
                let mut new_row: Vec<char> = vec![' '; self.number_of_columns];
                new_row[i.0] = 'S';
                new_row[i.1] = 'S';
                rows.push(new_row.iter().collect());
            }
            _ => (),
        }
        f.debug_struct("Pattern")
            .field("Map", &rows)
            .field("Summary", &self.summary.unwrap_or(0))
            .field(
                "Dimensions",
                &format!("{}x{}", self.number_of_rows, self.number_of_columns),
            )
            .finish()
    }
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
            mirror: None,
            smudge: None,
            summary: None,
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
                new_pattern.smudge = Some(smudge.clone());
                new_pattern.summarize();
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
                new_pattern.smudge = Some(smudge.clone());
                new_pattern.summarize();
                return new_pattern;
            }
            _ => panic!("Invalid smudge on pattern."),
        }
    }

    fn find_valid_smudges(&mut self) -> (Pattern, Vec<Pattern>) {
        let sum = self.summarize().unwrap();
        let smudges: Vec<Pattern> = self
            .find_smudges()
            .iter()
            .map(|s| self.desmudge(s))
            .collect();

        if smudges.iter().all(|s| s.summary.unwrap() == sum) {
            dbg!(&self);
            dbg!(&smudges);
            panic!("No new mirror could be found!");
        }

        (self.clone(), smudges)
    }

    fn get_mirror(&mut self) {
        if let Some(mirror) = Pattern::find_mirror_rows(&self.rows) {
            self.mirror = Some(Mirror::Row(mirror));
        } else if let Some(mirror) = Pattern::find_mirror_rows(&self.columns) {
            self.mirror = Some(Mirror::Columns(mirror));
        } else {
            self.mirror = None;
        }
    }

    fn summarize(&mut self) -> Option<usize> {
        self.get_mirror();
        if let Some(Mirror::Row(mirror)) = self.mirror {
            self.summary = Some(mirror.1 * 100);
            return self.summary;
        }
        if let Some(Mirror::Columns(mirror)) = self.mirror {
            self.summary = Some(mirror.1);
            return self.summary;
        }
        self.summary = None;
        self.summary
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
    test(5, example_patterns[0].summarize().unwrap());
    test(400, example_patterns[1].summarize().unwrap());
    let sum: usize = example_patterns
        .iter_mut()
        .map(|p| p.summarize().unwrap())
        .sum();
    test(405, sum);

    // Part 2 - Example
    let mut smudges_0 = example_patterns[0].find_valid_smudges();
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
    }
}
