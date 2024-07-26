mod macros;

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

// Return the greatest common multiple of a and b
fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

/// Return the least common multiple of a and b
fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

type StepsFromNode<'a> = (&'a Node, i32);

#[derive(Debug)]
struct Network {
    nodes: Vec<Node>,
}

impl Network {
    fn find_node_from_string(&self, label: &str) -> Option<&Node> {
        self.nodes.iter().find(|node| node.label == label)
    }

    fn find_node_from_string_end(&self, label_end: &str) -> Vec<&Node> {
        self.nodes
            .iter()
            .filter(|node| node.label.ends_with(label_end))
            .collect()
    }

    fn camel_steps_until_zzz_is_reached(&self, instructions: &Instructions) -> i32 {
        let start_node = self.find_node_from_string("AAA").unwrap();
        let end_node_ends_with = "ZZZ";

        self.steps_until_zzz_is_reached(instructions, start_node, &end_node_ends_with)
    }

    fn ghost_steps_until_zzz_is_reached(&self, instructions: &Instructions) -> u64 {
        let start_nodes = self.find_node_from_string_end("A");
        let end_node_ends_with = "Z";
        //dbg!(&start_nodes);
        let steps_from_nodes: Vec<StepsFromNode> = start_nodes
            .iter()
            .map(|node| {
                (
                    *node,
                    self.steps_until_zzz_is_reached(instructions, node, end_node_ends_with),
                )
            })
            .collect();
        dbg!(&steps_from_nodes);
        let mut least_common_multiple = 1;
        for (_, steps) in steps_from_nodes {
            least_common_multiple = lcm(least_common_multiple, steps as u64);
        }
        least_common_multiple
    }

    fn steps_until_zzz_is_reached(
        &self,
        instructions: &Instructions,
        start_node: &Node,
        end_node_ends_with: &str,
    ) -> i32 {
        let mut steps: i32 = 0;
        let mut current_node: &Node = start_node;
        loop {
            for instruction in instructions {
                if current_node.label.ends_with(end_node_ends_with) {
                    return steps;
                }

                let new_node = match instruction {
                    Instruction::Left => &current_node.left,
                    Instruction::Right => &current_node.right,
                };

                current_node = self.find_node_from_string(new_node).unwrap();
                steps += 1;
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
    let example_camel_steps_until_zzz_is_reached =
        network.camel_steps_until_zzz_is_reached(&instructions);
    test!(6, example_camel_steps_until_zzz_is_reached);
    //Part 1
    let input = aoc::get(2023, 8);
    let instructions = instructions_from_string(&input[0]);
    let network = Network::from_strings(&input);
    let camel_steps_until_zzz_is_reached = network.camel_steps_until_zzz_is_reached(&instructions);
    //dbg!(&network);
    //dbg!(&instructions);
    test!(14257, camel_steps_until_zzz_is_reached);
    //Part 2
    let ghost_steps_until_zzz_is_reached = network.ghost_steps_until_zzz_is_reached(&instructions);
    test!(16187743689077 as u64, ghost_steps_until_zzz_is_reached);
}
