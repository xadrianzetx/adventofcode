use std::collections::HashSet;
use std::cmp::{min, max};

type Rope = (i32, i32);

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
            _ => panic!()
        }
    }
}

fn move_head(head: &mut Rope, direction: &Directions) {
    match direction {
        Directions::Up => head.0 += 1,
        Directions::Down => head.0 -= 1,
        Directions::Left => head.1 -= 1,
        Directions::Right => head.1 += 1,
    };
}

fn move_rope(rope: &mut Vec<Rope>, pos: usize) {
    let head = rope[pos - 1];
    let dx = head.0 - rope[pos].0;
    let dy = head.1 - rope[pos].1;
    if (dx.abs() > 1 && dy != 0) || (dy.abs() > 1 && dx != 0) {
        rope[pos].0 += min(max(dx, -1), 1);
        rope[pos].1 += min(max(dy, -1), 1);
    } else {
        rope[pos].0 += dx / 2;
        rope[pos].1 += dy / 2;
    }
}

fn main() {
    let mut rope: Vec<Rope> = vec![(0, 0); 10];
    let mut visited = HashSet::new();
    let mut visited2 = HashSet::new();
    include_str!("../input").lines().for_each(|line| {
        let mut d = line.split(' ');
        let direction = Directions::from(d.next().unwrap());
        let amt: i32 = d.next().unwrap().parse().unwrap();
        for _ in 0..amt {
            move_head(&mut rope[0], &direction);
            for i in 1..rope.len() {
                move_rope(&mut rope, i);
            }
            visited.insert(rope[1].clone());
            visited2.insert(rope.last().copied().unwrap());
        }
    });
    println!("Part1: {}", visited.len());
    println!("Part2: {}", visited2.len());
}
