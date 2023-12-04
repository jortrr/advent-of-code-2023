mod regex_captures;

fn main() {
    // println!(
    //   "Solution: {:?}",
    let x = aoc_input::get(2023, 4)
        .iter()
        .filter(|s| !s.is_empty())
        .map(|line| line.split(|c| c == ':' || c == '|').collect::<Vec<&str>>())
        .map(|parts| {
            (
                parts[0],
                vec![parts[1], parts[2]]
                    .iter()
                    .map(|s| {
                        s.split(' ')
                            .filter(|w| !w.is_empty())
                            .map(|w| String::from(w))
                            .collect::<Vec<String>>()
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .map(|(card, nums)| {
            (
                card.split_ascii_whitespace()
                    .next_back()
                    .unwrap()
                    .parse::<u32>()
                    .unwrap(),
                nums[0].clone(),
                nums[1].clone(),
                nums[0]
                    .iter()
                    .filter(|value| nums[1].contains(value))
                    .map(|s| s.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>(),
            )
        })
        .map(|(card, win_values, has_values, wins)| {
            (
                card,
                win_values,
                has_values,
                if wins.len() == 0 {
                    0
                } else {
                    (2 as u32).pow((wins.len() - 1) as u32)
                },
                if wins.len() == 0 {
                    0
                } else {
                    (wins.len() - 1) as u32
                },
            )
        })
        .for_each(|f| println!("{:?}", f));
    //.sum::<u32>()
    //);
}
