use core::panic;

mod macros;
mod problem;
use problem::*;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
enum Card {
    Joker,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
    T,
    J,
    Q,
    K,
    A,
}

impl Card {
    fn from_char(c: char) -> Option<Card> {
        match c {
            'A' => Some(Card::A),
            'K' => Some(Card::K),
            'Q' => Some(Card::Q),
            'J' => Some(Card::J),
            'T' => Some(Card::T),
            '9' => Some(Card::_9),
            '8' => Some(Card::_8),
            '7' => Some(Card::_7),
            '6' => Some(Card::_6),
            '5' => Some(Card::_5),
            '4' => Some(Card::_4),
            '3' => Some(Card::_3),
            '2' => Some(Card::_2),
            _ => None,
        }
    }

    fn order_value(&self) -> u8 {
        return self.clone() as u8;
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.order_value().cmp(&other.order_value())
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

type Cards = [Card; 5];

fn cards_from_string(cards: String) -> Option<Cards> {
    if cards.len() == 5 {
        let mut result: Cards = [Card::_2, Card::_2, Card::_2, Card::_2, Card::_2];
        for (i, c) in cards.chars().enumerate() {
            let card = Card::from_char(c).unwrap();
            result[i] = card;
        }
        return Some(result);
    }

    return None;
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum CardsType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl CardsType {
    fn from_cards(cards: &Cards) -> Option<CardsType> {
        use std::collections::HashMap;
        use CardsType::*;
        let mut counts = HashMap::new();
        // Count the occurences of each card
        cards
            .iter()
            .filter(|&card| *card != Card::Joker)
            .for_each(|card| {
                *counts.entry(card).or_insert(0) += 1;
            });

        let mut occurences: Vec<i32> = counts.values().map(|x| *x).collect();
        occurences.sort();

        //Account for Jokers, by letting Jokers contribute to the best possible CardType
        let total: i32 = occurences.iter().sum();
        if (total as usize) < cards.len() {
            // There are Jokers
            match occurences.len() {
                0 => occurences.push(5),
                1 => occurences[0] = 5,
                2 => {
                    occurences[1] = match occurences[0] {
                        1 => 4,
                        2 => 3,
                        _ => panic!("This should never happen."),
                    };
                }
                3 => {
                    occurences[2] = match occurences[1] {
                        1 => 3,
                        2 => 2,
                        _ => panic!("This should never happen."),
                    };
                }
                4 => occurences[3] = 2,
                _ => panic!("This is impossible."),
            }
        }

        return match &occurences[..] {
            [5] => Some(FiveOfAKind),
            [1, 4] => Some(FourOfAKind),
            [2, 3] => Some(FullHouse),
            [1, 1, 3] => Some(ThreeOfAKind),
            [1, 2, 2] => Some(TwoPair),
            [1, 1, 1, 2] => Some(OnePair),
            [1, 1, 1, 1, 1] => Some(HighCard),
            _ => None,
        };
    }

    fn order_value(&self) -> u8 {
        return self.clone() as u8;
    }
}

impl Ord for CardsType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.order_value().cmp(&other.order_value())
    }
}

impl PartialOrd for CardsType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct Hand {
    to_string: String,
    cards: Cards,
    cards_type: CardsType,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering::*;
        let type_order = self.cards_type.cmp(&other.cards_type);
        return match type_order {
            Equal => {
                for i in 0..self.cards.len() {
                    let a = self.cards.get(i).unwrap();
                    let b = other.cards.get(i).unwrap();
                    let card_order = a.cmp(b);
                    match card_order {
                        Equal => continue,
                        _ => return card_order,
                    }
                }
                return Equal;
            }
            _ => type_order,
        };
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

impl Hand {
    fn from_string(hand: &str) -> Hand {
        let cards: Cards = cards_from_string(String::from(hand)).unwrap();
        let cards_type: CardsType = CardsType::from_cards(&cards).unwrap();
        Hand {
            to_string: String::from(hand),
            cards,
            cards_type,
        }
    }

    fn replace_all_j_with_joker(&self) -> Hand {
        let mut result: Hand = self.clone();
        result.cards.iter_mut().for_each(|card| {
            if *card == Card::J {
                *card = Card::Joker;
            }
        });
        result.cards_type = CardsType::from_cards(&result.cards).unwrap();
        result
    }
}

#[derive(Eq, PartialEq, Debug)]
struct Play {
    hand: Hand,
    bid: u32,
}

impl Ord for Play {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.hand.cmp(&other.hand)
    }
}

impl PartialOrd for Play {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

impl Play {
    fn from_tuple(play: &(&str, &str)) -> Play {
        let (cards, bid) = play;
        let hand = Hand::from_string(cards);
        let bid = bid.to_string().parse::<u32>().unwrap();
        Play { hand, bid }
    }
}

type Plays = Vec<Play>;

impl Parse for Plays {
    fn parse(input: Input) -> Self {
        input
            .lines()
            .map(|s| s.split_ascii_whitespace().collect::<Vec<_>>())
            .filter(|v| v.len() == 2)
            .map(|t| (t[0], t[1]))
            .map(|t| Play::from_tuple(&t))
            .collect()
    }
}

fn get_total_winnings(plays: &Plays) -> u32 {
    let mut winnings = 0;
    for (i, play) in plays.iter().enumerate() {
        let rank = i as u32 + 1;
        winnings += rank * play.bid;
    }
    winnings
}

struct DaySeven {}

impl Problem for DaySeven {
    const YEAR: Year = 2023;
    const DAY: Day = 7;
    const PART_ONE_EXAMPLE_EXPECTED: Answer = 6440;
    const PART_ONE_EXPECTED: Answer = 251806792;
    const PART_TWO_EXAMPLE_EXPECTED: Answer = 5905;
    const PART_TWO_EXPECTED: Answer = 252113488;

    fn example_input() -> ExampleInput {
        "
        32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483
        "
    }

    fn solve_part_one(input: Input, _is_example: bool) -> Answer {
        let mut plays = Plays::parse(input);
        plays.sort();
        let total_winnings = get_total_winnings(&plays);
        total_winnings as Answer
    }

    fn solve_part_two(input: Input, _is_example: bool) -> Answer {
        let mut plays = Plays::parse(input);
        plays = plays
            .iter()
            .map(|play| Play {
                bid: play.bid,
                hand: play.hand.replace_all_j_with_joker(),
            })
            .collect();
        plays.sort();
        let total_winnings_with_jokers = get_total_winnings(&plays);
        total_winnings_with_jokers as Answer
    }
}

run!(DaySeven);
