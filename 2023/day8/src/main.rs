use std::collections::HashMap;

type Network = HashMap<String, (String, String)>;

fn build_network(raw_nodes: &str) -> Network {
    let mut network = HashMap::new();
    for line in raw_nodes.lines() {
        let mut split = line.split(" = ");
        let current = split.next().unwrap();

        let mut choices = split.next().unwrap().split(',');
        let left = choices.next().unwrap().replace('(', "");
        let right = choices.next().unwrap().trim().replace(')', "");

        network.insert(current.to_string(), (left, right));
    }
    network
}

// I'm too lazy to write these, so credit where credit's due.
// https://github.com/TheAlgorithms/Rust/blob/7d2aa9e8be79cd23c36aa99cbfa66b520b132035/src/math/lcm_of_n_numbers.rs
pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd(a, b)
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn traverse(from: &str, to: &str, network: &Network, instructions: &[char]) -> usize {
    let mut steps = 0;
    let mut current = from.to_string();

    for instruction in instructions.iter().cycle() {
        let directions = network.get(&current).unwrap();
        match instruction {
            'L' => current = directions.0.clone(),
            'R' => current = directions.1.clone(),
            _ => unreachable!(),
        }

        steps += 1;
        if current.ends_with(to) {
            break;
        }
    }
    steps
}

fn main() {
    let input = include_str!("../input")
        .split("\n\n")
        .collect::<Vec<&str>>();

    let instructions = input[0].chars().collect::<Vec<char>>();
    let raw_nodes = input[1];
    let network = build_network(raw_nodes);

    let part1 = traverse("AAA", "ZZZ", &network, &instructions);
    println!("Part 1: {part1}");

    let mut start_nodes = Vec::new();
    network
        .keys()
        .filter(|k| k.ends_with('A'))
        .for_each(|k| start_nodes.push(k.clone()));

    let steps = start_nodes
        .iter()
        .map(|node| traverse(node, "Z", &network, &instructions))
        .collect::<Vec<usize>>();

    println!("Part 2: {}", lcm(&steps));
}
