use aocf::Aoc;
use regex::Regex;

fn main() {
    let sum = Aoc::new().year(Some(2023)).day(Some(2)).init().unwrap().get_input(false).split("\n").collect::<Vec<&str>>().iter().filter(|line|Regex::new(r"(\d+)\sblue").unwrap().captures_iter(line).any(|cap|&cap[1].parse::<u32> > 12)).map(|line|Regex::new(r"Game\s(\d+)").unwrap() //TODO
}
