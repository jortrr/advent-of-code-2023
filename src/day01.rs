use aocf::Aoc;

fn main() {
    println!("Solution: {:?}", Aoc::new().year(Some(2023)).day(Some(1)).init().unwrap().get_input(false).unwrap().split("\n").collect::<Vec<&str>>().iter().map(|line| (String::from(*line), String::from(*line), line.replace("one", "o1e").replace("two", "t2o").replace("three", "t3e").replace("four", "f4r").replace("five", "f5e").replace("six", "s6x").replace("seven", "s7n").replace("eight", "e8t").replace("nine", "n9e"))).map(|(ori, line1, line2)| (ori, line1.chars().filter(|c| c.is_digit(10)).collect::<String>(), line2.chars().filter(|c| c.is_digit(10)).collect::<String>())).filter(|(_, line1, _)| !line1.is_empty()).map(|(ori, line1, line2)| (ori, format!("{}{}", line1.chars().next().unwrap(), line1.chars().last().unwrap()).parse::<u32>().unwrap(), format!("{}{}", line2.chars().next().unwrap(), line2.chars().last().unwrap()).parse::<u32>().unwrap())).fold((0, 0), |acc: (u32, u32), tuple| (acc.0 + tuple.1, acc.1 + tuple.2)));
}
