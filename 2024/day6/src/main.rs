use std::collections::{HashMap, HashSet};

type Map = HashMap<(i32, i32), char>;

fn build_map(data: &str) -> (Map, Option<(i32, i32)>) {
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

fn follow_guard(mut guard: (i32, i32), map: &mut Map, place_barriers: bool) -> bool {
    let headings = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let starting = guard.clone();
    let mut seen = HashSet::new();

    let mut headptr = 0;
    let mut heading = headings[headptr];

    while let Some(cand) = map.get(&(guard.0 + heading.0, guard.1 + heading.1)) {
        if seen.contains(&((guard.0 + heading.0, guard.1 + heading.1), heading)) {
            return true;
        }

        if cand == &'.' {
            guard.0 += heading.0;
            guard.1 += heading.1;
            seen.insert((guard, heading));
        } else {
            headptr = (headptr + 1).rem_euclid(4);
            heading = headings[headptr];
        }
    }

    if place_barriers {
        let visited = seen
            .iter()
            .map(|elem| elem.0)
            .collect::<HashSet<(i32, i32)>>();

        println!("Part 1: {}", visited.len());

        let mut looping = 0;
        for barrier in visited {
            map.insert(barrier, '#');
            if follow_guard(starting, map, false) {
                looping += 1
            }
            map.insert(barrier, '.');
        }
        println!("Part 2: {looping}");
    }
    false
}

fn main() {
    let data = include_str!("../input");
    let (mut map, guard) = build_map(data);
    follow_guard(guard.unwrap(), &mut map, true);
}
