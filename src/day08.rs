#[derive(Debug)]
enum Instruction {
    Left,
    Right,
}

impl Instruction {
    fn from_char(c: char) -> Instruction {
        use Instruction::*;
        match c {
            'L' => Left,
            'R' => Right,
            _ => panic!("Invalid instruction: '{}'.", c),
        }
    }
}

type Instructions = Vec<Instruction>;

fn instructions_from_string(string: &str) -> Instructions {
    string.chars().map(|c| Instruction::from_char(c)).collect()
}

#[derive(Debug)]
struct Node {
    from_string: String,
    label: String,
    left: String,
    right: String,
}

impl Node {
    fn from_string(string: &str) -> Option<Node> {
        if string.len() != 16 {
            return None;
        }
        let from_string = string.to_string();
        let label = string[0..3].to_string();
        let left = string[7..10].to_string();
        let right = string[12..15].to_string();
        Some(Node {
            from_string,
            label,
            left,
            right,
        })
    }
}

#[derive(Debug)]
struct Network {
    nodes: Vec<Node>,
}

impl Network {
    fn find_node_from_string(&self, label: &str) -> Option<&Node> {
        self.nodes.iter().find(|node| node.label == label)
    }

    fn steps_until_zzz_is_reached(&self, instructions: &Instructions) -> i32 {
        let mut steps: i32 = 0;
        let mut current_node: &Node = self.find_node_from_string("AAA").unwrap();
        loop {
            for (i, instruction) in instructions.iter().enumerate() {
                //dbg!((steps, i));
                //dbg!(instruction);
                //dbg!(current_node);
                let new_node = match instruction {
                    Instruction::Left => &current_node.left,
                    Instruction::Right => &current_node.right,
                };
                current_node = self.find_node_from_string(new_node).unwrap();
                steps += 1;
                if current_node.label == "ZZZ" {
                    return steps;
                }
            }
        }
    }

    fn from_strings(input: &Vec<String>) -> Network {
        Network {
            nodes: input
                .iter()
                .filter(|line| line.len() == 16)
                .map(|line| Node::from_string(line).unwrap())
                .collect(),
        }
    }
}

fn main() {
    println!("Hello, World! from src/day08.rs!");
    // Example
    let input: Vec<String> = vec![
        "LLR",
        "",
        "AAA = (BBB, BBB)",
        "BBB = (AAA, ZZZ)",
        "ZZZ = (ZZZ, ZZZ)",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
    let instructions = instructions_from_string(&input[0]);
    let network = Network::from_strings(&input);
    let steps_until_zzz_is_reached = network.steps_until_zzz_is_reached(&instructions);
    dbg!(steps_until_zzz_is_reached);
    assert_eq!(
        6, steps_until_zzz_is_reached,
        "This value should always be 2."
    );
    //Part 1
    let input = aoc_input::get(2023, 8);
    let instructions = instructions_from_string(&input[0]);
    let network = Network::from_strings(&input);
    let steps_until_zzz_is_reached = network.steps_until_zzz_is_reached(&instructions);
    //dbg!(&network);
    //dbg!(&instructions);
    dbg!(steps_until_zzz_is_reached);
}
