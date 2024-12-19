use std::collections::{HashMap, HashSet};

fn build_patterns(data: &str) -> HashSet<&str> {
    let mut patterns = HashSet::new();
    for p in data.split(", ") {
        patterns.insert(p);
    }
    patterns
}

fn count_designs<'a>(
    design: &'a str,
    patterns: &HashSet<&str>,
    cache: &mut HashMap<&'a str, usize>,
) -> usize {
    if let Some(cached) = cache.get(design) {
        return *cached;
    }

    if design.is_empty() {
        return 1;
    }

    let mut combinations = 0;
    for p in patterns {
        if design.starts_with(p) {
            let plen = p.len();
            if plen <= design.len() {
                combinations += count_designs(&design[plen..], patterns, cache);
            }
        }
    }

    cache.insert(design, combinations);
    combinations
}

fn main() {
    let data = include_str!("../input")
        .split("\n\n")
        .collect::<Vec<&str>>();

    let patterns = build_patterns(data[0]);
    let combinations = data[1]
        .lines()
        .map(|l| {
            let mut cache = HashMap::new();
            count_designs(l, &patterns, &mut cache)
        })
        .collect::<Vec<usize>>();

    let part_1 = combinations.iter().filter(|c| c > &&0).count();
    println!("Part 1: {part_1}");

    let part_2 = combinations.iter().sum::<usize>();
    println!("Part 2: {part_2}");
}
