use crate::*;

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

impl Parse for Instructions {
    fn parse(input: Input) -> Self {
        let first_line = input.lines().next().unwrap();
        first_line
            .chars()
            .map(|c| Instruction::from_char(c))
            .collect()
    }
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

impl Parse for Network {
    fn parse(input: Input) -> Self {
        Network::from_strings(&input.lines().map(|s| s.to_string()).collect())
    }
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

pub struct DayEight {}

impl Problem for DayEight {
    fn year(&self) -> Year {
        2023
    }
    fn day(&self) -> Day {
        8
    }
    fn expect_part_one(&self) -> Answer {
        14257
    }
    fn expect_part_two(&self) -> Answer {
        16187743689077
    }

    define_examples! {
        (
            "
            LLR

            AAA = (BBB, BBB)
            BBB = (AAA, ZZZ)
            ZZZ = (ZZZ, ZZZ)
            ",
            Expect::PartsOneAndTwo(6, 6),
        )
    }

    fn solve_part_one(&self, input: Input, _is_example: bool) -> Answer {
        let instructions = Instructions::parse(input.clone());
        let network = Network::parse(input);
        let camel_steps_until_zzz_is_reached =
            network.camel_steps_until_zzz_is_reached(&instructions);
        camel_steps_until_zzz_is_reached as Answer
    }

    fn solve_part_two(&self, input: Input, _is_example: bool) -> Answer {
        let instructions = Instructions::parse(input.clone());
        let network = Network::parse(input);
        let ghost_steps_until_zzz_is_reached =
            network.ghost_steps_until_zzz_is_reached(&instructions);
        ghost_steps_until_zzz_is_reached as Answer
    }
}
