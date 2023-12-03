mod regex_captures;

fn main() {
    println!("Solution: {}", aoc_input::get(2023, 2).iter().filter(|line| regex_captures::get(r"(\d+)\s(\w+)", &line).iter().map(|cap| (*&cap[1].parse::<u32>().unwrap(), String::from(&cap[2]))).all(|(value, color)| (color == "red" && value <= 12) || (color == "green" && value <= 13) || (color == "blue" && value <= 14))).filter(|line| !line.is_empty()).map(|line| (line.clone(), regex_captures::get(r"Game\s(\d+)", &line).iter().next().unwrap()[1].parse::<u32>().unwrap())).map(|(_, id)| id).sum::<u32>());
}
