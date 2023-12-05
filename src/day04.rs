mod regex_captures;

fn main() {
    #[rustfmt::skip]
    println!("Solution: {:?}", aoc_input::get(2023, 4).iter().filter(|s| !s.is_empty()).map(|line| line.split(|c| c == ':' || c == '|').collect::<Vec<&str>>()).map(|parts| { (parts[0], vec![parts[1], parts[2]].iter().map(|s| { s.split(' ').filter(|w| !w.is_empty()).map(|w| String::from(w)).collect::<Vec<String>>() }).collect::<Vec<_>>(),) }).map(|(card, nums)| { (card, nums[0].clone(), nums[1].clone(), nums[0].iter().filter(|value| nums[1].contains(value)).map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>(),) }).enumerate().map(|(i, (_, _, _, wins))| {if wins.len() == 0 { (i as u32 + 1, 0, Vec::<u32>::new())} else { (i as u32 + 1, (2 as u32).pow((wins.len() - 1) as u32), ((i as u32 + 2)..=(i as u32 + 1) + (wins.len() as u32)).collect())}})
            .fold(((0, 0), std::collections::HashMap::<u32, u32>::new()), |mut acc, (card, points, copies)| {
                let copies_of_this_card = *acc.1.get(&card).unwrap_or(&1);
                println!("Processing card {}({}) , won: {:?}", card, copies_of_this_card, copies);
                println!("Total amount of cards: {}", acc.0 .1);
                copies.iter().for_each(|card_won| {
                    let card_won = *card_won;
                    let old = acc.1.get(&card_won).unwrap_or(&1);
                    let new = old + copies_of_this_card;
                    println!("  Adding {} copies: card {}({}) -> card {}({})", copies_of_this_card, card_won, old, card_won, new);
                    acc.1.insert(card_won, new);
                });
                ((acc.0 .0 + points, acc.0 .1 + copies_of_this_card), acc.1)
            }).0);
}
