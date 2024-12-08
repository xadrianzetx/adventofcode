use std::collections::{HashMap, HashSet};

type Map = HashMap<(i32, i32), char>;
type Antinodes = HashSet<(i32, i32)>;

fn build_map(data: &str) -> Map {
    let mut map = HashMap::new();
    for (row, line) in data.lines().enumerate() {
        for (col, antenna) in line.chars().enumerate() {
            if antenna != '.' {
                map.insert((row as i32, col as i32), antenna);
            }
        }
    }
    map
}

fn find_antinodes(map: &Map, min_bounce: i32, max_bounce: i32) -> Antinodes {
    let mut antinodes = HashSet::new();
    for (posa, nodea) in map {
        for (posb, nodeb) in map {
            if posa != posb && nodea == nodeb {
                // Slightly cheesing by bouncing the signal enough times to ensure we went out of bounds.
                for n in min_bounce..=max_bounce {
                    let x = posa.0 + ((posa.0 - posb.0) * n);
                    let y = posa.1 + ((posa.1 - posb.1) * n);
                    antinodes.insert((x, y));
                }
            }
        }
    }
    antinodes
}

fn count_valid_antinodes(data: &str, antinodes: &Antinodes) -> usize {
    let maxx = data.lines().next().unwrap().len() as i32 - 1;
    let maxy = data.lines().count() as i32 - 1;

    antinodes
        .iter()
        .filter(|(x, y)| (*x >= 0 && *x <= maxx) && (*y >= 0 && *y <= maxy))
        .count()
}

fn main() {
    let data = include_str!("../input");
    let map = build_map(data);
    let part_1 = count_valid_antinodes(data, &find_antinodes(&map, 1, 1));
    println!("Part 1: {part_1}");

    let part_2 = count_valid_antinodes(data, &find_antinodes(&map, 0, 100));
    println!("Part 2: {part_2}");
}
