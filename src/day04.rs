mod problem;

use nom::character::complete::{space0, space1};
use problem::*;

struct Card {
    id: Int,
    wins: Vec<Int>,
    numbers: Vec<Int>,
}

impl Parse for Card {
    fn parse(input: Input) -> Self {
        Card::parse_card(&input).unwrap().1
    }
}

impl Card {
    /// Parse a single Card
    fn parse_card(input: &str) -> IResult<&str, Card> {
        let (rest, id) = preceded(
            tuple((tag("Card"), space1)),
            terminated(parse_num, tuple((tag(":"), space1))),
        )(input)?;
        let parse_numbers = |input| separated_list1(space1, parse_num)(input);
        let (rest, wins) = parse_numbers(rest)?;
        let (rest, _) = tuple((space0, tag("|"), space0))(rest)?;
        let (rest, numbers) = parse_numbers(rest)?;
        Ok((rest, Card { id, wins, numbers }))
    }

    /// Get amount of winning numbers
    fn get_matches(&self) -> Int {
        self.numbers
            .iter()
            .filter(|x| self.wins.contains(x))
            .count() as Int
    }

    /// Get the value of this Card, which is 2.pow(self.get_matches()-1) or 0
    fn get_value(&self) -> Int {
        let matches = self.get_matches();
        if matches > 0 {
            (2 as Int).pow((matches - 1).try_into().unwrap())
        } else {
            0
        }
    }
}

struct DayFour {}

impl Problem for DayFour {
    const YEAR: Year = 2023;
    const DAY: Day = 4;
    const PART_ONE_EXAMPLE_EXPECTED: Answer = 13;
    const PART_ONE_EXPECTED: Answer = 20667;
    const PART_TWO_EXAMPLE_EXPECTED: Answer = 30;
    const PART_TWO_EXPECTED: Answer = 5833065;

    fn example_input() -> ExampleInput {
        "
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        "
    }

    fn solve_part_one(input: Input, _is_example: bool) -> Answer {
        let lines: Vec<String> = InputLines::from(input).into();
        let cards: Vec<Card> = lines.into_iter().map(Card::parse).collect();
        let total = cards.iter().map(Card::get_value).sum();
        total
    }

    fn solve_part_two(input: Input, _is_example: bool) -> Answer {
        let lines: Vec<String> = InputLines::from(input).into();
        let cards: Vec<Card> = lines.into_iter().map(Card::parse).collect();
        let mut card_count: HashMap<Int, Int> = HashMap::new();
        for card in &cards {
            let count = *card_count.entry(card.id).or_insert(1);
            for i in 0..card.get_matches() {
                let won_card = card.id + i + 1;
                if won_card as usize <= cards.len() {
                    *card_count.entry(won_card).or_insert(1) += count;
                }
            }
        }
        let total = card_count.values().sum();
        total
    }
}

run!(DayFour);
