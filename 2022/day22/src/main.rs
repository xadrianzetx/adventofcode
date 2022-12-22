use std::collections::HashMap;

#[derive(Debug)]
enum Board {
    Tile,
    Wall,
}

#[derive(Debug)]
enum Path {
    Move(u32),
    Turn(char),
}

struct PathDescription {
    path: Vec<char>,
}

impl From<&str> for PathDescription {
    fn from(path: &str) -> Self {
        PathDescription {
            path: path.chars().collect::<Vec<char>>(),
        }
    }
}

impl PathDescription {
    fn next(&mut self) -> Option<Path> {
        if self.path.is_empty() {
            return None;
        }

        if self.path[0] == 'L' || self.path[0] == 'R' {
            return Some(Path::Turn(self.path.drain(..1).collect::<Vec<char>>()[0]));
        }

        let mut cnt = 0;
        while self.path[cnt].is_digit(10) {
            cnt += 1;
            if cnt == self.path.len() {
                break;
            }
        }

        let n = self.path.drain(..cnt).collect::<String>().parse().unwrap();
        Some(Path::Move(n))
    }
}

#[derive(Debug)]
enum Directions {
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
struct Player {
    pos: (i32, i32),
    facing: Directions,
}

impl Player {
    fn with_position(position: (i32, i32)) -> Self {
        Player {
            pos: position,
            facing: Directions::East,
        }
    }

    fn take_step(&mut self, map: &HashMap<(i32, i32), Board>) -> Option<()> {
        let mut cand = match self.facing {
            Directions::North => (self.pos.0 - 1, self.pos.1),
            Directions::South => (self.pos.0 + 1, self.pos.1),
            Directions::East => (self.pos.0, self.pos.1 + 1),
            Directions::West => (self.pos.0, self.pos.1 - 1),
        };

        cand = find_valid_position(map, cand, &self.facing);
        if let Some(Board::Wall) = map.get(&cand) {
            return None;
        }

        self.pos = cand;
        Some(())
    }

    fn update_facing(&mut self, turn: char) {
        match self.facing {
            Directions::North => {
                if turn == 'L' {
                    self.facing = Directions::West
                } else {
                    self.facing = Directions::East
                }
            }
            Directions::South => {
                if turn == 'L' {
                    self.facing = Directions::East
                } else {
                    self.facing = Directions::West
                }
            }
            Directions::East => {
                if turn == 'L' {
                    self.facing = Directions::North
                } else {
                    self.facing = Directions::South
                }
            }
            Directions::West => {
                if turn == 'L' {
                    self.facing = Directions::South
                } else {
                    self.facing = Directions::North
                }
            }
        }
    }

    fn score_facing(&self) -> i32 {
        // Facing is 0 for right (>), 1 for down (v), 2 for left (<), and 3 for up (^).
        match self.facing {
            Directions::East => 0,
            Directions::South => 1,
            Directions::West => 2,
            Directions::North => 3,
        }
    }
}

fn find_valid_position(
    map: &HashMap<(i32, i32), Board>,
    pos: (i32, i32),
    facing: &Directions,
) -> (i32, i32) {
    if map.contains_key(&pos) {
        return pos;
    }

    match facing {
        Directions::North => {
            let maxrow = map
                .keys()
                .filter_map(|k| if k.1 == pos.1 { Some(k.0) } else { None })
                .max()
                .unwrap();
            (maxrow, pos.1)
        }
        Directions::South => {
            let minrow = map
                .keys()
                .filter_map(|k| if k.1 == pos.1 { Some(k.0) } else { None })
                .min()
                .unwrap();
            (minrow, pos.1)
        }
        Directions::East => {
            let mincol = map
                .keys()
                .filter_map(|k| if k.0 == pos.0 { Some(k.1) } else { None })
                .min()
                .unwrap();
            (pos.0, mincol)
        }
        Directions::West => {
            let maxcol = map
                .keys()
                .filter_map(|k| if k.0 == pos.0 { Some(k.1) } else { None })
                .max()
                .unwrap();
            (pos.0, maxcol)
        }
    }
}

fn main() {
    let mut map = HashMap::new();
    let mut row = 1;
    let mut leftmost = 0;
    include_str!("../map").lines().for_each(|line| {
        for (col, chr) in line.chars().enumerate() {
            match chr {
                '.' => {
                    if row == 1 && leftmost == 0 {
                        leftmost = col + 1;
                    }
                    map.insert((row, (col + 1) as i32), Board::Tile)
                }
                '#' => map.insert((row, (col + 1) as i32), Board::Wall),
                ' ' => None,
                _ => panic!(),
            };
        }
        row += 1;
    });

    let mut directions = PathDescription::from(include_str!("../input"));
    let mut player = Player::with_position((1, leftmost as i32));
    while let Some(p) = directions.next() {
        match p {
            Path::Move(steps) => {
                for _ in 0..steps {
                    let taken = player.take_step(&map);
                    if taken.is_none() {
                        break;
                    }
                }
            }
            Path::Turn(turn) => player.update_facing(turn),
        }
    }
    println!(
        "Part1: {}",
        (1000 * player.pos.0) + (4 * player.pos.1) + player.score_facing()
    );
}
