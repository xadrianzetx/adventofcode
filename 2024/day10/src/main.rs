use std::collections::{HashMap, HashSet, VecDeque};

type Map = HashMap<(i32, i32), i32>;

const DIRECTIONS: &[(i32, i32); 4] = &[(0, 1), (1, 0), (0, -1), (-1, 0)];

fn build_map(data: &str) -> Map {
    let mut map = HashMap::new();
    for (row, line) in data.lines().enumerate() {
        for (col, height) in line.chars().enumerate() {
            map.insert(
                (row as i32, col as i32),
                height.to_digit(10).unwrap() as i32,
            );
        }
    }
    map
}

fn find_trailheads(map: &Map) -> Vec<(i32, i32)> {
    map.iter()
        .filter(|pos| pos.1 == &0)
        .map(|pos| *pos.0)
        .collect::<Vec<(i32, i32)>>()
}

fn score_trailhead(start: (i32, i32), map: &Map) -> (usize, usize) {
    let mut queue = VecDeque::new();
    queue.push_back(start);
    let mut seen_unique = HashSet::new();
    let mut seen_total = 0;

    while let Some(pos) = queue.pop_front() {
        let height = map.get(&pos).unwrap();
        for direction in DIRECTIONS {
            if let Some(cand) = map.get(&(pos.0 + direction.0, pos.1 + direction.1)) {
                if cand == &9 && cand - height == 1 {
                    seen_unique.insert((pos.0 + direction.0, pos.1 + direction.1));
                    seen_total += 1;
                    continue;
                }

                if cand - height == 1 {
                    queue.push_back((pos.0 + direction.0, pos.1 + direction.1));
                }
            }
        }
    }
    (seen_unique.len(), seen_total)
}

fn main() {
    let data = include_str!("../input");
    let map = build_map(data);
    let trailheads = find_trailheads(&map);

    let (part_1, part_2) = trailheads
        .iter()
        .map(|t| score_trailhead(*t, &map))
        .reduce(|a, b| (a.0 + b.0, a.1 + b.1))
        .unwrap();

    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");
}
