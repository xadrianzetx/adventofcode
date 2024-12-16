use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

type Map = HashMap<(i32, i32), char>;

// Up, right, down, left
const HEADINGS: &[(i32, i32)] = &[(-1, 0), (0, 1), (1, 0), (0, -1)];

#[derive(Debug, PartialEq, Eq)]
struct Node {
    position: (i32, i32),
    heading: usize,
    total_cost: usize,
    visited: HashSet<(i32, i32)>,
}

impl Node {
    fn new(
        position: (i32, i32),
        heading: usize,
        total_cost: usize,
        mut visited: HashSet<(i32, i32)>,
    ) -> Self {
        visited.insert(position);
        Self {
            position,
            heading,
            total_cost,
            visited,
        }
    }

    fn next_move(&self, map: &Map) -> Vec<Self> {
        let mut next_moves = Vec::new();
        let direction = HEADINGS[self.heading];
        let neighbor = map
            .get(&(self.position.0 + direction.0, self.position.1 + direction.1))
            .unwrap();
        let turn_left = (self.heading + 3) % 4;
        let turn_right = (self.heading + 1) % 4;

        if neighbor == &'#' {
            next_moves.push(Self::new(
                self.position,
                turn_left,
                self.total_cost + 1000,
                self.visited.clone(),
            ));
            next_moves.push(Self::new(
                self.position,
                turn_right,
                self.total_cost + 1000,
                self.visited.clone(),
            ));
        } else {
            let newpos = (self.position.0 + direction.0, self.position.1 + direction.1);
            next_moves.push(Self::new(
                newpos,
                self.heading,
                self.total_cost + 1,
                self.visited.clone(),
            ));

            let maybe_left = HEADINGS[turn_left];
            if let Some('.') = map.get(&(
                self.position.0 + maybe_left.0,
                self.position.1 + maybe_left.1,
            )) {
                next_moves.push(Node::new(
                    self.position,
                    turn_left,
                    self.total_cost + 1000,
                    self.visited.clone(),
                ));
            }

            let maybe_right = HEADINGS[turn_right];
            if let Some('.') = map.get(&(
                self.position.0 + maybe_right.0,
                self.position.1 + maybe_right.1,
            )) {
                next_moves.push(Node::new(
                    self.position,
                    turn_right,
                    self.total_cost + 1000,
                    self.visited.clone(),
                ));
            }
        }
        next_moves
    }
}

fn build_maze(data: &str) -> (Map, (i32, i32), (i32, i32)) {
    let mut maze = HashMap::new();
    let mut start = (0, 0);
    let mut end = (0, 0);

    for (row, line) in data.lines().enumerate() {
        for (col, chr) in line.chars().enumerate() {
            if chr == 'S' {
                start = (row as i32, col as i32);
                maze.insert((row as i32, col as i32), '.');
            } else if chr == 'E' {
                end = (row as i32, col as i32);
                maze.insert((row as i32, col as i32), '.');
            } else {
                maze.insert((row as i32, col as i32), chr);
            }
        }
    }
    (maze, start, end)
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

fn traverse(maze: &Map, start: (i32, i32), end: (i32, i32)) -> Vec<Node> {
    let mut seen = HashMap::new();
    let mut queue = BinaryHeap::new();
    let mut end_nodes = Vec::new();
    queue.push(Reverse(Node::new(start, 1, 0, HashSet::new())));

    while let Some(Reverse(node)) = queue.pop() {
        if let Some(cost) = seen.get(&(node.position, node.heading)) {
            if cost < &node.total_cost {
                continue;
            }
        }
        seen.insert((node.position, node.heading), node.total_cost);

        if node.position == end {
            end_nodes.push(node);
            continue;
        }

        for neighbor in node.next_move(maze) {
            queue.push(Reverse(neighbor));
        }
    }
    end_nodes
}

fn main() {
    let data = include_str!("../input");

    let (maze, start, end) = build_maze(data);

    let paths = traverse(&maze, start, end);
    let part_1 = paths.iter().map(|p| p.total_cost).min().unwrap();
    println!("Part 1: {part_1}");

    let mut seats = HashSet::new();
    for p in paths.into_iter().filter(|p| p.total_cost == part_1) {
        for v in p.visited {
            seats.insert(v);
        }
    }
    println!("Part 2: {}", seats.len());
}
