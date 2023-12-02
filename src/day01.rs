use aocf::Aoc;

fn main() {
    let sum: u32 = Aoc::new().year(Some(2023)).day(Some(1)).init().unwrap().get_input(false).unwrap().split("\n").collect::<Vec<&str>>().iter().map(|line| line.chars().filter(|c| c.is_digit(10)).collect::<String>()).filter(|line| !line.is_empty()).map(|line| format!("{}{}", line.chars().next().unwrap(), line.chars().last().unwrap()).parse::<u32>().unwrap()).sum();
    dbg!(sum);
}
