use regex::Regex;

fn main() {
    let x: u32 = aoc_input::get(2023, 2)
        .into_iter()
        .filter(|line| {
            Regex::new(r"(\d+)\s(\w+)")
                .unwrap()
                .captures_iter(&line)
                .all(|cap| {
                    let value = *&cap[1].parse::<u32>().unwrap();
                    let color = &cap[2];
                    (color == "red" && value <= 12)
                        || (color == "green" && value <= 13)
                        || (color == "blue" && value <= 14)
                })
        })
        .filter(|line| !line.is_empty())
        .map(|line| {
            (
                line.clone(),
                Regex::new(r"Game\s(\d+)")
                    .unwrap()
                    .captures_iter(&line)
                    .next()
                    .unwrap()[1]
                    .parse::<u32>()
                    .unwrap(),
            )
        })
        .map(|(_, id)| id)
        .sum();
    dbg!(x);
}
