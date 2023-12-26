use std::cmp::max;
use std::collections::{HashMap, HashSet, VecDeque};

type Map = HashMap<(i32, i32), char>;
type CompressedMap = HashMap<(i32, i32), Vec<((i32, i32), i32)>>;

const DIRECTIONS: &[(i32, i32)] = &[(1, 0), (0, 1), (-1, 0), (0, -1)];

fn build_map(raw_map: &str) -> Map {
    let mut map = HashMap::new();
    for (row, line) in raw_map.lines().enumerate() {
        for (col, char) in line.chars().enumerate() {
            map.insert((row as i32, col as i32), char);
        }
    }
    map
}

fn walk(
    map: &Map,
    previous: &(i32, i32),
    current: &(i32, i32),
    finish: &(i32, i32),
    steps: usize,
) -> usize {
    if current == finish {
        return steps;
    }

    let mut longest = 0;
    let curr_tile = map.get(current).unwrap();
    if ['>', '<', '^', 'v'].contains(curr_tile) {
        let forced_new = match curr_tile {
            '>' => (current.0, current.1 + 1),
            '<' => (current.0, current.1 - 1),
            '^' => (current.0 - 1, current.1),
            'v' => (current.0 + 1, current.1),
            _ => unreachable!(),
        };

        if &forced_new != previous {
            return walk(map, current, &forced_new, finish, steps + 1);
        } else {
            return 0;
        }
    } else {
        for dir in DIRECTIONS {
            let newpos = (current.0 + dir.0, current.1 + dir.1);
            if &newpos == previous {
                continue;
            }

            if let Some(neighbor) = map.get(&newpos) {
                if ['>', '<', '^', 'v', '.'].contains(neighbor) {
                    let pth = walk(map, current, &newpos, finish, steps + 1);
                    longest = max(longest, pth);
                }
            }
        }

        longest
    }
}

fn find_junctions(map: &Map) -> Vec<(i32, i32)> {
    let mut junctions = Vec::new();
    map.iter()
        .filter(|entry| entry.1 == &'.')
        .for_each(|(pos, _)| {
            let mut neighbors = Vec::new();
            for offset in DIRECTIONS {
                let neighbor = (pos.0 + offset.0, pos.1 + offset.1);
                if let Some(char) = map.get(&neighbor) {
                    neighbors.push(char);
                }
            }
            if neighbors
                .iter()
                .all(|n| ['>', '<', '^', 'v', '#'].contains(n))
            {
                junctions.push(*pos);
            }
        });
    junctions
}

fn compress_map(
    map: &Map,
    junctions: &[(i32, i32)],
    start: &(i32, i32),
    finish: &(i32, i32),
) -> CompressedMap {
    let mut compressed_map = HashMap::new();
    compressed_map.insert(*start, distance_to_next_junctions(map, start));
    for j in junctions.iter().copied() {
        compressed_map.insert(j, distance_to_next_junctions(map, &j));
    }

    // DFS optimization - last junction has to lead to the finish, otherwise we'd violate
    // "never step onto the same tile twice" rule.
    let mut last_junction = None;
    for (j, neighbors) in &compressed_map {
        if neighbors
            .iter()
            .map(|n| n.0)
            .collect::<Vec<(i32, i32)>>()
            .contains(finish)
        {
            last_junction = Some(j);
        }
    }

    compressed_map
        .entry(*last_junction.unwrap())
        .and_modify(|e| {
            *e = e
                .iter()
                .cloned()
                .filter(|n| &n.0 == finish)
                .collect::<Vec<((i32, i32), i32)>>();
        });

    compressed_map
}

fn distance_to_next_junctions(map: &Map, start: &(i32, i32)) -> Vec<((i32, i32), i32)> {
    let mut queue = VecDeque::new();
    for offset in DIRECTIONS {
        let neighbor = (start.0 + offset.0, start.1 + offset.1);
        if let Some(n) = map.get(&neighbor) {
            if ['>', '<', '^', 'v', '.'].contains(n) {
                queue.push_back((neighbor, 1));
            }
        }
    }

    let mut seen = HashSet::new();
    seen.insert(*start);

    let mut found = Vec::new();

    while let Some((pos, dist)) = queue.pop_front() {
        for offset in DIRECTIONS {
            let neighbor = (pos.0 + offset.0, pos.1 + offset.1);
            if seen.contains(&neighbor) {
                continue;
            }

            if let Some(n) = map.get(&neighbor) {
                seen.insert(neighbor);
                if ['>', '<', '^', 'v', '.'].contains(n) {
                    if is_junction(map, &neighbor) {
                        found.push((neighbor, dist + 1));
                    } else {
                        queue.push_back((neighbor, dist + 1));
                    }
                }
            } else {
                if neighbor.0 > 0 {
                    found.push(((neighbor.0 - 1, neighbor.1), dist));
                }
            }
        }
    }

    found
}

fn is_junction(map: &Map, pos: &(i32, i32)) -> bool {
    for offset in DIRECTIONS {
        let neighbor = (pos.0 + offset.0, pos.1 + offset.1);
        if let Some(char) = map.get(&neighbor) {
            if char == &'.' {
                return false;
            }
        }
    }
    true
}

fn walk_compressed_map(
    map: &CompressedMap,
    current: &(i32, i32),
    previous: Vec<(i32, i32)>,
    finish: &(i32, i32),
    steps: usize,
) -> usize {
    if current == finish {
        return steps;
    }

    let mut longest = 0;
    for neighbor in map.get(current).unwrap() {
        if previous.contains(&neighbor.0) {
            continue;
        }

        let mut new_previous = previous.clone();
        new_previous.push(neighbor.0);
        let dst = walk_compressed_map(
            map,
            &neighbor.0,
            new_previous,
            finish,
            steps + neighbor.1 as usize,
        );
        longest = max(longest, dst);
    }
    longest
}

fn main() {
    let raw_map = include_str!("../input");

    let map = build_map(raw_map);
    let entry_vec = map
        .iter()
        .filter(|p| p.0 .0 == 0)
        .filter(|p| p.1 == &'.')
        .map(|p| p.0)
        .collect::<Vec<&(i32, i32)>>();
    let entry = entry_vec[0];

    let max_row = map
        .iter()
        .filter(|p| p.0 .1 == 0)
        .map(|p| p.0 .0)
        .max()
        .unwrap();
    let exit_vec = map
        .iter()
        .filter(|p| p.0 .0 == max_row)
        .filter(|p| p.1 == &'.')
        .map(|p| p.0)
        .collect::<Vec<&(i32, i32)>>();
    let exit = exit_vec[0];

    let part_1 = walk(&map, entry, entry, exit, 0);
    println!("Part 1: {part_1}");

    let junctions = find_junctions(&map);
    let compressed_map = compress_map(&map, &junctions, entry, exit);
    let part_2 = walk_compressed_map(&compressed_map, entry, Vec::new(), exit, 0);
    println!("Part 2: {part_2}");
}
