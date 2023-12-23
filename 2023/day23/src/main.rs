use std::cmp::max;
use std::collections::HashMap;

type Map = HashMap<(i32, i32), char>;

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
        return longest;
    }
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

    let ans = walk(&map, entry, entry, exit, 0);
    println!("{ans}");
}
