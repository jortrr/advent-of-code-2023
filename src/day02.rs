mod macros;
mod regex_captures;

fn main() {
    let solution = aoc_input::get(2023, 2)
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
                id,
                line,
                vec!["red", "green", "blue"]
                    .iter()
                    .map(|color| {
                        regex_captures::get(&format!(r"(\d+)\s{}", color), &line)
                            .iter()
                            .map(|cap| *&cap[1].parse::<u32>().unwrap())
                            .max()
                            .unwrap()
                    })
                    .collect::<Vec<u32>>(),
            )
        })
        .map(|(id, line, rgb)| {
            (
                (rgb[0] <= 12 && rgb[1] <= 13 && rgb[2] <= 14).then(|| id),
                line,
                rgb,
            )
        })
        .fold((0, 0), |acc: (u32, u32), (id, _, rgb)| {
            (acc.0 + id.unwrap_or(0), acc.1 + (rgb[0] * rgb[1] * rgb[2]))
        });

    println!("Solution: {:?}", solution);
    test!(1867, solution.0);
    test!(84538, solution.1);
}
