use aocf::Aoc;

fn main() {
    let sum = Aoc::new().year(Some(2023)).day(Some(1)).init().unwrap().get_input(false).split("\n").collect::<Vec<&str>>().iter().map(|&line| line.chars().filter(|c|c.is_digit()).collect::<String>()).map(|&line|format!("{}{}",line.chars().next().unwrap(),line.chars().last().unwrap()).parse::<u32>().unwrap()).sum();
}
