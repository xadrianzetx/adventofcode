fn mix_and_prune(mut secret_number: usize) -> usize {
    secret_number = (secret_number * 64) ^ secret_number;
    secret_number %= 16777216;
    secret_number = (secret_number / 32) ^ secret_number;
    secret_number %= 16777216;
    secret_number = (secret_number * 2048) ^ secret_number;
    secret_number %= 16777216;
    secret_number
}

fn find_secret_number(initial_number: usize) -> usize {
    let mut secret_number = initial_number;
    for _ in 0..2000 {
        secret_number = mix_and_prune(secret_number);
    }
    secret_number
}

fn get_sequence(initial_number: usize) -> Vec<Vec<i32>> {
    let mut secret_number = initial_number;
    let mut seq = Vec::new();
    seq.push(initial_number % 10);

    for _ in 0..2000 {
        secret_number = mix_and_prune(secret_number);
        seq.push(secret_number % 10);
    }

    let diffs = seq
        .windows(2)
        .map(|elems| elems[1] as i32 - elems[0] as i32)
        .collect::<Vec<i32>>();
    diffs
        .windows(4)
        .map(|v| v.to_vec())
        .collect::<Vec<Vec<i32>>>()
}

fn try_sequence(initial_number: usize, sequence: &[i32]) -> usize {
    let mut secret_number = initial_number;
    let mut seq = Vec::new();
    seq.push(initial_number % 10);

    for _ in 0..2000 {
        secret_number = mix_and_prune(secret_number);
        seq.push(secret_number % 10);
    }

    let diffs = seq
        .windows(2)
        .map(|elems| elems[1] as i32 - elems[0] as i32)
        .collect::<Vec<i32>>();

    let mut idx = 4;
    for w in diffs.windows(4) {
        if w == sequence {
            return seq[idx];
        }
        idx += 1;
    }
    0
}

fn main() {
    use std::collections::HashSet;

    let data = include_str!("../input")
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let part_1 = data
        .iter()
        .map(|num| find_secret_number(*num))
        .sum::<usize>();
    println!("Part 1: {part_1}");

    let mut possible_sequences = HashSet::new();
    data.iter().for_each(|num| {
        let sequence = get_sequence(*num);
        for entry in sequence {
            possible_sequences.insert(entry);
        }
    });

    let mut best_bananas = 0;
    for sequence in possible_sequences {
        let bananas = data
            .iter()
            .map(|num| try_sequence(*num, &sequence))
            .sum::<usize>();
        if bananas > best_bananas {
            best_bananas = bananas;
        }
    }
    println!("Part 2: {best_bananas}");
}
