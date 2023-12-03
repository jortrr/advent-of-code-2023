mod regex_captures;

fn main() {
    println!("Solution: {:?}", aoc_input::get(2023, 2).iter().filter(|line| !line.is_empty()).map(|line| (regex_captures::get(r"Game\s(\d+)", &line.clone()).iter().next().unwrap()[1].parse::<u32>().unwrap(), line,)).map(|(id, line)| (regex_captures::get(r"(\d+)\s(\w+)", &line).iter().map(|cap| (*&cap[1].parse::<u32>().unwrap(), String::from(&cap[2]))).all(|(value, color)| { (color == "red" && value <= 12) || (color == "green" && value <= 13) || (color == "blue" && value <= 14) }).then(|| id), line,)).map(|(id, line)| (id, line, vec!["red", "green", "blue"].iter().map(|color| regex_captures::get(&format!(r"(\d+)\s{}", color), &line).iter().map(|cap| *&cap[1].parse::<u32>().unwrap()).max().unwrap()).collect::<Vec<u32>>())).fold((0, 0), |acc: (u32, u32), (id, _, rgb)| { (acc.0 + id.unwrap_or(0), acc.1 + (rgb[0] * rgb[1] * rgb[2]),) }));
}
