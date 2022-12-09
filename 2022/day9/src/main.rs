use std::collections::HashSet;

type Rope = (i32, i32);

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum Directions {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
    Noop,
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

fn move_head(head: &mut Rope, direction: &Directions) {
    match direction {
        Directions::Up => head.0 += 1,
        Directions::Down => head.0 -= 1,
        Directions::Left => head.1 -= 1,
        Directions::Right => head.1 += 1,
        _ => panic!(),
    };
}

fn move_rope(rope: &mut Vec<Rope>, pos: usize, direction: &Directions) -> Directions {
    let head = rope[pos - 1];
    match direction {
        Directions::Noop => {
            return Directions::Noop;
        }

        Directions::Up => {
            if (head.0 - rope[pos].0).abs() > 1 {
                rope[pos].0 += 1;
                if head.1 > rope[pos].1 {
                    rope[pos].1 += 1;
                    return Directions::UpRight;
                } else if head.1 < rope[pos].1 {
                    rope[pos].1 -= 1;
                    return Directions::UpLeft;
                } else {
                    return Directions::Up;
                }
            }
        }
        Directions::Down => {
            if (head.0 - rope[pos].0).abs() > 1 {
                rope[pos].0 -= 1;
                if head.1 > rope[pos].1 {
                    rope[pos].1 += 1;
                    return Directions::DownRight;
                } else if head.1 < rope[pos].1 {
                    rope[pos].1 -= 1;
                    return Directions::DownLeft;
                } else {
                    return Directions::Down;
                }
            }
        }
        Directions::Left => {
            if (head.1 - rope[pos].1).abs() > 1 {
                rope[pos].1 -= 1;
                if head.0 > rope[pos].0 {
                    rope[pos].0 += 1;
                    return Directions::UpLeft;
                } else if head.0 < rope[pos].0 {
                    rope[pos].0 -= 1;
                    return Directions::DownLeft;
                } else {
                    return Directions::Left;
                }
            }
        }
        Directions::Right => {
            if (head.1 - rope[pos].1).abs() > 1 {
                rope[pos].1 += 1;
                if head.0 > rope[pos].0 {
                    rope[pos].0 += 1;
                    return Directions::UpRight;
                } else if head.0 < rope[pos].0 {
                    rope[pos].0 -= 1;
                    return Directions::DownRight;
                } else {
                    return Directions::Right;
                }
            }
        }
        Directions::UpLeft => {
            if (head.0 - rope[pos].0).abs() > 1 && (head.1 - rope[pos].1) == 0 {
                rope[pos].0 += 1;
                return Directions::Up;
            }
            if (head.1 - rope[pos].1).abs() > 1 && (head.0 - rope[pos].0) == 0 {
                rope[pos].1 -= 1;
                return Directions::Left;
            }
            if (head.1 - rope[pos].1).abs() > 1 || (head.0 - rope[pos].0).abs() > 1 {
                rope[pos].0 += 1;
                rope[pos].1 -= 1;
                return Directions::UpLeft;
            }
        }
        Directions::UpRight => {
            if (head.0 - rope[pos].0).abs() > 1 && (head.1 - rope[pos].1) == 0 {
                rope[pos].0 += 1;
                return Directions::Up;
            }
            if (head.1 - rope[pos].1).abs() > 1 && (head.0 - rope[pos].0) == 0 {
                rope[pos].1 += 1;
                return Directions::Right;
            }
            if (head.1 - rope[pos].1).abs() > 1 || (head.0 - rope[pos].0).abs() > 1 {
                rope[pos].0 += 1;
                rope[pos].1 += 1;
                return Directions::UpRight;
            }
        }
        Directions::DownLeft => {
            if (head.0 - rope[pos].0).abs() > 1 && (head.1 - rope[pos].1) == 0 {
                rope[pos].0 -= 1;
                return Directions::Down;
            }
            if (head.1 - rope[pos].1).abs() > 1 && (head.0 - rope[pos].0) == 0 {
                rope[pos].1 -= 1;
                return Directions::Left;
            }
            if (head.1 - rope[pos].1).abs() > 1 || (head.0 - rope[pos].0).abs() > 1 {
                rope[pos].0 -= 1;
                rope[pos].1 -= 1;
                return Directions::DownLeft;
            }
        }
        Directions::DownRight => {
            if (head.0 - rope[pos].0).abs() > 1 && (head.1 - rope[pos].1) == 0 {
                rope[pos].0 -= 1;
                return Directions::Down;
            }
            if (head.1 - rope[pos].1).abs() > 1 && (head.0 - rope[pos].0) == 0 {
                rope[pos].1 += 1;
                return Directions::Right;
            }
            if (head.1 - rope[pos].1).abs() > 1 || (head.0 - rope[pos].0).abs() > 1 {
                rope[pos].0 -= 1;
                rope[pos].1 += 1;
                return Directions::DownRight;
            }
        }
    }
    Directions::Noop
}

fn main() {
    let mut rope: Vec<Rope> = vec![(0, 0); 10];
    let mut visited = HashSet::new();
    include_str!("../input").lines().for_each(|line| {
        let mut d = line.split(' ');
        let direction = Directions::from(d.next().unwrap());
        let amt: i32 = d.next().unwrap().parse().unwrap();
        for _ in 0..amt {
            move_head(&mut rope[0], &direction);
            let mut last_dir = direction.clone();
            for i in 1..rope.len() {
                last_dir = move_rope(&mut rope, i, &last_dir);
            }
            visited.insert(rope.last().copied().unwrap());
        }
    });
    println!("{}", visited.len());
}
