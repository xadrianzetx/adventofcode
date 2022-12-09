use std::cmp::{max, min};
use std::collections::HashSet;

enum Directions {
    Up,
    Down,
    Left,
    Right,
}

impl From<&str> for Directions {
    fn from(dir: &str) -> Self {
        match dir {
            "U" => Directions::Up,
            "D" => Directions::Down,
            "L" => Directions::Left,
            "R" => Directions::Right,
            _ => panic!(),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Position {
        Position { x, y }
    }
}

struct Knot {
    position: Position,
    visited: HashSet<Position>,
    next: Option<Box<Knot>>,
}

impl Knot {
    fn new(next: Option<Box<Knot>>) -> Knot {
        let position = Position::new(0, 0);
        let visited = HashSet::new();
        Knot {
            position,
            visited,
            next,
        }
    }

    fn move_in_direction(&mut self, direction: &Directions) {
        match direction {
            Directions::Left => self.position.x -= 1,
            Directions::Right => self.position.x += 1,
            Directions::Up => self.position.y += 1,
            Directions::Down => self.position.y -= 1,
        };

        self.visited.insert(self.position);
        if let Some(next) = &mut self.next {
            next.cascade_updates(&self.position);
        }
    }

    fn cascade_updates(&mut self, prev: &Position) {
        let dx = prev.x - self.position.x;
        let dy = prev.y - self.position.y;
        if (dx.abs() > 1 && dy != 0) || (dy.abs() > 1 && dx != 0) {
            self.position.x += min(max(dx, -1), 1);
            self.position.y += min(max(dy, -1), 1);
        } else {
            self.position.x += dx / 2;
            self.position.y += dy / 2;
        }

        self.visited.insert(self.position);
        if let Some(next) = &mut self.next {
            next.cascade_updates(&self.position);
        }
    }

    fn count_visited_by_tail(&self) -> usize {
        if let Some(next) = &self.next {
            return next.count_visited_by_tail();
        }
        self.visited.len()
    }

    fn count_visited_by(&self, n: usize) -> usize {
        if let Some(next) = &self.next {
            if n > 0 {
                return next.count_visited_by(n - 1);
            }
        }
        self.visited.len()
    }
}

struct Rope {
    head: Knot,
}

impl Rope {
    fn with_length(length: usize) -> Rope {
        let mut knot = None;
        for _ in 0..length {
            knot = Some(Box::new(Knot::new(knot)));
        }
        Rope {
            head: *knot.unwrap(),
        }
    }

    fn move_head(&mut self, direction: &Directions) {
        self.head.move_in_direction(direction);
    }

    fn count_visited_by_tail(&self) -> usize {
        self.head.count_visited_by_tail()
    }

    fn count_visited_by(&self, n: usize) -> usize {
        self.head.count_visited_by(n)
    }
}

fn main() {
    let mut rope = Rope::with_length(10);
    include_str!("../input").lines().for_each(|line| {
        let line = line.split_whitespace().collect::<Vec<&str>>().join("");
        let (direction, n_steps) = line.split_at(1);
        for _ in 0..n_steps.parse().unwrap() {
            rope.move_head(&Directions::from(direction));
        }
    });
    println!("Part1: {}", rope.count_visited_by(1));
    println!("Part2: {}", rope.count_visited_by_tail());
}
