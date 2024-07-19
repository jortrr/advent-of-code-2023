use std::collections::VecDeque;

mod macros;

type Int = i32;
type Sequence = VecDeque<Int>;

struct History {
    sequences: Vec<Sequence>,
}

impl History {
    fn from_string(sequence: &str) -> History {
        let history: Sequence = sequence
            .split_ascii_whitespace()
            .map(|s| s.parse::<Int>().unwrap())
            .collect();
        History {
            sequences: vec![history],
        }
    }

    fn find_differences(&mut self) {
        loop {
            let last_sequence = self.sequences.last().unwrap();
            if last_sequence.iter().all(|x| *x == 0) {
                break;
            }
            let differences = History::get_differences(last_sequence);
            self.sequences.push(differences);
        }
    }

    fn extrapolate(&mut self) {
        let n = self.sequences.len() - 1;
        for i in (0..n).rev() {
            if i == n {
                self.sequences[i].push_back(0);
            } else {
                let extrapolation =
                    self.sequences[i + 1].back().unwrap() + self.sequences[i].back().unwrap();
                self.sequences[i].push_back(extrapolation);
            }
        }
    }

    fn extrapolate_first_value(&mut self) {
        let n = self.sequences.len() - 1;
        for i in (0..n).rev() {
            if i == n {
                self.sequences[i].push_front(0);
            } else {
                let extrapolation =
                    self.sequences[i].front().unwrap() - self.sequences[i + 1].front().unwrap();
                self.sequences[i].push_front(extrapolation);
            }
        }
    }

    fn get_differences(input_sequence: &Sequence) -> Sequence {
        Vec::from(input_sequence.clone())
            .windows(2)
            .map(|pair| pair[1] - pair[0])
            .collect()
    }
}

impl std::fmt::Debug for History {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut extrapolation = String::new();
        for (i, line) in self.sequences.iter().enumerate() {
            for _ in 0..i {
                extrapolation += "  ";
            }
            for n in line {
                extrapolation += &format!("{}   ", n).to_string();
            }
            extrapolation += "\n";
        }
        write!(f, "{}", extrapolation)
    }
}

#[derive(Debug)]
struct OASIS {
    histories: Vec<History>,
}

impl OASIS {
    fn from_strings(sequences: &Vec<String>) -> OASIS {
        let histories: Vec<History> = sequences
            .iter()
            .filter(|s| !s.is_empty())
            .map(|s| History::from_string(s))
            .collect();
        OASIS { histories }
    }

    fn extrapolate_histories(&mut self) {
        self.histories.iter_mut().for_each(|history| {
            history.find_differences();
            history.extrapolate();
            history.extrapolate_first_value()
        });
    }

    fn sum_of_histories_last_values(&self) -> Int {
        self.histories
            .iter()
            .map(|h| h.sequences.first().unwrap().back().unwrap())
            .sum()
    }

    fn sum_of_histories_first_values(&self) -> Int {
        self.histories
            .iter()
            .map(|h| h.sequences.first().unwrap().front().unwrap())
            .sum()
    }
}

fn main() {
    println!("Hello, World! from src/day09.rs!");
    // Example
    let input: Vec<String> = vec!["0 3 6 9 12 15", "1 3 6 10 15 21", "10 13 16 21 30 45"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let mut oasis = OASIS::from_strings(&input);
    oasis.extrapolate_histories();
    dbg!(&oasis);
    let sum_of_histories_last_values = oasis.sum_of_histories_last_values();
    test!(114, sum_of_histories_last_values);
    // Part 1
    let input = aoc_input::get(2023, 9);
    let mut oasis = OASIS::from_strings(&input);
    oasis.extrapolate_histories();
    let sum_of_histories_last_values = oasis.sum_of_histories_last_values();
    test!(2005352194, sum_of_histories_last_values);
    let sum_of_histories_first_values = oasis.sum_of_histories_first_values();
    test!(1077, sum_of_histories_first_values);
}
