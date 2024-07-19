mod macros;
mod regex_captures;

fn main() {
    let a = [
        vec![String::from("Begin")],
        aoc_input::get(2023, 3),
        vec!["".to_string()],
    ]
    .concat();
    dbg!(&a);
    let b: Vec<_> = a
        .windows(3)
        .map(|window| (window, regex_captures::get(r"(\d+)", &window[1])))
        .collect();
    //dbg!(&b);
    let c: Vec<_> = b
        .iter()
        .map(|(windows, captures)| {
            (
                windows,
                captures
                    .iter()
                    .map(|cap| cap.get(1).unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .collect();
    //dbg!(&c);
    let d: Vec<_> = c
        .iter()
        .map(|(windows, captures)| {
            (
                windows,
                captures
                    .iter()
                    .filter(|m| {
                        for i in m.start()..(m.end() + 2) {
                            for j in 0..3 {
                                let index = if i > 0 { i - 1 } else { i };
                                let token = windows[j].chars().nth(index).unwrap_or('.');
                                //println!("i: {}, j: {}, token: {}", i, j, token);
                                if !token.is_digit(10) && token != '.' {
                                    return true;
                                }
                            }
                        }
                        false
                    })
                    .map(|m| m.as_str().parse::<u32>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .collect();
    //dbg!(&d);
    let sum = d
        .iter()
        .map(|(_, numbers)| numbers.iter().sum::<u32>())
        .sum::<u32>();
    // Part 1
    test!(540131, sum);
    // Part 2
    //test!(false, "Part 2");
}
