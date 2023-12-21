use std::collections::{HashMap, HashSet};

const DIRECTIONS: &[(i32, i32)] = &[(0, -1), (0, 1), (-1, 0), (1, 0)];

type Map = HashMap<(i32, i32), char>;

fn build_map(raw_map: &str) -> (Map, (i32, i32)) {
    let mut start = (0, 0);
    let mut map = HashMap::new();
    for (row, line) in raw_map.lines().enumerate() {
        for (col, char) in line.chars().enumerate() {
            if char == 'S' {
                start = (row as i32, col as i32);
                map.insert((row as i32, col as i32), '.');
            } else {
                map.insert((row as i32, col as i32), char);
            }
        }
    }
    (map, start)
}

fn walk(map: &Map, start: &(i32, i32)) -> usize {
    let mut occupied: HashSet<(i32, i32)> = HashSet::new();
    occupied.insert(*start);

    for _ in 0..64 {
        let to_try = occupied.clone();
        occupied.drain();

        for pos in to_try {
            for direction in DIRECTIONS {
                let newpos = (pos.0 + direction.0, pos.1 + direction.1);
                if let Some('.') = map.get(&newpos) {
                    occupied.insert(newpos);
                }
            }
        }
    }
    occupied.len()
}

fn main() {
    let raw_map = include_str!("../input");
    let (map, start) = build_map(raw_map);

    println!("{}", walk(&map, &start));
}
