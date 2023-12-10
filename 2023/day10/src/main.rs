use std::collections::{HashMap, HashSet, VecDeque};

type Map = HashMap<(i32, i32), char>;
type Loop = HashSet<(i32, i32)>;

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

fn steps_to_farthest(map: &Map, start: &(i32, i32)) -> (usize, Loop) {
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
            }
            visited.insert(pos);
        }
    }
    (*steps.values().max().unwrap(), visited)
}

fn get_valid_start_directions(map: &Map, start: &(i32, i32)) -> Vec<(i32, i32)> {
    let mut directions = Vec::new();
    if let Some(east) = map.get(&(start.0, start.1 + 1)) {
        if vec!['J', '7', '-'].contains(east) {
            directions.push((start.0, start.1 + 1));
        }
    }
    if let Some(west) = map.get(&(start.0, start.1 - 1)) {
        if vec!['L', 'F', '-'].contains(west) {
            directions.push((start.0, start.1 - 1));
        }
    }
    if let Some(north) = map.get(&(start.0 - 1, start.1)) {
        if vec!['7', 'F', '|'].contains(north) {
            directions.push((start.0 - 1, start.1));
        }
    }
    if let Some(south) = map.get(&(start.0 + 1, start.1)) {
        if vec!['J', 'L', '|'].contains(south) {
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
        _ => unreachable!(),
    }
}

fn remove_disconnected(map: &mut Map, loop_: &Loop) {
    for (pos, elem) in map {
        if !loop_.contains(pos) {
            *elem = '.';
        }
    }
}

enum BsideDirection {
    Top,
    Bottom,
    Left,
    Right,
}

fn update_bside(curr_pipe: &char, bside: BsideDirection) -> BsideDirection {
    match curr_pipe {
        '|' => bside,
        '-' => bside,
        'L' | '7' => match bside {
            BsideDirection::Bottom => BsideDirection::Left,
            BsideDirection::Top => BsideDirection::Right,
            BsideDirection::Left => BsideDirection::Bottom,
            BsideDirection::Right => BsideDirection::Top,
        },
        'J' | 'F' => match bside {
            BsideDirection::Bottom => BsideDirection::Right,
            BsideDirection::Top => BsideDirection::Left,
            BsideDirection::Left => BsideDirection::Top,
            BsideDirection::Right => BsideDirection::Bottom,
        },
        _ => unreachable!(),
    }
}

fn mark_neighbors(
    map: &Map,
    marked: &mut Map,
    curr_pos: &(i32, i32),
    pipe: &char,
    bside: &BsideDirection,
) {
    // Ugh.
    match pipe {
        '|' => match bside {
            BsideDirection::Left => {
                maybe_mark(map, marked, &(curr_pos.0, curr_pos.1 - 1), 'B');
                maybe_mark(map, marked, &(curr_pos.0, curr_pos.1 + 1), 'A');
            }
            BsideDirection::Right => {
                maybe_mark(map, marked, &(curr_pos.0, curr_pos.1 - 1), 'A');
                maybe_mark(map, marked, &(curr_pos.0, curr_pos.1 + 1), 'B');
            }
            _ => unreachable!(),
        },
        '-' => match bside {
            BsideDirection::Top => {
                maybe_mark(map, marked, &(curr_pos.0 - 1, curr_pos.1), 'B');
                maybe_mark(map, marked, &(curr_pos.0 + 1, curr_pos.1), 'A');
            }
            BsideDirection::Bottom => {
                maybe_mark(map, marked, &(curr_pos.0 + 1, curr_pos.1), 'B');
                maybe_mark(map, marked, &(curr_pos.0 - 1, curr_pos.1), 'A');
            }
            _ => unreachable!(),
        },
        'L' => match bside {
            BsideDirection::Top | BsideDirection::Right => {
                maybe_mark(map, marked, &(curr_pos.0 + 1, curr_pos.1), 'A');
                maybe_mark(map, marked, &(curr_pos.0, curr_pos.1 - 1), 'A');
            }
            BsideDirection::Bottom | BsideDirection::Left => {
                maybe_mark(map, marked, &(curr_pos.0 + 1, curr_pos.1), 'B');
                maybe_mark(map, marked, &(curr_pos.0, curr_pos.1 - 1), 'B');
            }
        },
        'J' => match bside {
            BsideDirection::Bottom | BsideDirection::Right => {
                maybe_mark(map, marked, &(curr_pos.0 + 1, curr_pos.1), 'B');
                maybe_mark(map, marked, &(curr_pos.0, curr_pos.1 + 1), 'B');
            }
            BsideDirection::Top | BsideDirection::Left => {
                maybe_mark(map, marked, &(curr_pos.0 + 1, curr_pos.1), 'A');
                maybe_mark(map, marked, &(curr_pos.0, curr_pos.1 + 1), 'A');
            }
        },
        '7' => match bside {
            BsideDirection::Top | BsideDirection::Right => {
                maybe_mark(map, marked, &(curr_pos.0 - 1, curr_pos.1), 'B');
                maybe_mark(map, marked, &(curr_pos.0, curr_pos.1 + 1), 'B');
            }
            BsideDirection::Bottom | BsideDirection::Left => {
                maybe_mark(map, marked, &(curr_pos.0 - 1, curr_pos.1), 'A');
                maybe_mark(map, marked, &(curr_pos.0, curr_pos.1 + 1), 'A');
            }
        },
        'F' => match bside {
            BsideDirection::Top | BsideDirection::Left => {
                maybe_mark(map, marked, &(curr_pos.0 - 1, curr_pos.1), 'B');
                maybe_mark(map, marked, &(curr_pos.0, curr_pos.1 - 1), 'B');
            }
            BsideDirection::Bottom | BsideDirection::Right => {
                maybe_mark(map, marked, &(curr_pos.0 - 1, curr_pos.1), 'A');
                maybe_mark(map, marked, &(curr_pos.0, curr_pos.1 - 1), 'A');
            }
        },
        _ => unreachable!(),
    }
}

fn maybe_mark(map: &Map, marked: &mut Map, pos: &(i32, i32), value: char) {
    if let Some('.') = map.get(pos) {
        marked.insert(*pos, value);
    }
}

fn follow_the_wall(map: &mut Map, start: &(i32, i32)) -> Map {
    let mut visited = HashSet::new();
    visited.insert(*start);

    let mut marked = HashMap::new();
    let mut bside = BsideDirection::Left;

    let mut queue = VecDeque::from_iter(get_valid_start_directions(map, start));
    // Only need to go in one direction.
    queue.pop_front();

    while let Some(pos) = queue.pop_front() {
        let pipe = map.get(&pos).unwrap();
        mark_neighbors(map, &mut marked, &pos, pipe, &bside);
        bside = update_bside(pipe, bside);
        let neighbors = get_valid_neighbors(pipe, &pos);
        for neighbor_pos in neighbors {
            if !visited.contains(&neighbor_pos) {
                queue.push_back(neighbor_pos);
                visited.insert(pos);
            }
        }
    }

    marked
}

fn flood(map: &Map, marked: &Map, sign: char) -> Option<HashSet<(i32, i32)>> {
    let targets = marked
        .iter()
        .filter(|elem| elem.1 == &sign)
        .map(|elem| *elem.0)
        .collect::<Vec<(i32, i32)>>();
    let mut queue = VecDeque::from_iter(targets);
    let mut visited = HashSet::new();

    while let Some(pos) = queue.pop_front() {
        let val = map.get(&pos);
        val?;

        if let Some('.') = val {
            visited.insert(pos);
            for offset_row in -1..1 {
                for offset_col in -1..1 {
                    let neighbor = (pos.0 + offset_row, pos.1 + offset_col);
                    if !visited.contains(&neighbor) {
                        queue.push_back(neighbor);
                    }
                }
            }
        }
    }
    Some(visited)
}

fn main() {
    let raw_map = include_str!("../input");
    let mut map = build_map(raw_map);
    let start = find_start(&map);

    // Part 1 is straight forward. Just BFS in valid directions (connected pipes).
    let (part_1, loop_) = steps_to_farthest(&map, &start);
    println!("Part 1: {part_1}");

    // For part 2 the strat is to follow the wall of pipe loop, consistently marking the sides.
    // Starting from S and picking the left hand side to be represented as "A" in:
    // ..........
    // .S------7.
    // .|F----7|.
    // .||....||.
    // .||....||.
    // .|L-7F-J|.
    // .|..||..|.
    // .L--JL--J.
    // ..........
    //
    // Results in:
    // AAAAAAAAAA
    // AS------7A
    // A|F----7|A
    // A||BBBB||A
    // A||BBBB||A
    // A|L-7F-J|A
    // A|BB||BB|A
    // AL--JL--JA
    // AAAAAAAAAA
    //
    // For the real input, this will leave a bunch of spots that were not visited, so we can just flood fill
    // with both "A" and "B". One flood fill will go beyond map bounds and can be discarded, as it represents
    // regions outside of the loop. Total area of the remaining region is the answer to part 2. :^)
    remove_disconnected(&mut map, &loop_);
    let marked = follow_the_wall(&mut map, &start);
    let flooded = flood(&map, &marked, 'B');

    if let Some(filled) = flooded {
        println!("Part 2: {}", filled.len());
    }
}
