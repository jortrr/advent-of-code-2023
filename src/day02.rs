mod regex_captures;

fn main() {
    let x = aoc_input::get(2023, 2)
        .iter()
        .filter(|line| !line.is_empty())
        .map(|line| {
            (
                regex_captures::get(r"Game\s(\d+)", &line.clone())
                    .iter()
                    .next()
                    .unwrap()[1]
                    .parse::<u32>()
                    .unwrap(),
                line,
            )
        })
        .map(|(id, line)| {
            (
                regex_captures::get(r"(\d+)\s(\w+)", &line)
                    .iter()
                    .map(|cap| (*&cap[1].parse::<u32>().unwrap(), String::from(&cap[2])))
                    .all(|(value, color)| {
                        (color == "red" && value <= 12)
                            || (color == "green" && value <= 13)
                            || (color == "blue" && value <= 14)
                    })
                    .then(|| id),
                line,
            )
        })
        .map(|(id, line)| {
            let mut max_red = 0;
            let mut max_green = 0;
            let mut max_blue = 0;
            regex_captures::get(r"(\d+)\s(\w+)", &line)
                .iter()
                .map(|cap| (*&cap[1].parse::<u32>().unwrap(), String::from(&cap[2])))
                .for_each(|(value, color)| {
                    if color == "red" {
                        max_red = std::cmp::max(value, max_red);
                    } else if color == "green" {
                        max_green = std::cmp::max(value, max_green);
                    } else if color == "blue" {
                        max_blue = std::cmp::max(value, max_blue);
                    }
                });
            (id, line, max_red, max_green, max_blue)
        })
        .fold((0, 0), |acc: (u32, u32), tuple| {
            (
                acc.0 + tuple.0.unwrap_or(0),
                acc.1 + (tuple.2 * tuple.3 * tuple.4),
            )
        });

    dbg!(x);
}
