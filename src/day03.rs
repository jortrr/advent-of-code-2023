use crate::*;

/// The amount of part numbers a symbol needs to be adjacent to
enum Adjacent {
    Any,
    Two,
}

/// Find the sum of part numbers from a grid of chars where adjacent holds for each symbol
fn solve(g: Grid<char>, adjacent: Adjacent) -> Int {
    let mut sum = 0;
    for y_1 in 0..g.len() {
        for x_1 in 0..g[0].len() {
            let current = g[y_1][x_1];
            let is_symbol = match adjacent {
                Adjacent::Any => !current.is_digit(10) && current != '.',
                Adjacent::Two => current == '*',
            };
            if is_symbol {
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
                            adj.push(n.parse::<Int>().unwrap());
                        }
                    }
                }
                sum += match adjacent {
                    Adjacent::Any => adj.iter().sum(),
                    Adjacent::Two => {
                        if adj.len() == 2 {
                            adj[0] * adj[1]
                        } else {
                            0
                        }
                    }
                };
            }
        }
    }
    sum
}

pub struct DayThree {}

impl Problem for DayThree {
    fn year(&self) -> Year {
        2023
    }
    fn day(&self) -> Day {
        3
    }
    fn expect_part_one(&self) -> Answer {
        540131
    }
    fn expect_part_two(&self) -> Answer {
        86879020
    }

    define_examples! {
        (
            "
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
            ",
            Expect::PartsOneAndTwo(4361, 467835),
        )
    }

    fn solve_part_one(&self, input: Input, _is_example: bool) -> Answer {
        let grid: Grid<char> = InputLines::from(input).into();
        solve(grid, Adjacent::Any)
    }

    fn solve_part_two(&self, input: Input, _is_example: bool) -> Answer {
        let grid: Grid<char> = InputLines::from(input).into();
        solve(grid, Adjacent::Two)
    }
}
