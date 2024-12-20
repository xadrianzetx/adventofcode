use std::collections::{HashMap, HashSet, VecDeque};

type Map = HashMap<(i32, i32), char>;

// Up, right, down, left
const DIRECTIONS: &[(i32, i32)] = &[(-1, 0), (0, 1), (1, 0), (0, -1)];

fn build_map(data: &str) -> (Map, (i32, i32), (i32, i32)) {
    let mut map = HashMap::new();
    let mut start = (0, 0);
    let mut end = (0, 0);

    for (row, line) in data.lines().enumerate() {
        for (col, chr) in line.chars().enumerate() {
            if chr == 'S' {
                start = (row as i32, col as i32);
                map.insert((row as i32, col as i32), '.');
            } else if chr == 'E' {
                end = (row as i32, col as i32);
                map.insert((row as i32, col as i32), '.');
            } else {
                map.insert((row as i32, col as i32), chr);
            }
        }
    }

    (map, start, end)
}

fn find_regular_path(
    map: &Map,
    start: &(i32, i32),
    end: &(i32, i32),
) -> Option<HashMap<(i32, i32), i32>> {
    let mut queue = VecDeque::new();
    queue.push_back((start.0, start.1, 0));

    let mut path = HashMap::new();
    path.insert(*start, 0);

    let mut seen = HashSet::new();
    seen.insert(*start);

    while let Some(elem) = queue.pop_front() {
        for direction in DIRECTIONS {
            let newpos = (elem.0 + direction.0, elem.1 + direction.1);
            if &newpos == end {
                path.insert(newpos, elem.2 + 1);
                return Some(path);
            }

            if let Some('.') = map.get(&newpos) {
                if !seen.contains(&newpos) {
                    queue.push_back((newpos.0, newpos.1, elem.2 + 1));
                    seen.insert(newpos);
                    path.insert(newpos, elem.2 + 1);
                }
            }
        }
    }
    None
}

fn find_shortcuts(path: &HashMap<(i32, i32), i32>, cheat_length: i32) -> HashMap<i32, i32> {
    let mut p = path.iter().collect::<Vec<(&(i32, i32), &i32)>>();
    p.sort_by_key(|elem| elem.1);
    let mut shortcuts = HashMap::new();

    for pos in p {
        let mut seen_ends = HashSet::new();
        for x_steps in -cheat_length..=cheat_length {
            for y_steps in -cheat_length..=cheat_length {
                let cheat_path_lenght = x_steps.abs() + y_steps.abs();
                if cheat_path_lenght <= cheat_length {
                    let newpos = (pos.0 .0 + x_steps, pos.0 .1 + y_steps);
                    if seen_ends.contains(&newpos) {
                        continue;
                    }
                    if let Some(maybe_skip) = path.get(&newpos) {
                        if maybe_skip - pos.1 > 0 {
                            shortcuts
                                .entry(maybe_skip - pos.1 - cheat_path_lenght)
                                .and_modify(|e| *e += 1)
                                .or_insert(1);
                            seen_ends.insert(newpos);
                        }
                    }
                }
            }
        }
    }
    shortcuts
}

fn main() {
    let data = include_str!("../input");
    let (map, start, end) = build_map(data);

    let path = find_regular_path(&map, &start, &end).unwrap();

    let skips_1 = find_shortcuts(&path, 2);
    let part_1 = skips_1
        .iter()
        .filter(|elem| elem.0 >= &100)
        .map(|elem| elem.1)
        .sum::<i32>();
    println!("{part_1}");

    let skips_2 = find_shortcuts(&path, 20);
    let part_2 = skips_2
        .iter()
        .filter(|elem| elem.0 >= &100)
        .map(|elem| elem.1)
        .sum::<i32>();
    println!("{part_2}");
}
