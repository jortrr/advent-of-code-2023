type Int = i32;

struct Record {
    springs: String,
    damaged_spring_groups: Vec<Int>,
    valid_arrangements: Vec<String>,
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
        let mut record = Record {
            springs,
            damaged_spring_groups,
            valid_arrangements: Vec::new(),
        };
        record.generate_arrangements();
        record
    }

    fn valid(input: &str, damaged_spring_groups: &Vec<Int>) -> bool {
        input
            .split('.')
            .filter(|s| !s.is_empty())
            .map(|s| s.len() as Int)
            .collect::<Vec<Int>>()
            == *damaged_spring_groups
    }

    fn generate(input: &str) -> Vec<String> {
        let mut ready_strings = Vec::new();
        let mut unready_strings = vec![input.to_string()];
        while !unready_strings.is_empty() {
            let current_string = unready_strings.pop().unwrap();
            if current_string.contains("?") {
                let a = current_string.replacen("?", ".", 1);
                let b = current_string.replacen("?", "#", 1);
                unready_strings.push(a);
                unready_strings.push(b);
            } else {
                ready_strings.push(current_string);
            }
        }
        ready_strings
    }

    fn generate_arrangements(&mut self) {
        self.valid_arrangements = Record::generate(&self.springs)
            .iter()
            .filter(|s| Record::valid(s, &self.damaged_spring_groups))
            .cloned()
            .collect();
    }

    fn sum_arrangements(input: Vec<String>) -> Int {
        input
            .iter()
            .filter(|&s| !s.is_empty())
            .map(|s| Record::from_string(s))
            .map(|r| r.valid_arrangements.len() as Int)
            .sum()
    }
}

fn main() {
    // Part 1 - Example
    assert!(Record::valid(&"#.#.###", &vec![1, 1, 3]));
    assert!(Record::valid(&".###..##.#..", &vec![3, 2, 1]));
    let example_input = vec![
        "???.### 1,1,3",
        ".??..??...?##. 1,1,3",
        "?#?#?#?#?#?#?#? 1,3,1,6",
        "????.#...#... 4,1,1",
        "????.######..#####. 1,6,5",
        "?###???????? 3,2,1",
    ];
    let sum: Int = Record::sum_arrangements(example_input.iter().map(|s| s.to_string()).collect());
    dbg!(sum);
    assert_eq!(21, sum, "This example value is always equal to 21.");

    // Part 1
    let sum = Record::sum_arrangements(aoc_input::get(2023, 12));
    dbg!(sum);
}
