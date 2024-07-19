mod macros;
use std::fmt::Debug;

type Int = i32;
type InitializationSequence = Vec<Step>;

struct Lens {
    label: String,
    focal_length: Int,
    box_number: Int,
}

impl Debug for Lens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{} {}]", self.label, self.focal_length)
    }
}

impl Lens {
    fn from_string(step: &str) -> Lens {
        let mut command = step.split("=");
        let label = command.next().unwrap().to_string();
        let focal_length: Int = command.next().unwrap().parse().unwrap();
        let box_number = Step::get_hash_value(&label);
        Lens {
            label,
            focal_length,
            box_number,
        }
    }
}

struct Box {
    box_number: Int,
    lenses: Vec<Lens>,
}

impl Debug for Box {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.lenses.is_empty() {
            write!(
                f,
                "Box {}: {}",
                self.box_number,
                self.lenses
                    .iter()
                    .map(|l| format!("{:?}", l))
                    .collect::<Vec<_>>()
                    .join(" ")
            )
        } else {
            write!(f, "")
        }
    }
}

impl Box {
    fn get_lens_position(&self, label: &str) -> Option<usize> {
        self.lenses.iter().position(|l| l.label == label)
    }

    fn get_focusing_power(&self) -> Int {
        let focusing_power_from_box = 1 + self.box_number;
        let focussing_power = self
            .lenses
            .iter()
            .enumerate()
            .map(|(i, l)| focusing_power_from_box * (i + 1) as Int * l.focal_length)
            .sum();
        debug!(false, focussing_power);
        focussing_power
    }
}

struct BoxSequence {
    boxes: Vec<Box>,
}

impl Debug for BoxSequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.boxes
                .iter()
                .filter(|b| !b.lenses.is_empty())
                .map(|b| format!("{:?}", b))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

impl BoxSequence {
    fn new() -> BoxSequence {
        static NUMBER_OF_BOXES: Int = 256;
        let mut boxes: Vec<Box> = Vec::new();
        for i in 0..NUMBER_OF_BOXES {
            boxes.push(Box {
                box_number: i,
                lenses: Vec::new(),
            });
        }
        BoxSequence { boxes }
    }

    fn get_total_focussing_power(&self) -> Int {
        self.boxes.iter().map(|b| b.get_focusing_power()).sum()
    }

    fn execute(&mut self, step: &Step) {
        if step.to_string.contains("-") {
            self.remove_lens(step);
        } else if step.to_string.contains("=") {
            self.insert_lens(step);
        } else {
            panic!("Invalid step: '{:?}'.", step);
        }
        debug!(false, "After {:?}:", step.to_string);
        debug!(false, "{:?}\n", self);
    }

    fn remove_lens(&mut self, step: &Step) {
        assert!(step.to_string.contains("-"));
        let label = step.to_string.split("-").next().unwrap();
        let box_number = Step::get_hash_value(&label);
        let current_box = &mut self.get_box(box_number);
        if let Some(position_of_lens_with_label) = current_box.get_lens_position(&label) {
            current_box.lenses.remove(position_of_lens_with_label);
        }
    }

    fn insert_lens(&mut self, step: &Step) {
        assert!(step.to_string.contains("="));
        let lens = Lens::from_string(&step.to_string);
        let current_box = &mut self.get_box(lens.box_number);
        if let Some(position_of_lens_with_label) = current_box.get_lens_position(&lens.label) {
            current_box.lenses[position_of_lens_with_label] = lens;
        } else {
            current_box.lenses.push(lens);
        }
    }

    fn get_box(&mut self, index: Int) -> &mut Box {
        &mut self.boxes[index as usize]
    }
}

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
        test!(hash_value, Step::from_string_slice(input).hash_value, input);
    }
    let example_sequence_input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    let example_sequence = Step::from_string_slice_list(&example_sequence_input);
    let example_sequence_sum = sum_steps(&example_sequence);
    test!(1320, example_sequence_sum);

    // Part 1
    let input: String = aoc_input::get(2023, 15)
        .iter()
        .filter(|s| !s.is_empty())
        .cloned()
        .collect();
    dbg!(&input);
    let sequence = Step::from_string_list(&input);
    let steps_sum = sum_steps(&sequence);
    test!(507769, steps_sum, "Part 1: steps_sum == {}", steps_sum);

    // Part 2 - Example
    let mut example_box_sequence = BoxSequence::new();
    example_sequence
        .iter()
        .for_each(|s| example_box_sequence.execute(&s));
    let example_total_focussing_power = example_box_sequence.get_total_focussing_power();
    test!(145, example_total_focussing_power);

    // Part 2
    let mut box_sequence = BoxSequence::new();
    sequence.iter().for_each(|s| box_sequence.execute(&s));
    let total_focussing_power = box_sequence.get_total_focussing_power();
    test!(269747, total_focussing_power);
}
