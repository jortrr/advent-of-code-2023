mod problem;
use problem::*;

type Key = (String, Vec<Int>);
type Memo = HashMap<Key, Int>;

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
            (self.springs.to_string() + ".").clone(),
            self.damaged_spring_groups.clone(),
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

    #[cfg(test)]
    fn test(input: &str, expected_arrangements: Int, memo: &mut Memo) {
        let record = Record::from_string(input);
        let actual_arrangements = record.solve(memo);
        test!(
            expected_arrangements,
            actual_arrangements,
            "({} -> {})",
            input,
            actual_arrangements
        );
    }

    #[cfg(test)]
    fn test_expanded(input: &str, expected_arrangements: Int, memo: &mut Memo) {
        let record = Record::from_string(input).expand();
        let actual_arrangements = record.solve(memo);
        test!(
            expected_arrangements,
            actual_arrangements,
            "({} -> {})",
            input,
            actual_arrangements
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
fn solve(record: String, groups: Vec<Int>, memo: &mut Memo) -> Int {
    let key: Key = (record.clone(), groups.clone());
    if memo.contains_key(&key) {
        return *memo.get(&key).unwrap();
    }
    let mut result = 0;
    if groups.is_empty() {
        if !record.contains("#") || record.is_empty() {
            result = 1;
        }
    } else {
        for (i, c) in record.chars().enumerate() {
            match c {
                '.' => {
                    if i == 0 {
                        result = solve(record[1..].to_string(), groups, memo);
                        break;
                    } else if i as Int == *groups.first().unwrap() {
                        result = solve(record[i..].to_string(), groups[1..].to_vec(), memo);
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
                    result = solve(a, groups.clone(), memo) + solve(b, groups, memo);
                    break;
                }
                _ => panic!("Invalid char '{}' in record.", c),
            };
        }
    }
    memo.insert(key, result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one_example() {
        let mut memo = Memo::new();
        Record::test("???.### 1,1,3", 1, &mut memo);
        Record::test(".??..??...?##. 1,1,3", 4, &mut memo);
        Record::test("?#?#?#?#?#?#?#? 1,3,1,6", 1, &mut memo);
        Record::test("????.#...#... 4,1,1", 1, &mut memo);
        Record::test("????.######..#####. 1,6,5", 4, &mut memo);
        Record::test("?###???????? 3,2,1", 10, &mut memo);
    }

    #[test]
    fn test_part_two_example() {
        let mut memo = Memo::new();
        Record::test_expanded("???.### 1,1,3", 1, &mut memo);
        Record::test_expanded(".??..??...?##. 1,1,3", 16384, &mut memo);
        Record::test_expanded("?#?#?#?#?#?#?#? 1,3,1,6", 1, &mut memo);
        Record::test_expanded("????.#...#... 4,1,1", 16, &mut memo);
        Record::test_expanded("????.######..#####. 1,6,5", 2500, &mut memo);
        Record::test_expanded("?###???????? 3,2,1", 506250, &mut memo);
    }
}

struct DayTwelve {}

impl Problem for DayTwelve {
    const YEAR: Year = 2023;
    const DAY: Day = 12;
    const PART_ONE_EXAMPLE_EXPECTED: Answer = 21;
    const PART_ONE_EXPECTED: Answer = 6935;
    const PART_TWO_EXAMPLE_EXPECTED: Answer = 525152;
    const PART_TWO_EXPECTED: Answer = 3920437278260;

    fn example_input() -> ExampleInput {
        "
        ???.### 1,1,3
        .??..??...?##. 1,1,3
        ?#?#?#?#?#?#?#? 1,3,1,6
        ????.#...#... 4,1,1
        ????.######..#####. 1,6,5
        ?###???????? 3,2,1
        "
    }

    fn solve_part_one(input: Input, _is_example: bool) -> Answer {
        let mut memo = Memo::new();
        let sum: Int = input
            .lines()
            .filter(|s| !s.is_empty())
            .map(|s| Record::from_string(s).solve(&mut memo))
            .sum();
        sum
    }

    fn solve_part_two(input: Input, _is_example: bool) -> Answer {
        let mut memo = Memo::new();
        let sum: Int = input
            .lines()
            .filter(|s| !s.is_empty())
            .map(|s| Record::from_string(s).expand().solve(&mut memo))
            .sum();
        sum
    }
}

run!(DayTwelve);
