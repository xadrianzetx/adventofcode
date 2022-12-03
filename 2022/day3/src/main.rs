use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn prepare_alphabet_lookup() -> HashMap<char, u32> {
    let alphabet = String::from_utf8((b'a'..=b'z').chain(b'A'..=b'Z').collect()).unwrap();
    let priorities: Vec<u32> = (0..alphabet.len() as u32).collect();
    alphabet.chars().zip(priorities).into_iter().collect()
}

fn find_common_items(items: Vec<HashSet<char>>) -> u32 {
    let alphabet = prepare_alphabet_lookup();
    items
        .into_iter()
        .reduce(|a, b| a.intersection(&b).cloned().collect())
        .unwrap()
        .iter()
        .map(|c| alphabet.get(c).unwrap() + 1)
        .sum()
}

fn find_misplaced_and_prioritize(data: &str) -> u32 {
    data.lines()
        .map(|line| {
            let compartments = line.split_at(line.len() / 2);
            let items: Vec<HashSet<char>> = vec![
                HashSet::from_iter(compartments.0.chars()),
                HashSet::from_iter(compartments.1.chars()),
            ];
            find_common_items(items)
        })
        .sum()
}

fn find_badges_and_prioritize(data: &str) -> u32 {
    data.lines()
        .chunks(3)
        .into_iter()
        .map(|grp| {
            let items = grp.map(|c| HashSet::from_iter(c.chars())).collect();
            find_common_items(items)
        })
        .sum()
}

fn main() {
    let data = include_str!("../input");
    println!("Part1: {}", find_misplaced_and_prioritize(data));
    println!("Part2: {}", find_badges_and_prioritize(data));
}
