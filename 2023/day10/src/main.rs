use std::collections::{HashMap, VecDeque, HashSet};

type Map = HashMap<(i32, i32), char>;

fn build_map(raw_map: &str) -> Map {
    let mut map = HashMap::new();
    for (row, line) in raw_map.lines().enumerate() {
        for (col, pipe) in line.chars().enumerate() {
            map.insert((row as i32, col as i32), pipe);
        }
    }
    map
}

fn find_start(map: &Map) -> (i32, i32) {
    for (k, v) in map {
        if v == &'S' {
            return *k;
        }
    }
    (0, 0)
}

fn steps_to_farthest(map: &mut Map, start: &(i32, i32)) -> (usize, HashSet<(i32, i32)>) {
    let mut visited = HashSet::new();
    visited.insert(*start);

    let mut steps = HashMap::new();
    steps.insert(*start, 0);
    
    let mut queue = VecDeque::from_iter(get_valid_start_directions(map, start));

    while let Some(pos) = queue.pop_front() {
        let pipe = map.get(&pos).unwrap();
        let neighbors = get_valid_neighbors(pipe, &pos);
        for neighbor_pos in neighbors {
            if visited.contains(&neighbor_pos) {
                let neighbor_steps = steps.get(&neighbor_pos).unwrap();
                steps.insert(pos, neighbor_steps + 1);
            } else {
                queue.push_back(neighbor_pos);
                visited.insert(pos);
            }
        }
    }
    (*steps.values().max().unwrap(), visited)
}

fn get_valid_start_directions(map: &Map, start: &(i32, i32)) -> Vec<(i32, i32)> {
    let mut directions =Vec::new();
    if let Some(east) = map.get(&(start.0, start.1 + 1)) {
        if vec!['J', '7', '-'].contains(&east) {
            directions.push((start.0, start.1 + 1));
        }
    }
    if let Some(west) = map.get(&(start.0, start.1 - 1)) {
        if vec!['L', 'F', '-'].contains(&west) {
            directions.push((start.0, start.1 - 1));
        }
    }
    if let Some(north) = map.get(&(start.0 - 1, start.1)) {
        if vec!['7', 'F', '|'].contains(&north) {
            directions.push((start.0 - 1, start.1));
        }
    }
    if let Some(south) = map.get(&(start.0 + 1, start.1)) {
        if vec!['J', 'L', '|'].contains(&south) {
            directions.push((start.0 + 1, start.1));
        }
    }

    assert!(directions.len() == 2);
    directions
}

fn get_valid_neighbors(pipe: &char, pos: &(i32, i32)) -> Vec<(i32, i32)> {
    match pipe {
        '|' => vec![(pos.0 + 1, pos.1), (pos.0 - 1, pos.1)],
        '-' => vec![(pos.0, pos.1 + 1), (pos.0, pos.1 - 1)],
        'L' => vec![(pos.0 - 1, pos.1), (pos.0, pos.1 + 1)],
        'J' => vec![(pos.0 - 1, pos.1), (pos.0, pos.1 - 1)],
        '7' => vec![(pos.0 + 1, pos.1), (pos.0, pos.1 - 1)],
        'F' => vec![(pos.0 + 1, pos.1), (pos.0, pos.1 + 1)],
        _ => unreachable!()
    }
}

fn remove_disconnected(map: &mut Map, loop_: &HashSet<(i32, i32)>) {
    for (pos, elem) in map {
        if !loop_.contains(pos) {
            *elem = '.';
        }
    }
}

fn main() {
    let raw_map = include_str!("../test-input3");
    let mut map = build_map(raw_map);
    let start = find_start(&map);
    let (part_1, loop_) = steps_to_farthest(&mut map, &start);
    println!("{part_1}");

    remove_disconnected(&mut map, &loop_);
    println!("{map:?}");
}
