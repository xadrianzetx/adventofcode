use std::collections::{HashMap, HashSet, VecDeque};

type Map = HashMap<(i32, i32), char>;

// Up, right, down, left
const DIRECTIONS: &[(i32, i32)] = &[(-1, 0), (0, 1), (1, 0), (0, -1)];

struct Node {
    pos: (i32, i32),
    cost: usize,
}

impl Node {
    fn with_pos_and_cost(pos: (i32, i32), cost: usize) -> Self {
        Self { pos, cost }
    }
}

fn build_map(coordinates: &[&str], width: i32, height: i32) -> Map {
    let mut map = HashMap::new();
    for coord in coordinates {
        let xy = coord
            .split(',')
            .map(|c| c.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        map.insert((xy[0], xy[1]), '#');
    }

    for row in 0..=width {
        for col in 0..=height {
            map.entry((row, col)).or_insert('.');
        }
    }
    map
}

fn add_coordinate(map: &mut Map, coordinate: &str) {
    let xy = coordinate
        .split(',')
        .map(|c| c.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    map.insert((xy[0], xy[1]), '#');
}

fn traverse(map: &Map, start: (i32, i32), end: (i32, i32)) -> Option<usize> {
    let mut queue = VecDeque::new();
    queue.push_back(Node::with_pos_and_cost(start, 0));

    let mut seen = HashSet::new();
    seen.insert(start);

    while let Some(elem) = queue.pop_front() {
        for direction in DIRECTIONS {
            let newpos = (elem.pos.0 + direction.0, elem.pos.1 + direction.1);
            if newpos == end {
                return Some(elem.cost + 1);
            }
            if let Some('.') = map.get(&newpos) {
                if !seen.contains(&newpos) {
                    queue.push_back(Node::with_pos_and_cost(newpos, elem.cost + 1));
                    seen.insert(newpos);
                }
            }
        }
    }
    None
}

fn main() {
    let coordinates = include_str!("../input").lines().collect::<Vec<&str>>();

    let mut map = build_map(&coordinates[..1024], 70, 70);
    let part_1 = traverse(&map, (0, 0), (70, 70)).unwrap();
    println!("Part 1: {part_1}");

    for coordinate in coordinates[1024..].iter() {
        add_coordinate(&mut map, coordinate);
        if traverse(&map, (0, 0), (70, 70)).is_none() {
            println!("Part 2: {coordinate}");
            break;
        }
    }
}
