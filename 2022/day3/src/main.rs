use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn prepare_alphabet_lookup() -> HashMap<char, i32> {
    let alphabet = String::from_utf8((b'a'..=b'z').chain(b'A'..=b'Z').collect()).unwrap();
    let other: Vec<i32> = (0..alphabet.len() as i32).collect();
    alphabet.chars().zip(other).into_iter().collect()
}

fn compare_sets(sets: Vec<HashSet<char>>) -> i32 {
    let alphabet = prepare_alphabet_lookup();
    sets.into_iter()
        .reduce(|a, b| a.intersection(&b).cloned().collect())
        .unwrap()
        .iter()
        .map(|c| alphabet.get(c).unwrap() + 1)
        .sum()
}

fn find_misplaced_and_prioritize(data: &str) -> i32 {
    data.lines()
        .map(|line| {
            let ab = line.split_at(line.len() / 2);
            let sets: Vec<HashSet<char>> = vec![
                HashSet::from_iter(ab.0.chars()),
                HashSet::from_iter(ab.1.chars()),
            ];
            compare_sets(sets)
        })
        .sum()
}

fn find_badges_and_prioritize(data: &str) -> i32 {
    data.lines()
        .chunks(3)
        .into_iter()
        .map(|grp| {
            let sets = grp.map(|g| HashSet::from_iter(g.chars())).collect();
            compare_sets(sets)
        })
        .sum()
}

fn main() {
    let data = include_str!("../input");
    println!("Part1: {}", find_misplaced_and_prioritize(data));
    println!("Part2: {}", find_badges_and_prioritize(data));
}
