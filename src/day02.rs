fn main() {
    println!("Solution: {}", aoc_input::get(2023, 2).into_iter().filter(|line| regex::Regex::new(r"(\d+)\s(\w+)").unwrap().captures_iter(&line).map(|cap| (*&cap[1].parse::<u32>().unwrap(), String::from(&cap[2]))).all(|(value, color)| (color == "red" && value <= 12) || (color == "green" && value <= 13) || (color == "blue" && value <= 14))).filter(|line| !line.is_empty()).map(|line| (line.clone(), regex::Regex::new(r"Game\s(\d+)").unwrap().captures_iter(&line).next().unwrap()[1].parse::<u32>().unwrap())).map(|(_, id)| id).sum::<u32>());
}
