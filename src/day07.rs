#[derive(Debug, Eq, PartialEq, Clone, Hash)]
enum Card {
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
        for card in cards {
            *counts.entry(card).or_insert(0) += 1;
        }

        let mut occurences: Vec<&i32> = counts.values().collect();
        occurences.sort();

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

#[derive(Eq, PartialEq, Debug)]
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

fn get_total_winnings(plays: &Plays) -> u32 {
    let mut winnings = 0;
    for (i, play) in plays.iter().enumerate() {
        let rank = i as u32 + 1;
        winnings += rank * play.bid;
    }
    winnings
}

fn main() {
    println!("Hello, World! from src/day07.rs!");
    let input: Vec<String> = aoc_input::get(2023, 7);

    let mut plays: Plays = vec![
        ("32T3K", "765"),
        ("T55J5", "684"),
        ("KK677", "28"),
        ("KTJJT", "220"),
        ("QQQJA", "483"),
    ]
    .iter()
    .map(|t| Play::from_tuple(t))
    .collect();
    plays.sort();
    dbg!(&plays);
    let total_winnings = get_total_winnings(&plays);
    dbg!(total_winnings);
    assert_eq!(
        total_winnings, 6440,
        "Example failed, this value should be 6440."
    );
}
