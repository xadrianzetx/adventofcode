use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

impl HandType {
    fn from_cards(value: &str, part2: bool) -> Self {
        let mut card_counts = HashMap::new();
        value.chars().for_each(|char| {
            card_counts
                .entry(char)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        });

        if part2 {
            return Self::map_with_jokers(&mut card_counts);
        }
        Self::map_without_jokers(&card_counts)
    }

    fn map_without_jokers(card_counts: &HashMap<char, i32>) -> HandType {
        let highest = card_counts.values().max().unwrap();
        let n_cards = card_counts.len();

        match (highest, n_cards) {
            (5, _) => Self::FiveKind,
            (4, _) => Self::FourKind,
            (3, 2) => Self::FullHouse,
            (3, 3) => Self::ThreeKind,
            (2, 3) => Self::TwoPair,
            (2, 4) => Self::OnePair,
            _ => Self::HighCard,
        }
    }

    fn map_with_jokers(card_counts: &mut HashMap<char, i32>) -> HandType {
        let n_jokers = card_counts.remove(&'J').unwrap_or(0);

        if n_jokers == 0 {
            return Self::map_without_jokers(card_counts);
        }

        let highest = card_counts.values().max().unwrap_or(&0);
        let n_remaining = card_counts.len();

        match (highest + n_jokers, n_remaining) {
            (5, _) => Self::FiveKind,
            (4, _) => Self::FourKind,
            (3, 2) => Self::FullHouse,
            (3, 3) => Self::ThreeKind,
            _ => Self::OnePair,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    // T: 10, J: 11, Q: 12, K: 13, A: 14, 9: 9, ...
    cards: Vec<usize>,
    bid: usize,
    type_: HandType,
}

impl Hand {
    fn from_cards(value: &str, part2: bool) -> Self {
        let values = value.split(' ').collect::<Vec<&str>>();
        let bid = values.last().unwrap().parse::<usize>().unwrap();

        let cards = values.first().unwrap().to_owned();
        let hand_type = HandType::from_cards(cards, part2);
        let parsed_cards = parse_cards(cards, part2);

        Hand {
            cards: parsed_cards,
            bid,
            type_: hand_type,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let type_ordering = self.type_.cmp(&other.type_);
        match type_ordering {
            Ordering::Equal => self.cards.cmp(&other.cards),
            _ => type_ordering,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_cards(cards: &str, part2: bool) -> Vec<usize> {
    let mut parsed = Vec::new();
    for card in cards.chars() {
        if let Some(value) = card.to_digit(10) {
            parsed.push(value as usize);
            continue;
        }

        match card {
            'T' => parsed.push(10),
            'J' => {
                if part2 {
                    parsed.push(1)
                } else {
                    parsed.push(11)
                }
            }
            'Q' => parsed.push(12),
            'K' => parsed.push(13),
            'A' => parsed.push(14),
            _ => unreachable!(),
        }
    }

    parsed
}

fn total_winnings(lines: &[&str], part2: bool) -> usize {
    let mut hands = lines
        .iter()
        .map(|cards| Hand::from_cards(cards, part2))
        .collect::<Vec<Hand>>();

    hands.sort();

    hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| hand.bid * (rank + 1))
        .sum()
}

fn main() {
    let lines = include_str!("../input").lines().collect::<Vec<&str>>();
    println!("Part 1: {}", total_winnings(&lines, false));
    println!("Part 2: {}", total_winnings(&lines, true));
}
