type Int = i32;

static RUN_PART_1: bool = true;
static RUN_PART_2: bool = false;
static DEBUG: bool = false;

macro_rules! debug_print {
    ($($arg:tt)*) => {
        if DEBUG {
            use std::io::{self, Write};
            println!($($arg)*);
            io::stdout().flush().unwrap();
        }
    };
}

struct Record {
    springs: String,
    damaged_spring_groups: Vec<Int>,
}

impl Record {
    fn from_string(input: &str) -> Record {
        let mut iter = input.split_ascii_whitespace();
        let springs = iter.next().unwrap().to_string();
        let damaged_spring_groups: Vec<Int> = iter
            .next()
            .unwrap()
            .split(",")
            .map(|v| v.parse::<Int>().unwrap())
            .collect();
        Record {
            springs,
            damaged_spring_groups,
        }
    }

    fn solve(&self) -> Int {
        solve(&self.springs, &self.damaged_spring_groups, 0, "")
    }

    fn expand(&self) -> Record {
        Record {
            springs: vec![&self.springs]
                .repeat(5)
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .join("?"),
            damaged_spring_groups: self.damaged_spring_groups.repeat(5),
        }
    }

    fn test(input: &str, expected_arrangements: Int) {
        let record = Record::from_string(input);
        let actual_arrangements = record.solve();
        dbg!((input, actual_arrangements));
        assert_eq!(
            expected_arrangements, actual_arrangements,
            "Test case failed: this value should always equal '{}'.",
            expected_arrangements
        );
    }

    fn test_expanded(input: &str, expected_arrangements: Int) {
        let record = Record::from_string(input).expand();
        let actual_arrangements = record.solve();
        dbg!((input, actual_arrangements));
        assert_eq!(
            expected_arrangements, actual_arrangements,
            "Test case failed: this value should always equal '{}'.",
            expected_arrangements
        );
    }
}

fn print_valid(indent: &str, thrown_away: &str, record: &str) {
    debug_print!("{}* Valid: \"{}{}\"", indent, thrown_away, record);
}
fn print_invalid(indent: &str, thrown_away: &str, record: &str) {
    debug_print!("{}* Invalid: \"{}{}\"", indent, thrown_away, record);
}

/// #Bried Take in a record and a vector of groups of damaged strings, and return the amount of different valid records that are possible.
/// # Parameters
/// - record: e.g. `????.######..#####.`
/// - groups: e.g. `vec![1,6,5]`
/// # Returns
/// - Int: e.g. 4
///
/// # Example
/// We will use dynamic programming to solve this problem.
/// For example:
/// ```rust
///     solve(`????.######..#####.`, vec![1,6,5])
///     ==
///     solve(`????`, vec![1]) *
///     solve(`######`, vec![6]) *
///     solve(`#####`, vec![5])
///     ==
///     4 * 1 * 1
///     ==
///     4
///```
#[cfg(not(feature = "print_enabled"))]
fn solve(record: &str, groups: &Vec<Int>, depth: usize, thrown_away: &str) -> Int {
    if depth == 0 && !record.ends_with(".") {
        return solve(&format!("{}.", record), groups, depth, thrown_away);
    }
    let indent = "-".repeat(depth);
    if DEBUG {
        println!(
            "{}* solve(\"{}\", \"{}\", {:?}, {})",
            indent, thrown_away, record, groups, depth
        );
    }
    let groups_sum: Int = groups.iter().sum();
    if groups_sum >= record.len() as Int {
        print_invalid(&indent, thrown_away, record);
        return 0;
    }
    if groups.is_empty() {
        if record.chars().all(|c| c == '.' || c == '?') || record.is_empty() {
            print_valid(&indent, thrown_away, record);
            return 1;
        }
        print_invalid(&indent, thrown_away, record);
        return 0;
    } else if record.is_empty() {
        print_invalid(&indent, thrown_away, record);
        return 0;
    }
    let depth = depth + 1;

    for (i, c) in record.chars().enumerate() {
        if i as Int > *groups.first().unwrap() {
            print_invalid(&indent, thrown_away, record);
            return 0;
        }
        match c {
            '.' => {
                if i == 0 {
                    return solve(
                        &record[1..],
                        groups,
                        depth,
                        &format!("{}{}", thrown_away, &record[..1]),
                    );
                } else if i as Int == *groups.first().unwrap() {
                    debug_print!("{}* Removed group of '{}'.", indent, i);
                    return solve(
                        &record[i..],
                        &groups.iter().skip(1).cloned().collect(),
                        depth,
                        &format!("{}{}", thrown_away, &record[..i]),
                    );
                } else {
                    print_invalid(&indent, thrown_away, record);
                    return 0;
                }
            }
            '#' => {
                continue;
            }
            '?' => {
                let a = record.replacen("?", ".", 1);
                let b = record.replacen("?", "#", 1);
                return solve(&a, groups, depth, thrown_away)
                    + solve(&b, groups, depth, thrown_away);
            }
            _ => panic!("Invalid char '{}' in record.", c),
        };
    }
    print_valid(&indent, thrown_away, record);
    1
}

fn main() {
    // Part 1 - Example
    //assert!(Record::valid(&"#.#.###", &vec![1, 1, 3]));
    //assert!(Record::valid(&".###..##.#..", &vec![3, 2, 1]));
    let sum: Int = vec![
        "???.### 1,1,3",
        ".??..??...?##. 1,1,3",
        "?#?#?#?#?#?#?#? 1,3,1,6",
        "????.#...#... 4,1,1",
        "????.######..#####. 1,6,5",
        "?###???????? 3,2,1",
    ]
    .iter()
    .map(|s| Record::from_string(s).solve())
    .sum();
    dbg!(sum);
    assert_eq!(21, sum, "This example value is always equal to 21.");

    Record::test("???.### 1,1,3", 1);
    Record::test(".??..??...?##. 1,1,3", 4);
    Record::test("?#?#?#?#?#?#?#? 1,3,1,6", 1);
    Record::test("????.#...#... 4,1,1", 1);
    Record::test("????.######..#####. 1,6,5", 4);
    Record::test("?###???????? 3,2,1", 10);

    // Part 2 - Example
    Record::test_expanded("???.### 1,1,3", 1);
    Record::test_expanded(".??..??...?##. 1,1,3", 16384);
    Record::test_expanded("?#?#?#?#?#?#?#? 1,3,1,6", 1);
    Record::test_expanded("????.#...#... 4,1,1", 16);
    Record::test_expanded("????.######..#####. 1,6,5", 2500);
    Record::test_expanded("?###???????? 3,2,1", 506250);

    // Part 1
    if RUN_PART_1 {
        let sum: Int = aoc_input::get(2023, 12)
            .iter()
            .filter(|s| !s.is_empty())
            .map(|s| Record::from_string(s).solve())
            .sum();
        dbg!(sum);
        assert_eq!(6935, sum, "This AOC value is always equal to 6935 for me.")
    }

    // Part 2
    if RUN_PART_2 {
        let sum: Int = aoc_input::get(2023, 12)
            .iter()
            .filter(|s| !s.is_empty())
            .map(|s| Record::from_string(s).expand().solve())
            .sum();
        dbg!(sum);
    }
    // assert_eq!(6935, sum, "This AOC value is always equal to 6935 for me.")
}
