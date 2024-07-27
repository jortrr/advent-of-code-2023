use colored::Colorize;

mod macros;

fn main() {
    let e = vec_of_strings![
        "seeds: 79 14 55 13",
        "",
        "seed-to-soil map:",
        "50 98 2",
        "52 50 48",
        "",
        "soil-to-fertilizer map:",
        "0 15 37",
        "37 52 2",
        "39 0 15",
        "",
        "fertilizer-to-water map:",
        "49 53 8",
        "0 11 42",
        "42 0 7",
        "57 7 4",
        "",
        "water-to-light map:",
        "88 18 7",
        "18 25 70",
        "",
        "light-to-temperature map:",
        "45 77 23",
        "81 45 19",
        "68 64 13",
        "",
        "temperature-to-humidity map:",
        "0 69 1",
        "1 0 69",
        "",
        "humidity-to-location map:",
        "60 56 37",
        "56 93 4",
    ];
    let mut a: Vec<Vec<_>> = Vec::new();
    a.push(Vec::new());
    let mut c = 0;
    for l in e {
        if l.contains("seeds:") {
            let i: Vec<_> = l[6..]
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect();
            for x in i {
                a[c].push(((x, x + 0), (x, x + 0)));
            }
        } else if l.contains("map:") {
            a.push(Vec::new());
            c += 1;
        } else if l.is_empty() {
            continue;
        } else {
            let i: Vec<_> = l
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect();

            a[c].push(((i[1], i[1] + i[2]), (i[0], i[0] + i[2])));
        }
    }
    dbg!(&a);
    let cs = vec!["red", "green", "blue", "yellow"];
    for y in 0..a.len() {
        for x in 0..100 {
            let mut c = String::from(".");
            for (i, t) in a[y].iter().enumerate() {
                if x >= t.0 .0 && x < t.0 .1 {
                    c = if x == t.0 .0 || x == t.0 .1 - 1 {
                        "$"
                    } else {
                        "+"
                    }
                    .color(cs[i])
                    .to_string();
                } else if t.0 .0 == t.0 .1 && t.0 .1 == x {
                    c = "$".color(cs[i]).to_string();
                }
            }
            print!("{}", c);
        }
        println!();
        for x in 0..100 {
            let mut c = String::from(".");
            for (i, t) in a[y].iter().enumerate() {
                if x >= t.1 .0 && x <= t.1 .1 {
                    c = "*".color(cs[i]).to_string();
                }
            }
            print!("{}", c);
        }
        println!();
        println!();
    }
}
