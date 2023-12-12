use std::collections::VecDeque;

fn check_valid(config: &str, groups: &[usize], mut fill_queue: VecDeque<char>) -> usize {
    let mut buff = String::new();
    for char in config.chars() {
        if char == '?' {
            buff.push(fill_queue.pop_front().unwrap());
        } else {
            buff.push(char);
        }
    }

    let got = buff
        .replace('.', " ")
        .split_ascii_whitespace()
        .into_iter()
        .map(|grp| grp.len())
        .collect::<Vec<usize>>();
    if got == groups {
        return 1;
    } else {
        return 0;
    }
}

fn fill_voids(config: &str, groups: &[usize], free: usize, fill_queue: VecDeque<char>) -> usize {
    if free == 0 {
        return check_valid(config, groups, fill_queue);
    }

    let mut left_queue = fill_queue.clone();
    left_queue.push_back('.');
    let left = fill_voids(config, groups, free - 1, left_queue);

    let mut right_queue = fill_queue.clone();
    right_queue.push_back('#');
    let right = fill_voids(config, groups, free - 1, right_queue);

    left + right
}

fn main() {
    let mut cnt = 0;
    include_str!("../debug-input").lines().for_each(|line| {
        let mut records = line.split(' ');
        let config = records.next().unwrap();
        let groups = records
            .next()
            .unwrap()
            .split(',')
            .filter_map(|num| num.parse().ok())
            .collect::<Vec<usize>>();
        // println!("{}, {:?}", config, groups);
        let free = config.chars().filter(|char| char == &'?').count();
        let ret = fill_voids(config, &groups, free, VecDeque::new());
        cnt += ret;
    });
    println!("{cnt}");
}
