use std::collections::{HashMap, HashSet};
use uuid::Uuid;

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

#[derive(Debug)]
struct Plant {
    plant_type: char,
    region_id: Option<Uuid>,
    perimeter: usize,
}

impl Plant {
    fn from_type(plant_type: char) -> Self {
        Self {
            plant_type,
            region_id: None,
            perimeter: 0,
        }
    }
}

fn build_garden(data: &str) -> HashMap<(i32, i32), Plant> {
    let mut garden = HashMap::new();
    for (row, line) in data.lines().enumerate() {
        for (col, plant) in line.chars().enumerate() {
            garden.insert((row as i32, col as i32), Plant::from_type(plant));
        }
    }

    garden
}

fn get_garden_bounds(garden: &HashMap<(i32, i32), Plant>) -> (i32, i32) {
    let maxrow = garden.keys().map(|k| k.0).max().unwrap();
    let maxcol = garden.keys().map(|k| k.1).max().unwrap();
    (maxrow, maxcol)
}

fn find_perimeters(garden: &mut HashMap<(i32, i32), Plant>) {
    let (maxrow, maxcol) = get_garden_bounds(garden);

    for row in 0..=maxrow {
        for col in 0..=maxcol {
            let plant = garden.get(&(row, col)).unwrap();
            let p = find_perimeter(plant, (row, col), garden);
            garden.entry((row, col)).and_modify(|e| e.perimeter = p);
        }
    }
}

fn find_perimeter(plant: &Plant, coords: (i32, i32), garden: &HashMap<(i32, i32), Plant>) -> usize {
    let mut total = 0;
    for direction in DIRECTIONS {
        if let Some(neighbnor) = garden.get(&(coords.0 + direction.0, coords.1 + direction.1)) {
            if neighbnor.plant_type == plant.plant_type {
                continue;
            }
        }
        total += 1;
    }
    total
}

fn group_regions(garden: &mut HashMap<(i32, i32), Plant>) {
    let (maxrow, maxcol) = get_garden_bounds(garden);

    for row in 0..=maxrow {
        for col in 0..=maxcol {
            let plant = garden.get(&(row, col)).unwrap();
            let up = garden.get(&(row - 1, col));
            let left = garden.get(&(row, col - 1));

            let no_up = up.is_none() || up.unwrap().plant_type != plant.plant_type;
            let no_left = left.is_none() || left.unwrap().plant_type != plant.plant_type;

            if no_up && no_left {
                garden
                    .entry((row, col))
                    .and_modify(|p| p.region_id = Some(Uuid::new_v4()));
            } else if !no_up && no_left {
                let id = up.unwrap().region_id;
                garden.entry((row, col)).and_modify(|p| p.region_id = id);
            } else if no_up && !no_left {
                let id = left.unwrap().region_id;
                garden.entry((row, col)).and_modify(|p| p.region_id = id);
            } else {
                let left_id = left.unwrap().region_id;
                let up_id = up.unwrap().region_id;

                if left_id == up_id {
                    garden
                        .entry((row, col))
                        .and_modify(|p| p.region_id = up_id);
                } else {
                    let new_id = Uuid::new_v4();
                    replace_ids(garden, left_id.unwrap(), new_id);
                    replace_ids(garden, up_id.unwrap(), new_id);
                    garden
                        .entry((row, col))
                        .and_modify(|p| p.region_id = Some(new_id));
                }
            }
        }
    }
}

fn replace_ids(garden: &mut HashMap<(i32, i32), Plant>, target: Uuid, replacement: Uuid) {
    for entry in garden.values_mut() {
        if entry.region_id == Some(target) {
            entry.region_id = Some(replacement);
        }
    }
}

fn get_price_perimeter(garden: &HashMap<(i32, i32), Plant>) -> usize {
    let mut regions: HashMap<Uuid, (usize, usize)> = HashMap::new();
    for plant in garden.values() {
        regions
            .entry(plant.region_id.unwrap())
            .and_modify(|ap| {
                ap.0 += 1;
                ap.1 += plant.perimeter;
            })
            .or_insert((1, plant.perimeter));
    }

    regions.values().map(|v| v.0 * v.1).sum::<usize>()
}

fn get_price_sides(garden: &HashMap<(i32, i32), Plant>) -> usize {
    let mut regions = HashSet::new();
    for plant in garden.values() {
        regions.insert(plant.region_id.unwrap());
    }

    let (maxrow, maxcol) = get_garden_bounds(garden);
    let mut regions_totals = HashMap::new();

    for region in regions {
        let mut region_sides = 0;
        let mut region_area = 0;
        for row in 0..=maxrow {
            for col in 0..=maxcol {
                let plant = garden.get(&(row, col)).unwrap();
                if plant.region_id != Some(region) {
                    continue;
                }

                region_area += 1;

                let ll = garden.get(&(row, col - 1));
                let lu = garden.get(&(row - 1, col - 1));
                let uu = garden.get(&(row - 1, col));
                let ru = garden.get(&(row - 1, col + 1));
                let rr = garden.get(&(row, col + 1));
                let dd = garden.get(&(row + 1, col));
                let ld = garden.get(&(row + 1, col - 1));
                let has_ll = ll.is_some() && ll.unwrap().region_id == Some(region);
                let has_lu = lu.is_some() && lu.unwrap().region_id == Some(region);
                let has_uu = uu.is_some() && uu.unwrap().region_id == Some(region);
                let has_dd = dd.is_some() && dd.unwrap().region_id == Some(region);
                let has_ld = ld.is_some() && ld.unwrap().region_id == Some(region);
                let has_rr = rr.is_some() && rr.unwrap().region_id == Some(region);
                let has_ru = ru.is_some() && ru.unwrap().region_id == Some(region);

                if (has_ll && !has_lu) || has_uu {} else {region_sides += 1;}
                if (has_ll && !has_ld) || has_dd {} else {region_sides += 1;}
                if (has_uu && !has_lu) || has_ll {} else {region_sides += 1;}
                if (has_uu && !has_ru) || has_rr {} else {region_sides += 1;}
            }
        }
        regions_totals.insert(region, (region_sides, region_area));
    }

    regions_totals.values().map(|r| r.0 * r.1).sum()
}

fn main() {
    let data = include_str!("../input");

    let mut garden = build_garden(data);
    find_perimeters(&mut garden);
    group_regions(&mut garden);

    let part_1 = get_price_perimeter(&garden);
    println!("Part 1: {part_1}");

    let part_2 = get_price_sides(&garden);
    println!("Part 2: {part_2}");
}
