use std::collections::HashSet;

#[derive(Debug)]
struct Card {
    winning: HashSet<usize>,
    had: HashSet<usize>,
}

impl From<&str> for Card {
    fn from(value: &str) -> Self {
        let card = value.split(':').collect::<Vec<&str>>();
        let numbers = card[1]
            .split('|')
            .map(|list| {
                list.split(' ')
                    .filter_map(|number| number.parse::<usize>().ok())
                    .collect::<HashSet<usize>>()
            })
            .collect::<Vec<HashSet<usize>>>();

        Card {
            winning: numbers.first().unwrap().clone(),
            had: numbers.last().unwrap().clone(),
        }
    }
}

impl Card {
    fn count_winning(&self) -> usize {
        self.winning.intersection(&self.had).count()
    }

    fn count_points(&self) -> usize {
        let hits = self.count_winning();
        if hits == 0 {
            return 0;
        }
        2_usize.pow((hits - 1) as u32)
    }
}

fn count_scratchcards(cards: &Vec<Card>) -> usize {
    let mut counts = vec![1; cards.len()];

    for id in 0..cards.len() {
        let n_winning = cards[id].count_winning();
        let count = counts[id];
        counts
            .iter_mut()
            .skip(id + 1)
            .take(n_winning)
            .for_each(|item| *item += count)
    }

    counts.iter().sum()
}

fn main() {
    let cards = include_str!("../input")
        .lines()
        .map(Card::from)
        .collect::<Vec<Card>>();

    let part_1 = cards.iter().map(|card| card.count_points()).sum::<usize>();
    println!("Part 1: {part_1}");
    println!("Part 2: {}", count_scratchcards(&cards))
}
