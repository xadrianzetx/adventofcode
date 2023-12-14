use std::collections::HashMap;

type Map = HashMap<(i32, i32), char>;

enum Directions {
    North,
    South,
    East,
    West,
}

fn build_map(raw_map: &str, map: &mut Map) {
    for (row, line) in raw_map.lines().enumerate() {
        for (col, char) in line.chars().enumerate() {
            map.insert((row as i32, col as i32), char);
        }
    }
}

fn tilt(map: &mut Map, directrion: &Directions) {
    let width = map.keys().filter(|k| k.0 == 0).count();
    let height = map.keys().filter(|k| k.1 == 0).count();

    let (rowrng, colrng) = match directrion {
        Directions::North => (
            (0..height).collect::<Vec<usize>>(),
            (0..width).collect::<Vec<usize>>(),
        ),
        Directions::South => (
            (0..height).rev().collect::<Vec<usize>>(),
            (0..width).collect::<Vec<usize>>(),
        ),
        Directions::East => (
            (0..height).collect::<Vec<usize>>(),
            (0..width).rev().collect::<Vec<usize>>(),
        ),
        Directions::West => (
            (0..height).collect::<Vec<usize>>(),
            (0..width).collect::<Vec<usize>>(),
        ),
    };

    for row in rowrng {
        for col in colrng.clone() {
            if let Some('O') = map.get(&(row as i32, col as i32)) {
                let (newrow, newcol) = get_new_position(map, row as i32, col as i32, directrion);
                map.insert((row as i32, col as i32), '.');
                map.insert((newrow, newcol), 'O');
            }
        }
    }
}

fn get_new_position(map: &Map, row: i32, col: i32, directrion: &Directions) -> (i32, i32) {
    match directrion {
        Directions::North => {
            let mut pos = row - 1;
            while let Some(elem) = map.get(&(pos, col)) {
                if elem == &'.' {
                    pos -= 1;
                } else {
                    return (pos + 1, col);
                }
            }
            (pos + 1, col)
        }
        Directions::South => {
            let mut pos = row + 1;
            while let Some(elem) = map.get(&(pos, col)) {
                if elem == &'.' {
                    pos += 1;
                } else {
                    return (pos - 1, col);
                }
            }
            (pos - 1, col)
        }
        Directions::East => {
            let mut pos = col + 1;
            while let Some(elem) = map.get(&(row, pos)) {
                if elem == &'.' {
                    pos += 1;
                } else {
                    return (row, pos - 1);
                }
            }
            (row, pos - 1)
        }
        Directions::West => {
            let mut pos = col - 1;
            while let Some(elem) = map.get(&(row, pos)) {
                if elem == &'.' {
                    pos -= 1;
                } else {
                    return (row, pos + 1);
                }
            }
            (row, pos + 1)
        }
    }
}

fn tilt_all_directions(map: &mut Map) {
    tilt(map, &Directions::North);
    tilt(map, &Directions::West);
    tilt(map, &Directions::South);
    tilt(map, &Directions::East);
}

fn serialize_map(map: &Map) -> String {
    let mut buff = String::new();
    let width = map.keys().filter(|k| k.0 == 0).count();
    let height = map.keys().filter(|k| k.1 == 0).count();
    for row in 0..height {
        for col in 0..width {
            let elem = map.get(&(row as i32, col as i32)).unwrap();
            buff.push(*elem);
        }
        buff.push('\n');
    }
    buff
}

fn skip_to_cycle_end(
    memory: &HashMap<String, String>,
    cycle_start: String,
    cycle_after: i32,
) -> String {
    let mut next = cycle_start.clone();
    let mut cycle_length = 0;
    while let Some(value) = memory.get(&next) {
        next = value.to_string();
        cycle_length += 1;
    }

    let cycle_offset = (1000000000_i32 - cycle_after).rem_euclid(cycle_length + 1);
    let mut next = cycle_start;
    for _ in 0..cycle_offset {
        let val = memory.get(&next).unwrap();
        next = val.to_string();
    }
    next
}

fn summarize(map: &Map) -> usize {
    let height = map.keys().filter(|k| k.1 == 0).count();
    let mut total_load = 0;
    for (pos, rock) in map {
        if rock == &'O' {
            total_load += height as i32 - pos.0;
        }
    }
    total_load as usize
}

fn main() {
    let raw_map = include_str!("../input");
    let mut map = HashMap::new();
    build_map(raw_map, &mut map);
    tilt(&mut map, &Directions::North);
    println!("Part 1: {}", summarize(&map));

    let mut map = HashMap::new();
    build_map(raw_map, &mut map);
    let mut memory: HashMap<String, String> = HashMap::new();
    let mut cycle_after = 0;

    loop {
        cycle_after += 1;
        let before_tilt = serialize_map(&map);
        tilt_all_directions(&mut map);
        let after_tilt = serialize_map(&map);

        if memory.contains_key(&after_tilt) {
            let final_postition = skip_to_cycle_end(&memory, after_tilt, cycle_after);
            let mut final_map = HashMap::new();
            build_map(&final_postition, &mut final_map);
            println!("Part 2: {}", summarize(&final_map));
            break;
        }

        memory.insert(before_tilt, after_tilt);
    }
}
