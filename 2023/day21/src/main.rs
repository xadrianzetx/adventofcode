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

fn walk(map: &Map, start: &(i32, i32), steps: usize) -> usize {
    let mut occupied: HashSet<(i32, i32)> = HashSet::new();
    occupied.insert(*start);

    for _ in 0..steps {
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
    let (map, (startr, startc)) = build_map(raw_map);

    println!("Part 1 {}", walk(&map, &(startr, startc), 64));

    // Nope. 
    // https://github.com/hyper-neutrino/advent-of-code/blob/b4795ca5b1b9d84aeeccc0cba8908fa92cba8796/2023/day21p2.py
    let size = map.keys().filter(|k| k.0 == 0).map(|k| k.1).count();
    let steps = 26501365;

    let grid_width = steps / size - 1;

    let odd = (grid_width / 2 * 2 + 1).pow(2);
    let even = ((grid_width + 1) / 2 * 2).pow(2);

    let odd_points = walk(&map, &(startr, startc), size * 2 + 1);
    let even_points = walk(&map, &(startr, startc), size * 2);

    let corner_t = walk(&map, &(size as i32 - 1, startc), size - 1);
    let corner_r = walk(&map, &(startr, 0), size - 1);
    let corner_b = walk(&map, &(0, startc), size - 1);
    let corner_l = walk(&map, &(startr, size as i32 - 1), size - 1);

    let small_tr = walk(&map, &(size as i32 - 1, 0), size / 2 - 1);
    let small_tl = walk(&map, &(size as i32 - 1, size as i32 - 1), size / 2 - 1);
    let small_br = walk(&map, &(0, 0), size / 2 - 1);
    let small_bl = walk(&map, &(0, size as i32 - 1), size / 2 - 1);

    let large_tr = walk(&map, &(size as i32 - 1, 0), size * 3 / 2 - 1);
    let large_tl = walk(&map, &(size as i32 - 1, size as i32 - 1), size * 3 / 2 - 1);
    let large_br = walk(&map, &(0, 0), size * 3 / 2 - 1);
    let large_bl = walk(&map, &(0, size as i32 - 1), size * 3 / 2 - 1);

    println!(
        "Part2: {}",
        odd * odd_points
            + even * even_points
            + corner_t
            + corner_r
            + corner_b
            + corner_l
            + (grid_width + 1) * (small_tr + small_tl + small_br + small_bl)
            + grid_width * (large_tr + large_tl + large_br + large_bl)
    )
}
