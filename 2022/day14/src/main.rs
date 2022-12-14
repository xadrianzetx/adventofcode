use std::collections::HashMap;

enum Material {
    Rock,
    Sand,
}

type Cave = HashMap<(i32, i32), Material>;

#[inline]
fn sign(val: i32) -> i32 {
    match val {
        n if n > 0 => 1,
        0 => 0,
        _ => -1,
    }
}

fn parse_coordinates(coords: &str) -> (i32, i32) {
    let parsed = coords
        .split(',')
        .into_iter()
        .take(2)
        .map(|coord| coord.parse().unwrap())
        .collect::<Vec<i32>>();
    (parsed[0], parsed[1])
}

fn create_cave() -> Cave {
    let mut cave = HashMap::new();
    include_str!("../input").lines().for_each(|line| {
        line.split(" -> ")
            .collect::<Vec<&str>>()
            .windows(2)
            .for_each(|pair| {
                let (x1, y1) = parse_coordinates(pair[0]);
                let (x2, y2) = parse_coordinates(pair[1]);
                let dx = x1 - x2;
                let dy = y1 - y2;
                for x in 0..=dx.abs() {
                    for y in 0..=dy.abs() {
                        cave.insert((x2 + (x * sign(dx)), y2 + (y * sign(dy))), Material::Rock);
                    }
                }
            })
    });
    cave
}

fn find_ceil(cave: &Cave) -> i32 {
    cave.keys().map(|k| k.1).max().unwrap()
}

fn at_rest(cave: &Cave, sand: &mut (i32, i32)) -> bool {
    for offset in [(0, 1), (-1, 1), (1, 1)].iter() {
        if !cave.contains_key(&(sand.0 + offset.0, sand.1 + offset.1)) {
            sand.0 += offset.0;
            sand.1 += offset.1;
            return false;
        }
    }
    true
}

fn units_before_abyss() {
    let mut cave = create_cave();
    let rocks = cave.len();
    let ceil = find_ceil(&cave);
    let mut in_abyss = false;

    while !in_abyss {
        let mut sand = (500, 0);
        loop {
            if at_rest(&cave, &mut sand) {
                cave.insert(sand, Material::Sand);
                break;
            }

            if sand.1 > ceil {
                in_abyss = true;
                break;
            }
        }
    }
    println!("Part1: {}", cave.len() - rocks);
}

fn units_before_floor() {
    let mut cave = create_cave();
    cave.insert((500, 0), Material::Sand);
    let ceil = find_ceil(&cave);
    let rocks = cave.len();
    let mut sand = (499, 501);
    for row in 1..ceil + 2 {
        for col in sand.0..=sand.1 {
            if let Some(Material::Rock) = cave.get(&(col, row)) {
                continue;
            }
            for offset in -1..=1 {
                if let Some(Material::Sand) = cave.get(&(col + offset, row - 1)) {
                    cave.insert((col, row), Material::Sand);
                    break;
                }
            }
        }
        sand.0 -= 1;
        sand.1 += 1;
    }
    println!("Part2: {}", cave.len() - rocks + 1);
}

fn main() {
    units_before_abyss();
    units_before_floor()
}
