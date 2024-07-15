mod test;
use test::*;
type Int = i32;
type InitializationSequence = Vec<Step>;

#[derive(Debug)]
struct Step {
    to_string: String,
    hash_value: Int,
}

impl Step {
    fn from_string_list(input: &String) -> InitializationSequence {
        Step::from_string_slice_list(&input)
    }

    fn from_string_slice_list(input: &str) -> InitializationSequence {
        input
            .split(",")
            .collect::<Vec<&str>>()
            .iter()
            .map(|s| Step::from_string_slice(s))
            .collect()
    }

    fn from_string(input: &String) -> Step {
        let to_string = input.clone();
        let hash_value = Step::get_hash_value(&to_string);
        Self {
            to_string,
            hash_value,
        }
    }

    fn from_string_slice(input: &str) -> Step {
        Step::from_string(&input.to_string())
    }

    fn get_hash_value(string: &str) -> Int {
        let mut current_value: Int = 0;
        for c in string.chars() {
            let ascii_value_from_char = Step::get_char_hash_value(c);
            current_value += ascii_value_from_char;
            current_value *= 17;
            current_value %= 256;
        }
        current_value
    }

    fn get_char_hash_value(character: char) -> Int {
        character as u8 as Int
    }
}

fn sum_steps(initialization_sequence: &InitializationSequence) -> Int {
    initialization_sequence.iter().map(|s| s.hash_value).sum()
}

fn main() {
    println!("Hello, World! from src/day15.rs!");
    // Part 1 - Examples
    let test_cases = vec![
        ("rn=1", 30),
        ("cm-", 253),
        ("qp=3", 97),
        ("cm=2", 47),
        ("qp-", 14),
        ("pc=4", 180),
        ("ot=9", 9),
        ("ab=5", 197),
        ("pc-", 48),
        ("pc=6", 214),
        ("ot=7", 231),
    ];
    for (input, hash_value) in test_cases {
        test_and_debug(
            &hash_value,
            &Step::from_string_slice(input).hash_value,
            &format!("({:?}, {})", input, hash_value),
        );
    }
    let example_sequence_input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    let example_sequence = Step::from_string_slice_list(&example_sequence_input);
    let example_sequence_sum = sum_steps(&example_sequence);
    test_and_debug(
        &1320,
        &example_sequence_sum,
        &format!("({:?}, {})", example_sequence_input, example_sequence_sum),
    );

    // Part 1
    let input: String = aoc_input::get(2023, 15)
        .iter()
        .filter(|s| !s.is_empty())
        .cloned()
        .collect();
    let sequence = Step::from_string_list(&input);
    let steps_sum = sum_steps(&sequence);
    test_and_debug(
        &507769,
        &steps_sum,
        &format!("Part 1: steps_sum == {}", steps_sum),
    );
}
