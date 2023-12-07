use std::cmp::{max, min, Ordering};
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

type Mapper = dyn Fn(usize, usize, usize) -> HandType;

impl HandType {
    fn from_mapped_cards(value: &str, mapper: &Mapper) -> Self {
        let mut card_counts = HashMap::new();
        value.chars().for_each(|char| {
            card_counts
                .entry(char)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        });

        let n_jokers = card_counts.remove(&'J').unwrap_or(0);
        let highest = card_counts.values().max().unwrap_or(&0).to_owned();
        let n_cards = card_counts.len();
        mapper(highest, n_cards, n_jokers)
    }
}

fn regular_rules(highest: usize, n_cards: usize, n_jokers: usize) -> HandType {
    match (max(highest, n_jokers), n_cards + min(1, n_jokers)) {
        (5, _) => HandType::FiveKind,
        (4, _) => HandType::FourKind,
        (3, 2) => HandType::FullHouse,
        (3, 3) => HandType::ThreeKind,
        (2, 3) => HandType::TwoPair,
        (2, 4) => HandType::OnePair,
        _ => HandType::HighCard,
    }
}

fn joker_rules(highest: usize, n_cards: usize, n_jokers: usize) -> HandType {
    match (highest + n_jokers, n_cards, n_jokers) {
        (5, _, _) => HandType::FiveKind,
        (4, _, _) => HandType::FourKind,
        (3, 2, _) => HandType::FullHouse,
        (3, 3, _) => HandType::ThreeKind,
        (2, 3, 0) => HandType::TwoPair,
        (2, 4, 0) => HandType::OnePair,
        (_, _, 1..=5) => HandType::OnePair,
        (_, _, 0) => HandType::HighCard,
        _ => unreachable!(),
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    // T: 10, J: 11 | 1, Q: 12, K: 13, A: 14, 9: 9, ...
    cards: Vec<usize>,
    bid: usize,
    type_: HandType,
}

impl Hand {
    fn from_mapped_cards(value: &str, mapper: &Mapper) -> Self {
        let values = value.split(' ').collect::<Vec<&str>>();
        let bid = values.last().unwrap().parse::<usize>().unwrap();

        let cards = values.first().unwrap().to_owned();
        let hand_type = HandType::from_mapped_cards(cards, mapper);

        // Query mapper to see what value needs to represent a Joker. Kinda hacky.
        let parsed_cards = match mapper(0, 0, 1) {
            HandType::HighCard => parse_cards(cards, 11),
            HandType::OnePair => parse_cards(cards, 1),
            _ => unreachable!(),
        };

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

fn parse_cards(cards: &str, joker_value: usize) -> Vec<usize> {
    let mut parsed = Vec::new();
    for card in cards.chars() {
        match card {
            'T' => parsed.push(10),
            'J' => parsed.push(joker_value),
            'Q' => parsed.push(12),
            'K' => parsed.push(13),
            'A' => parsed.push(14),
            '2'..='9' => parsed.push(card.to_digit(10).unwrap() as usize),
            _ => unreachable!(),
        }
    }

    parsed
}

fn total_winnings(lines: &[&str], mapper: &Mapper) -> usize {
    let mut hands = lines
        .iter()
        .map(|cards| Hand::from_mapped_cards(cards, mapper))
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
    println!("Part 1: {}", total_winnings(&lines, &regular_rules));
    println!("Part 2: {}", total_winnings(&lines, &joker_rules));
}
