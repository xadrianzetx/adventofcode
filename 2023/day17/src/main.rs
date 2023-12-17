use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};

type Map = HashMap<(i32, i32), usize>;

// Up, right, down, left
const HEADINGS: &[(i32, i32)] = &[(-1, 0), (0, 1), (1, 0), (0, -1)];

fn build_map(raw_map: &str) -> Map {
    let mut map = HashMap::new();
    for (row, line) in raw_map.lines().enumerate() {
        for (col, num) in line.chars().enumerate() {
            map.insert((row as i32, col as i32), num.to_digit(10).unwrap() as usize);
        }
    }
    map
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Node {
    coordinates: (i32, i32),
    heading: usize,
    // Max number of steps before we're required to take a turn.
    turn_after: usize,
    // Min number of steps before we can change heading.
    keep_for: usize,
    total_cost: usize,
}

impl Node {
    fn new(
        coordinates: (i32, i32),
        heading: usize,
        turn_after: usize,
        keep_for: usize,
        total_cost: usize,
    ) -> Self {
        Self {
            coordinates,
            heading,
            turn_after,
            keep_for,
            total_cost,
        }
    }

    fn get_valid_neighbors(&self, cost: usize, turn_after: usize, keep_for: usize) -> Vec<Self> {
        let mut neighbors = Vec::new();

        let row = self.coordinates.0;
        let col = self.coordinates.1;

        if self.turn_after > 0 {
            neighbors.push(Self::new(
                (
                    row + HEADINGS[self.heading].0,
                    col + HEADINGS[self.heading].1,
                ),
                self.heading,
                self.turn_after - 1,
                self.keep_for.saturating_sub(1),
                self.total_cost + cost,
            ));
        }

        if self.keep_for <= 1 {
            let heading_a = (self.heading + 1) % HEADINGS.len();
            neighbors.push(Self::new(
                (row + HEADINGS[heading_a].0, col + HEADINGS[heading_a].1),
                heading_a,
                turn_after,
                keep_for,
                self.total_cost + cost,
            ));

            let heading_b = (self.heading + 3) % HEADINGS.len();
            neighbors.push(Self::new(
                (row + HEADINGS[heading_b].0, col + HEADINGS[heading_b].1),
                heading_b,
                turn_after,
                keep_for,
                self.total_cost + cost,
            ));
        }
        neighbors
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.total_cost.cmp(&other.total_cost)
    }
}

fn find_route(map: &Map, min_steps: usize, max_steps: usize) -> Option<usize> {
    let nrows = map.iter().filter(|elem| elem.0 .1 == 0).count();
    let ncols = map.iter().filter(|elem| elem.0 .0 == 0).count();

    let mut seen = HashSet::new();
    let mut queue = BinaryHeap::new();
    queue.push(Reverse(Node::new((1, 0), 2, max_steps - 1, min_steps, 0)));
    queue.push(Reverse(Node::new((0, 1), 1, max_steps - 1, min_steps, 0)));

    while let Some(Reverse(node)) = queue.pop() {
        if seen.contains(&(node.coordinates, node.heading, node.turn_after)) {
            continue;
        }
        seen.insert((node.coordinates, node.heading, node.turn_after));

        if let Some(cost) = map.get(&node.coordinates) {
            if node.coordinates == ((nrows - 1) as i32, (ncols - 1) as i32) {
                return Some(node.total_cost + cost);
            }
            for neighnor in node.get_valid_neighbors(*cost, max_steps - 1, min_steps) {
                queue.push(Reverse(neighnor));
            }
        }
    }

    None
}

fn main() {
    let raw_map = include_str!("../input");
    let map = build_map(raw_map);

    println!("Part 1: {}", find_route(&map, 0, 3).unwrap());
    println!("Part 2: {}", find_route(&map, 4, 10).unwrap());
}
