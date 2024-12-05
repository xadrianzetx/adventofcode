use std::{
    collections::{BTreeSet, HashMap},
    usize,
};

fn parse_rules(data: &str) -> HashMap<usize, BTreeSet<usize>> {
    let mut rules = HashMap::new();
    data.lines().for_each(|line| {
        let numbers = line
            .split('|')
            .map(|elem| elem.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        rules
            .entry(numbers[0])
            .and_modify(|e: &mut BTreeSet<usize>| {
                e.insert(numbers[1]);
            })
            .or_insert(BTreeSet::from([numbers[1]]));
    });
    rules
}

fn is_valid_order(numbers: &[usize], rules: &HashMap<usize, BTreeSet<usize>>) -> bool {
    let mut seen = BTreeSet::new();
    for num in numbers {
        if !rules.contains_key(num) {
            seen.insert(*num);
            continue;
        }

        if !rules.get(num).unwrap().is_disjoint(&seen) {
            return false;
        }
        seen.insert(*num);
    }
    true
}

fn reorder(numbers: &[usize], rules: &HashMap<usize, BTreeSet<usize>>) -> Vec<usize> {
    let mut reordered = Vec::new();
    for num in numbers {
        if !rules.contains_key(num) {
            reordered.push(*num);
            continue;
        }

        let mut indexes = Vec::new();
        for rule in rules.get(num).unwrap() {
            if let Some(idx) = reordered.iter().position(|elem| elem == rule) {
                indexes.push(idx);
            }
        }

        if indexes.is_empty() {
            reordered.push(*num);
            continue;
        }

        let insert_idx = indexes.iter().min().unwrap();
        reordered.insert(*insert_idx, *num);
    }
    reordered
}

fn get_middle(numbers: Vec<usize>) -> usize {
    let idx = numbers.len() / 2;
    numbers[idx]
}

fn main() {
    let data = include_str!("../input")
        .split("\n\n")
        .collect::<Vec<&str>>();
    let rules = parse_rules(data[0]);

    let part_1 = data[1]
        .lines()
        .map(|line| {
            line.split(',')
                .map(|elem| elem.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .filter(|elem| is_valid_order(elem, &rules))
        .map(get_middle)
        .sum::<usize>();
    println!("{:?}", part_1);

    let part_2 = data[1]
        .lines()
        .map(|line| {
            line.split(',')
                .map(|elem| elem.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .filter(|elem| !is_valid_order(elem, &rules))
        .map(|elem| reorder(&elem, &rules))
        .map(get_middle)
        .sum::<usize>();
    println!("{:?}", part_2);
}
