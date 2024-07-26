mod macros;

fn main() {
    let solution = aoc::get(2023, 1)
        .iter()
        .map(|line| {
            (vec![
                line.clone(),
                line.replace("one", "o1e")
                    .replace("two", "t2o")
                    .replace("three", "t3e")
                    .replace("four", "f4r")
                    .replace("five", "f5e")
                    .replace("six", "s6x")
                    .replace("seven", "s7n")
                    .replace("eight", "e8t")
                    .replace("nine", "n9e"),
            ])
        })
        .map(|lines| {
            lines
                .iter()
                .map(|line| line.chars().filter(|c| c.is_digit(10)).collect::<String>())
                .collect::<Vec<String>>()
        })
        .filter(|lines| !lines.iter().any(String::is_empty))
        .map(|lines| {
            lines
                .iter()
                .map(|line| {
                    format!(
                        "{}{}",
                        line.chars().next().unwrap(),
                        line.chars().last().unwrap()
                    )
                    .parse::<u32>()
                    .unwrap()
                })
                .collect::<Vec<u32>>()
        })
        .fold((0, 0), |acc: (u32, u32), lines| {
            (acc.0 + lines[0], acc.1 + lines[1])
        });

    println!("Solution: {:?}", solution);
    test!(55386, solution.0);
    test!(54824, solution.1);
}
