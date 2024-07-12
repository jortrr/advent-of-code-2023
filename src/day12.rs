use std::collections::HashMap;

type Int = i64;
type Key = (String, Vec<Int>);
type Memo = HashMap<Key, Int>;

static RUN_PART_1: bool = true;
static RUN_PART_2: bool = true;

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

    fn solve(&self, memo: &mut Memo) -> Int {
        solve(
            &(self.springs.to_string() + "."),
            &self.damaged_spring_groups,
            memo,
        )
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

    fn test(input: &str, expected_arrangements: Int, memo: &mut Memo) {
        let record = Record::from_string(input);
        let actual_arrangements = record.solve(memo);
        dbg!((input, actual_arrangements));
        assert_eq!(
            expected_arrangements, actual_arrangements,
            "Test case failed: this value should always equal '{}'.",
            expected_arrangements
        );
    }

    fn test_expanded(input: &str, expected_arrangements: Int, memo: &mut Memo) {
        let record = Record::from_string(input).expand();
        let actual_arrangements = record.solve(memo);
        dbg!((input, actual_arrangements));
        assert_eq!(
            expected_arrangements, actual_arrangements,
            "Test case failed: this value should always equal '{}'.",
            expected_arrangements
        );
    }
}

/// # Brief
/// Take in a record and a vector of groups of damaged strings, and return the amount of
/// different valid records that are possible.
/// # Parameters
/// - record: e.g. `????.######..#####.`
/// - groups: e.g. `vec![1,6,5]`
/// # Returns
/// - Int: e.g. 4
///
/// # Notes
/// We will use dynamic programming to solve this problem.
fn solve(record: &str, groups: &Vec<Int>, memo: &mut Memo) -> Int {
    let key: Key = (record.to_string(), groups.to_vec());
    if memo.contains_key(&key) {
        return *memo.get(&key).unwrap();
    }
    let mut result = 0;
    if groups.is_empty() {
        if record.chars().all(|c| c == '.' || c == '?') || record.is_empty() {
            result = 1;
        } else {
            result = 0;
        }
    } else if record.is_empty() {
        result = 0;
    } else {
        for (i, c) in record.chars().enumerate() {
            match c {
                '.' => {
                    if i == 0 {
                        result = solve(&record[1..], groups, memo);
                        break;
                    } else if i as Int == *groups.first().unwrap() {
                        result = solve(
                            &record[i..],
                            &groups.iter().skip(1).cloned().collect(),
                            memo,
                        );
                        break;
                    } else {
                        result = 0;
                        break;
                    }
                }
                '#' => {
                    continue;
                }
                '?' => {
                    let a = record.replacen("?", ".", 1);
                    let b = record.replacen("?", "#", 1);
                    result = solve(&a, groups, memo) + solve(&b, groups, memo);
                    break;
                }
                _ => panic!("Invalid char '{}' in record.", c),
            };
        }
    }
    memo.insert(key, result);
    result
}

fn main() {
    let mut memo = Memo::new();
    // Part 1 - Example
    Record::test("???.### 1,1,3", 1, &mut memo);
    Record::test(".??..??...?##. 1,1,3", 4, &mut memo);
    Record::test("?#?#?#?#?#?#?#? 1,3,1,6", 1, &mut memo);
    Record::test("????.#...#... 4,1,1", 1, &mut memo);
    Record::test("????.######..#####. 1,6,5", 4, &mut memo);
    Record::test("?###???????? 3,2,1", 10, &mut memo);

    let sum: Int = vec![
        "???.### 1,1,3",
        ".??..??...?##. 1,1,3",
        "?#?#?#?#?#?#?#? 1,3,1,6",
        "????.#...#... 4,1,1",
        "????.######..#####. 1,6,5",
        "?###???????? 3,2,1",
    ]
    .iter()
    .map(|s| Record::from_string(s).solve(&mut memo))
    .sum();
    dbg!(sum);
    assert_eq!(21, sum, "This example value is always equal to 21.");

    // Part 2 - Example
    Record::test_expanded("???.### 1,1,3", 1, &mut memo);
    Record::test_expanded(".??..??...?##. 1,1,3", 16384, &mut memo);
    Record::test_expanded("?#?#?#?#?#?#?#? 1,3,1,6", 1, &mut memo);
    Record::test_expanded("????.#...#... 4,1,1", 16, &mut memo);
    Record::test_expanded("????.######..#####. 1,6,5", 2500, &mut memo);
    Record::test_expanded("?###???????? 3,2,1", 506250, &mut memo);

    // Part 1
    if RUN_PART_1 {
        let sum: Int = aoc_input::get(2023, 12)
            .iter()
            .filter(|s| !s.is_empty())
            .map(|s| Record::from_string(s).solve(&mut memo))
            .sum();
        dbg!(sum);
        //assert_eq!(6935, sum, "This AOC value is always equal to 6935 for me.")
    }

    // Part 2
    if RUN_PART_2 {
        let sum: Int = aoc_input::get(2023, 12)
            .iter()
            .filter(|s| !s.is_empty())
            .map(|s| Record::from_string(s).expand().solve(&mut memo))
            .sum();
        dbg!(sum);
    }
}
