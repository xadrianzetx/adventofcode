use std::collections::HashSet;

type Rope = (i32, i32);

enum Directions {
    Up,
    Down,
    Left,
    Right
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

fn move_rope(head: &mut Rope, tail: &mut Rope, direction: &Directions) {
    match direction {
        Directions::Up => {
            head.0 += 1;
            if (head.0 - tail.0).abs() > 1 {
                tail.0 += 1;
                if head.1 != tail.1 {
                    tail.1 = head.1;
                }
            }
        },
        Directions::Down => {
            head.0 -= 1;
            if (head.0 - tail.0).abs() > 1 {
                tail.0 -= 1;
                if head.1 != tail.1 {
                    tail.1 = head.1;
                }
            }
        },
        Directions::Left => {
            head.1 -= 1;
            if (head.1 - tail.1).abs() > 1 {
                tail.1 -= 1;
                if head.0 != tail.0 {
                    tail.0 = head.0;
                }
            }
        },
        Directions::Right => {
            head.1 += 1;
            if (head.1 - tail.1).abs() > 1 {
                tail.1 += 1;
                if head.0 != tail.0 {
                    tail.0 = head.0;
                }
            }
        },
    }
}

fn main() {
    let mut head: Rope = (0, 0);
    let mut tail: Rope = (0, 0);
    let mut visited = HashSet::new();
    include_str!("../input").lines().for_each(|line| {
        let mut d = line.split(' ');
        let direction = Directions::from(d.next().unwrap());
        let amt: i32 = d.next().unwrap().parse().unwrap();
        for _ in 0..amt {
            move_rope(&mut head, &mut tail, &direction);
            visited.insert(tail.clone());
        }
    });
    println!("{}", visited.len());
}
