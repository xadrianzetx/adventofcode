use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};

type Map = HashMap<(i32, i32), usize>;

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
    steps_left: usize,
    steps_todo: usize,
    cost_so_far: usize,
}

impl Node {
    fn new(
        coordinates: (i32, i32),
        heading: usize,
        steps_left: usize,
        steps_todo: usize,
        cost_so_far: usize,
    ) -> Self {
        Self {
            coordinates,
            heading,
            steps_left,
            steps_todo,
            cost_so_far,
        }
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost_so_far.cmp(&other.cost_so_far)
    }
}

fn find_route(map: &mut Map, min_steps: usize, max_steps: usize) -> Option<usize> {
    let mut queue = BinaryHeap::new();
    let mut seen = HashSet::new();

    let nrows = map.iter().filter(|elem| elem.0 .1 == 0).count();
    let ncols = map.iter().filter(|elem| elem.0 .0 == 0).count();

    // Up, right, down, left
    let headings = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];

    queue.push(Reverse(Node::new((1, 0), 2, max_steps - 1, min_steps, 0)));
    queue.push(Reverse(Node::new((0, 1), 1, max_steps - 1, min_steps, 0)));

    while let Some(Reverse(node)) = queue.pop() {
        if let Some(cost) = map.get(&node.coordinates) {
            if seen.contains(&(node.coordinates, node.heading, node.steps_left)) {
                continue;
            }
            seen.insert((node.coordinates, node.heading, node.steps_left));

            if node.coordinates == ((nrows - 1) as i32, (ncols - 1) as i32) {
                return Some(node.cost_so_far + cost);
            }

            let row = node.coordinates.0;
            let col = node.coordinates.1;
            let mut new_steps_todo = 0;
            if node.steps_todo > 1 {
                new_steps_todo = node.steps_todo - 1;
            }

            if node.steps_left > 0 {
                queue.push(Reverse(Node::new(
                    (
                        row + headings[node.heading].0,
                        col + headings[node.heading].1,
                    ),
                    node.heading,
                    node.steps_left - 1,
                    new_steps_todo,
                    node.cost_so_far + cost,
                )));
            }

            if new_steps_todo == 0 {
                let heading_a = (node.heading + 1) % headings.len();
                queue.push(Reverse(Node::new(
                    (row + headings[heading_a].0, col + headings[heading_a].1),
                    heading_a,
                    max_steps - 1,
                    min_steps,
                    node.cost_so_far + cost,
                )));

                let heading_b = (node.heading + 3) % headings.len();
                queue.push(Reverse(Node::new(
                    (row + headings[heading_b].0, col + headings[heading_b].1),
                    heading_b,
                    max_steps - 1,
                    min_steps,
                    node.cost_so_far + cost,
                )));
            }
        }
    }

    None
}

fn main() {
    let raw_map = include_str!("../input");
    let mut map = build_map(raw_map);

    println!("Part 1: {}", find_route(&mut map, 0, 3).unwrap());
    println!("Part 2: {}", find_route(&mut map, 4, 10).unwrap());
}
