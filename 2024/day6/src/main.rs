use std::collections::{HashMap, HashSet};

type Map = (HashMap<(i32, i32), char>, Option<(i32, i32)>);

fn build_map(data: &str) -> Map {
    let mut map = HashMap::new();
    let mut guard = None;
    for (row, line) in data.lines().enumerate() {
        for (col, chr) in line.chars().enumerate() {
            if chr == '^' {
                guard = Some((row as i32, col as i32));
                map.insert((row as i32, col as i32), '.');
                continue;
            }
            map.insert((row as i32, col as i32), chr);
        }
    }
    (map, guard)
}

fn follow_guard(
    mut guard: (i32, i32),
    map: &HashMap<(i32, i32), char>,
    place_bariers: bool,
) -> bool {
    let headings = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let starting = guard.clone();
    let mut seen = HashSet::new();
    let mut barriers = HashSet::new();

    let mut headptr = 0;
    let mut heading = headings[headptr];
    seen.insert((guard, heading));

    while let Some(cand) = map.get(&(guard.0 + heading.0, guard.1 + heading.1)) {
        if seen.contains(&((guard.0 + heading.0, guard.1 + heading.1), heading)) {
            return true;
        }

        if cand == &'.' {
            if place_bariers && (guard.0 + heading.0, guard.1 + heading.1) != starting {
                let start_heading = (headptr + 1).rem_euclid(4);
                // Barrier pos, starting pos, starting heading
                barriers.insert((
                    (guard.0 + heading.0, guard.1 + heading.1),
                    guard,
                    start_heading,
                ));
            }

            guard.0 += heading.0;
            guard.1 += heading.1;
            seen.insert((guard, heading));
        } else {
            headptr = (headptr + 1).rem_euclid(4);
            heading = headings[headptr];
        }
    }

    if place_bariers {
        let part_1 = seen
            .iter()
            .map(|elem| elem.0)
            .collect::<HashSet<(i32, i32)>>()
            .len();
        println!("Part 1: {part_1}");

        let mut looping = HashSet::new();
        for barrier in barriers {
            let mut alt_map = map.clone();
            alt_map.insert(barrier.0, '#');
            if follow_guard(starting, &alt_map, false) {
                looping.insert(barrier.0);
            }
        }
        println!("Part 2: {}", looping.len());
    }
    false
}

fn main() {
    let data = include_str!("../input");
    let (map, guard) = build_map(data);
    follow_guard(guard.unwrap(), &map, true);
}
