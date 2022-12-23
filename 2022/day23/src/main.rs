use std::collections::{HashMap, HashSet};

fn spread_out(elfs: &mut HashSet<(i32, i32)>, directions: &[(i32, i32)]) -> bool {
    let mut proposed = HashMap::new();
    let mut prefs = HashMap::new();
    for elf in elfs.iter() {
        let mut elf_cand = None;
        let mut tot_neighbors = 0;
        for direction in directions.chunks(3) {
            let neighbors = direction
                .iter()
                .filter_map(|d| {
                    let neighbor = (elf.0 + d.0, elf.1 + d.1);
                    elfs.get(&neighbor)
                })
                .count();
            tot_neighbors += neighbors;
            if neighbors == 0 && elf_cand.is_none() {
                elf_cand = Some((elf.0 + direction[0].0, elf.1 + direction[0].1));
            }
        }

        if tot_neighbors > 0 && elf_cand.is_some() {
            proposed
                .entry(elf_cand.unwrap())
                .and_modify(|e| *e += 1)
                .or_insert(1);
            prefs.insert(*elf, elf_cand.unwrap());
        }
    }

    let mut new_elfs = HashSet::new();
    let mut moved = false;
    for elf in elfs.iter() {
        match prefs.get(elf) {
            Some(pref) => {
                if proposed.get(pref).unwrap() == &1 {
                    new_elfs.insert(*pref);
                    moved = true;
                } else {
                    new_elfs.insert(*elf);
                }
            }
            None => {
                new_elfs.insert(*elf);
            }
        }
    }

    *elfs = new_elfs;
    moved
}

fn count_empty_ground(elfs: &HashSet<(i32, i32)>) -> i32 {
    let rows = elfs.iter().map(|e| e.0).collect::<Vec<i32>>();
    let cols = elfs.iter().map(|e| e.1).collect::<Vec<i32>>();
    let height = rows.iter().max().unwrap() - rows.iter().min().unwrap();
    let width = cols.iter().max().unwrap() - cols.iter().min().unwrap();
    let area = (height + 1) * (width + 1);
    area - elfs.len() as i32
}

fn main() {
    let mut row = 0;
    let mut elfs = HashSet::new();
    include_str!("../input").lines().for_each(|line| {
        for (col, chr) in line.chars().enumerate() {
            if chr == '#' {
                elfs.insert((row, col as i32));
            }
        }
        row += 1;
    });

    let mut directions = vec![
        (-1, 0),  // N
        (-1, 1),  // NE
        (-1, -1), // NW
        (1, 0),   // S
        (1, 1),   // SE
        (1, -1),  //SW
        (0, -1),  // W
        (-1, -1), // NW
        (1, -1),  // SW
        (0, 1),   // E
        (-1, 1),  // NE
        (1, 1),   // SE
    ];

    for round in 0..10000 {
        let moved = spread_out(&mut elfs, &directions);
        if round == 10 - 1 {
            println!("Part1: {}", count_empty_ground(&elfs));
        }

        if !moved {
            println!("Part2: {}", round + 1);
            break;
        }

        directions.rotate_left(3);
    }
}
