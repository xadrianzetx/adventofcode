use std::collections::{HashSet, VecDeque};

type BrickCoords = HashSet<((usize, usize, usize), (usize, usize, usize))>;

#[derive(Debug, Clone)]
struct Brick {
    c0: (usize, usize, usize),
    c1: (usize, usize, usize),
    supported_by: BrickCoords,
}

impl From<&str> for Brick {
    fn from(value: &str) -> Self {
        let mut coords = value.split('~');

        let c1 = coords
            .next()
            .unwrap()
            .split(',')
            .filter_map(|num| num.parse().ok())
            .collect::<Vec<usize>>();

        let c2 = coords
            .next()
            .unwrap()
            .split(',')
            .filter_map(|num| num.parse().ok())
            .collect::<Vec<usize>>();

        Self {
            c0: (c1[0], c1[1], c1[2]),
            c1: (c2[0], c2[1], c2[2]),
            supported_by: HashSet::new(),
        }
    }
}

impl Brick {
    fn collides_with(&self, other: &Brick) -> bool {
        let this_x: HashSet<usize> = HashSet::from_iter(self.c0.0..=self.c1.0);
        let this_y: HashSet<usize> = HashSet::from_iter(self.c0.1..=self.c1.1);
        let other_x: HashSet<usize> = HashSet::from_iter(other.c0.0..=other.c1.0);
        let other_y: HashSet<usize> = HashSet::from_iter(other.c0.1..=other.c1.1);
        this_x.intersection(&other_x).count() > 0 && this_y.intersection(&other_y).count() > 0
    }

    fn move_down(&mut self) {
        self.c0.2 -= 1;
        self.c1.2 -= 1;
    }

    fn on_the_floor(&self) -> bool {
        self.c0.2 == 1
    }

    fn register_support(&mut self, brick: &Brick) {
        self.supported_by.insert((brick.c0, brick.c1));
    }

    fn has_support(&self) -> bool {
        !self.supported_by.is_empty()
    }
}

fn settle(bricks: Vec<Brick>) -> Vec<Brick> {
    let mut settled: Vec<Brick> = Vec::new();
    let mut to_settle = VecDeque::from_iter(bricks.into_iter());
    while let Some(mut brick) = to_settle.pop_front() {
        while !brick.has_support() {
            if brick.on_the_floor() {
                break;
            }

            let below = settled
                .iter()
                .filter(|b| b.c1.2 == brick.c0.2 - 1)
                .collect::<Vec<&Brick>>();

            for brick_below in below {
                if brick.collides_with(brick_below) {
                    brick.register_support(brick_below);
                }
            }

            if !brick.has_support() {
                brick.move_down();
            }
        }
        settled.push(brick.clone());
    }
    settled
}

fn get_supporting(bricks: &[Brick]) -> BrickCoords {
    let mut required = HashSet::new();
    for brick in bricks.iter().rev() {
        if brick.supported_by.len() == 1 {
            for support in &brick.supported_by {
                required.insert(*support);
            }
        }
    }
    required
}

fn count_removable(bricks: &Vec<Brick>) -> usize {
    bricks.len() - get_supporting(bricks).len()
}


fn count_falling(bricks: &Vec<Brick>) -> usize {
    let mut all_falling = 0;
    for support in get_supporting(bricks) {
        let mut falling = HashSet::new();
        falling.insert(support);

        loop {
            let mut still_falling = false;
            for brick in bricks {
                if falling.contains(&(brick.c0, brick.c1)) {
                    continue;
                }

                if !brick.supported_by.is_empty() && brick.supported_by.is_subset(&falling) {
                    falling.insert((brick.c0, brick.c1));
                    still_falling = true;
                }
            }

            if !still_falling {
                break;
            }
        }

        all_falling += falling.len() - 1;
    }
    all_falling
}

fn main() {
    let mut bricks = include_str!("../input")
        .lines()
        .map(Brick::from)
        .collect::<Vec<Brick>>();

    bricks.sort_by_key(|b| b.c0.2);
    let settled = settle(bricks);

    println!("Part 1: {}", count_removable(&settled));
    println!("Part 2: {}", count_falling(&settled));
}
