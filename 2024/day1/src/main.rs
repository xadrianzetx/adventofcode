use std::collections::HashMap;

fn count_frequencies(items: &[i32]) -> HashMap<i32, i32> {
    let mut map = HashMap::new();
    items.iter().for_each(|item| {
        map.entry(*item).and_modify(|i| *i += 1).or_insert(1);
    });
    map
}

fn main() {
    let mut left = Vec::new();
    let mut right = Vec::new();

    include_str!("../input").lines().for_each(|line| {
        let splt = line.split("   ").collect::<Vec<&str>>();
        left.push(splt[0].parse::<i32>().unwrap());
        right.push(splt[1].parse::<i32>().unwrap());
    });

    left.sort();
    right.sort();

    let part_1 = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum::<i32>();

    println!("Part 1: {}", part_1);

    let freqmap = count_frequencies(&right);
    let part_2 = left
        .iter()
        .map(|item| item * freqmap.get(item).unwrap_or(&0))
        .sum::<i32>();

    println!("Part 2: {part_2}")
}
