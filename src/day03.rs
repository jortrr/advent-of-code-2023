mod macros;
mod regex_captures;

fn main() {
    let a = [
        vec![String::from("Begin")],
        aoc::get(2023, 3),
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
    let g = grid_of_chars![
        "467..114..",
        "...*......",
        "..35..633.",
        "......#...",
        "617*......",
        ".....+.58.",
        "..592.....",
        "......755.",
        "...$.*....",
        ".664.598..",
    ];
    let solve = |g: Vec<Vec<char>>| {
        let mut sum = 0;
        for y_1 in 0..g.len() {
            for x_1 in 0..g[0].len() {
                if g[y_1][x_1] == '*' {
                    let mut adj = Vec::new();
                    let mut bl = Vec::new();
                    for y_2 in y_1.saturating_sub(1)..y_1 + 2 {
                        for x_2 in x_1.saturating_sub(1)..x_1 + 2 {
                            if y_2 >= g.len() || x_2 >= g[0].len() || bl.contains(&(y_2, x_2)) {
                                continue;
                            }
                            if g[y_2][x_2].is_digit(10) {
                                let mut n = format!("{}", g[y_2][x_2]);
                                let mut c = 1;
                                while g[y_2][x_2.saturating_sub(c)].is_digit(10) && c <= x_2 {
                                    n = format!("{}{}", g[y_2][x_2.saturating_sub(c)], n);
                                    bl.push((y_2, x_2.saturating_sub(c)));
                                    c += 1;
                                }
                                c = 1;
                                while g[y_2][(x_2 + c).min(g[0].len() - 1)].is_digit(10)
                                    && x_2 + c < g[0].len()
                                {
                                    n = format!("{}{}", n, g[y_2][x_2 + c]);
                                    bl.push((y_2, x_2 + c));
                                    c += 1;
                                }
                                //dbg!(y_2, x_2, &n);
                                adj.push(n.parse::<i32>().unwrap());
                            }
                        }
                    }
                    //dbg!(&adj);
                    if adj.len() == 2 {
                        sum += adj[0] * adj[1];
                    }
                }
            }
        }
        sum
    };
    let sum = solve(g);
    test!(467835, sum);
    let sum = solve(aoc::grid(2023, 3));
    test!(86879020, sum);
}
