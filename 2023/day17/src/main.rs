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

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Heading {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Node {
    coordinates: (i32, i32),
    heading: Heading,
    steps_left: usize,
    steps_todo: usize,
    cost_so_far: usize,
}

impl Node {
    fn new(
        coordinates: (i32, i32),
        heading: Heading,
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

fn find_route(map: &mut Map, min_steps: usize, max_steps: usize) -> usize {
    let mut queue = BinaryHeap::new();
    let mut seen = HashSet::new();

    let nrows = map.iter().filter(|elem| elem.0 .1 == 0).count();
    let ncols = map.iter().filter(|elem| elem.0 .0 == 0).count();

    queue.push(Reverse(Node::new(
        (1, 0),
        Heading::Down,
        max_steps - 1,
        min_steps,
        0,
    )));
    queue.push(Reverse(Node::new(
        (0, 1),
        Heading::Right,
        max_steps - 1,
        min_steps,
        0,
    )));

    while let Some(Reverse(node)) = queue.pop() {
        if let Some(cost) = map.get(&node.coordinates) {
            if seen.contains(&(node.coordinates, node.heading, node.steps_left)) {
                continue;
            }
            seen.insert((node.coordinates, node.heading, node.steps_left));

            if node.coordinates == ((nrows - 1) as i32, (ncols - 1) as i32) {
                return node.cost_so_far + cost;
            }

            let row = node.coordinates.0;
            let col = node.coordinates.1;
            let mut new_steps_todo = 0;
            if node.steps_todo > 1 {
                new_steps_todo = node.steps_todo - 1;
            }

            match node.heading {
                Heading::Up => {
                    if node.steps_left > 0 {
                        queue.push(Reverse(Node::new(
                            (row - 1, col),
                            node.heading,
                            node.steps_left - 1,
                            new_steps_todo,
                            node.cost_so_far + cost,
                        )));
                    }
                    if new_steps_todo == 0 {
                        queue.push(Reverse(Node::new(
                            (row, col + 1),
                            Heading::Right,
                            max_steps - 1,
                            min_steps,
                            node.cost_so_far + cost,
                        )));
                        queue.push(Reverse(Node::new(
                            (row, col - 1),
                            Heading::Left,
                            max_steps - 1,
                            min_steps,
                            node.cost_so_far + cost,
                        )));
                    }
                }
                Heading::Down => {
                    if node.steps_left > 0 {
                        queue.push(Reverse(Node::new(
                            (row + 1, col),
                            node.heading,
                            node.steps_left - 1,
                            new_steps_todo,
                            node.cost_so_far + cost,
                        )));
                    }
                    if new_steps_todo == 0 {
                        queue.push(Reverse(Node::new(
                            (row, col + 1),
                            Heading::Right,
                            max_steps - 1,
                            min_steps,
                            node.cost_so_far + cost,
                        )));
                        queue.push(Reverse(Node::new(
                            (row, col - 1),
                            Heading::Left,
                            max_steps - 1,
                            min_steps,
                            node.cost_so_far + cost,
                        )));
                    }
                }
                Heading::Left => {
                    if node.steps_left > 0 {
                        queue.push(Reverse(Node::new(
                            (row, col - 1),
                            node.heading,
                            node.steps_left - 1,
                            new_steps_todo,
                            node.cost_so_far + cost,
                        )));
                    }
                    if new_steps_todo == 0 {
                        queue.push(Reverse(Node::new(
                            (row + 1, col),
                            Heading::Down,
                            max_steps - 1,
                            min_steps,
                            node.cost_so_far + cost,
                        )));
                        queue.push(Reverse(Node::new(
                            (row - 1, col),
                            Heading::Up,
                            max_steps - 1,
                            min_steps,
                            node.cost_so_far + cost,
                        )));
                    }
                }
                Heading::Right => {
                    if node.steps_left > 0 {
                        queue.push(Reverse(Node::new(
                            (row, col + 1),
                            node.heading,
                            node.steps_left - 1,
                            new_steps_todo,
                            node.cost_so_far + cost,
                        )));
                    }
                    if new_steps_todo == 0 {
                        queue.push(Reverse(Node::new(
                            (row + 1, col),
                            Heading::Down,
                            max_steps - 1,
                            min_steps,
                            node.cost_so_far + cost,
                        )));
                        queue.push(Reverse(Node::new(
                            (row - 1, col),
                            Heading::Up,
                            max_steps - 1,
                            min_steps,
                            node.cost_so_far + cost,
                        )));
                    }
                }
            }
        }
    }
    0
}

fn main() {
    let raw_map = include_str!("../input");
    let mut map = build_map(raw_map);

    println!("Part 1: {}", find_route(&mut map, 0, 3));
    println!("Part 2: {}", find_route(&mut map, 4, 10));
}
