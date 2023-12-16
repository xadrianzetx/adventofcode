use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Arc, Mutex};
use std::thread;

type Map = HashMap<(i32, i32), char>;

#[derive(Debug, Hash, PartialEq, Eq)]
enum Heading {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Beam {
    coordinates: (i32, i32),
    heading: Heading,
}

impl Beam {
    fn new(coordinates: (i32, i32), heading: Heading) -> Self {
        Self {
            coordinates,
            heading,
        }
    }

    fn get_new_heading(&self, map: &Map) -> Vec<Beam> {
        // let mut outgoing = Vec::new();
        let encountered = map.get(&self.coordinates).unwrap();
        match (encountered, &self.heading) {
            ('.', Heading::Up)
            | ('/', Heading::Right)
            | ('\\', Heading::Left)
            | ('|', Heading::Up) => vec![Self::new(
                (self.coordinates.0 - 1, self.coordinates.1),
                Heading::Up,
            )],
            ('.', Heading::Down)
            | ('/', Heading::Left)
            | ('\\', Heading::Right)
            | ('|', Heading::Down) => vec![Self::new(
                (self.coordinates.0 + 1, self.coordinates.1),
                Heading::Down,
            )],
            ('.', Heading::Left)
            | ('/', Heading::Down)
            | ('\\', Heading::Up)
            | ('-', Heading::Left) => vec![Self::new(
                (self.coordinates.0, self.coordinates.1 - 1),
                Heading::Left,
            )],
            ('.', Heading::Right)
            | ('/', Heading::Up)
            | ('\\', Heading::Down)
            | ('-', Heading::Right) => vec![Self::new(
                (self.coordinates.0, self.coordinates.1 + 1),
                Heading::Right,
            )],
            ('|', Heading::Left) | ('|', Heading::Right) => vec![
                Self::new((self.coordinates.0 - 1, self.coordinates.1), Heading::Up),
                Self::new((self.coordinates.0 + 1, self.coordinates.1), Heading::Down),
            ],
            ('-', Heading::Up) | ('-', Heading::Down) => vec![
                Self::new((self.coordinates.0, self.coordinates.1 - 1), Heading::Left),
                Self::new((self.coordinates.0, self.coordinates.1 + 1), Heading::Right),
            ],
            _ => unreachable!(),
        }
    }
}

fn build_map(raw_map: &str) -> Map {
    let mut map = HashMap::new();
    for (row, line) in raw_map.lines().enumerate() {
        for (col, char) in line.chars().enumerate() {
            map.insert((row as i32, col as i32), char);
        }
    }

    map
}

fn count_energized(map: &Map, entry: Beam) -> usize {
    let mut energized = HashSet::new();
    let mut visited = HashSet::new();

    let mut to_visit = VecDeque::new();
    to_visit.push_back(entry);

    while let Some(beam) = to_visit.pop_front() {
        if !map.contains_key(&beam.coordinates) {
            // Out of map.
            continue;
        }

        energized.insert(beam.coordinates);
        if !visited.contains(&beam) {
            to_visit.extend(beam.get_new_heading(map));
            visited.insert(beam);
        }
    }
    energized.len()
}

fn main() {
    let raw_map = include_str!("../input");
    let map = build_map(raw_map);
    println!(
        "Part 1: {}",
        count_energized(&map, Beam::new((0, 0), Heading::Right))
    );

    let nrows = map.iter().filter(|elem| elem.0 .1 == 0).count();
    let ncols = map.iter().filter(|elem| elem.0 .0 == 0).count();
    let all_energized = Arc::new(Mutex::new(Vec::new()));

    thread::scope(|s| {
        s.spawn(|| {
            let mut energized = Vec::new();
            for row in 0..nrows {
                let entry = Beam::new((row as i32, 0), Heading::Right);
                energized.push(count_energized(&map, entry));
            }
            all_energized.lock().unwrap().extend(energized);
        });

        s.spawn(|| {
            let mut energized = Vec::new();
            for row in 0..nrows {
                let entry = Beam::new((row as i32, ncols as i32), Heading::Left);
                energized.push(count_energized(&map, entry));
            }
            all_energized.lock().unwrap().extend(energized);
        });

        s.spawn(|| {
            let mut energized = Vec::new();
            for col in 0..ncols {
                let entry = Beam::new((0, col as i32), Heading::Down);
                energized.push(count_energized(&map, entry));
            }
            all_energized.lock().unwrap().extend(energized);
        });

        s.spawn(|| {
            let mut energized = Vec::new();
            for col in 0..ncols {
                let entry = Beam::new((nrows as i32, col as i32), Heading::Up);
                energized.push(count_energized(&map, entry));
            }
            all_energized.lock().unwrap().extend(energized);
        });
    });

    println!(
        "Part 2: {}",
        all_energized.lock().unwrap().iter().max().unwrap()
    );
}
