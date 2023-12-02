use aocf::Aoc;

fn main() {
    let first_sum: u32 = Aoc::new().year(Some(2023)).day(Some(1)).init().unwrap().get_input(false).unwrap().split("\n").collect::<Vec<&str>>().iter().map(|line| line.chars().filter(|c| c.is_digit(10)).collect::<String>()).filter(|line| !line.is_empty()).map(|line| format!("{}{}", line.chars().next().unwrap(), line.chars().last().unwrap()).parse::<u32>().unwrap()).sum();
    let second_sum: u32 = Aoc::new().year(Some(2023)).day(Some(1)).init().unwrap().get_input(false).unwrap().split("\n").collect::<Vec<&str>>().iter().map(|line| (String::from(*line), line.replace("one", "o1e").replace("two", "t2o").replace("three", "t3e").replace("four", "f4r").replace("five", "f5e").replace("six", "s6x").replace("seven", "s7n").replace("eight", "e8t").replace("nine", "n9e"))).map(|(ori, line)| (ori, line.chars().filter(|c| c.is_digit(10)).collect::<String>())).filter(|(_, line)| !line.is_empty()).map(|(ori, line)| (ori, format!("{}{}", line.chars().next().unwrap(), line.chars().last().unwrap()).parse::<u32>().unwrap())).map(|(_, x)| x).sum();
    dbg!(first_sum, second_sum);
}
