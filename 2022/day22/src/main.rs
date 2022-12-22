use std::collections::HashMap;

#[derive(Clone, PartialEq)]
enum Element {
    Tile,
    Wall,
}

#[derive(Clone)]
struct Map((i32, i32), Element);
type Face = HashMap<(i32, i32), Map>;

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

struct Player {
    pos: (i32, i32),
    face: usize,
    facing: i32,
}

impl Player {
    fn new() -> Self {
        Player {
            pos: (0, 0),
            face: 0,
            facing: 0,
        }
    }

    fn take_step(&mut self, faces: &[Face]) -> Option<()> {
        let cand = match self.facing {
            0 => (self.pos.0, self.pos.1 + 1),
            1 => (self.pos.0 + 1, self.pos.1),
            2 => (self.pos.0, self.pos.1 - 1),
            3 => (self.pos.0 - 1, self.pos.1),
            _ => panic!(),
        };

        let (cand_pos, cand_face, cand_facing) = remap(faces, cand, self.face, self.facing);
        let cb = faces[cand_face].get(&cand_pos).unwrap();
        if cb.1 == Element::Wall {
            return None;
        }

        self.pos = cand_pos;
        self.face = cand_face;
        self.facing = cand_facing;
        Some(())
    }

    fn update_facing(&mut self, turn: char) {
        let update = match turn {
            'L' => -1,
            'R' => 1,
            _ => panic!(),
        };
        self.facing = ((self.facing + update % 4) + 4) % 4;
    }
}

fn remap(faces: &[Face], pos: (i32, i32), face: usize, facing: i32) -> ((i32, i32), usize, i32) {
    if faces[face].contains_key(&pos) {
        return (pos, face, facing);
    }

    // Wrapping rules LUT.
    match (face, facing) {
        (0, 3) => ((pos.1, 0), 5, 0),
        (0, 1) => ((0, pos.1), 2, 1),
        (0, 0) => ((pos.0, 0), 1, 0),
        (0, 2) => ((49 - pos.0, 0), 3, 0),
        (1, 3) => ((49, pos.1), 5, 3),
        (1, 1) => ((pos.1, 49), 2, 2),
        (1, 0) => ((49 - pos.0, 49), 4, 2),
        (1, 2) => ((pos.0, 49), 0, 2),
        (2, 3) => ((49, pos.1), 0, 3),
        (2, 1) => ((0, pos.1), 4, 1),
        (2, 0) => ((49, pos.0), 1, 3),
        (2, 2) => ((0, pos.0), 3, 1),
        (3, 3) => ((pos.1, 0), 2, 0),
        (3, 1) => ((0, pos.1), 5, 1),
        (3, 0) => ((pos.0, 0), 4, 0),
        (3, 2) => ((49 - pos.0, 0), 0, 0),
        (4, 3) => ((49, pos.1), 2, 3),
        (4, 1) => ((pos.1, 49), 5, 2),
        (4, 0) => ((49 - pos.0, 49), 1, 2),
        (4, 2) => ((pos.0, 49), 3, 2),
        (5, 3) => ((49, pos.1), 3, 3),
        (5, 1) => ((0, pos.1), 1, 1),
        (5, 0) => ((49, pos.0), 4, 3),
        (5, 2) => ((0, pos.0), 0, 1),
        _ => panic!(),
    }
}

fn main() {
    let mut row = 1;
    let mut faces: Vec<Face> = vec![HashMap::new(); 12];
    include_str!("../map").lines().for_each(|line| {
        for (col, chr) in line.chars().enumerate() {
            let face = (3 * ((row - 1) / 50)) + (col / 50);
            let face_coords = (((row - 1) % 50) as i32, (col % 50) as i32);
            let map_coords = (row as i32, (col + 1) as i32);
            match chr {
                '.' => faces[face].insert(face_coords, Map(map_coords, Element::Tile)),
                '#' => faces[face].insert(face_coords, Map(map_coords, Element::Wall)),
                ' ' => None,
                _ => panic!(),
            };
        }
        row += 1;
    });

    faces = faces
        .into_iter()
        .filter(|f| !f.is_empty())
        .collect::<Vec<Face>>();

    let mut path = PathDescription::from(include_str!("../input"));
    let mut player = Player::new();
    while let Some(p) = path.next() {
        match p {
            Path::Move(steps) => {
                for _ in 0..steps {
                    let taken = player.take_step(&faces);
                    if taken.is_none() {
                        break;
                    }
                }
            }
            Path::Turn(turn) => player.update_facing(turn),
        }
    }

    let map_pos = faces[player.face].get(&player.pos).unwrap().0;
    let ans = 1000 * map_pos.0 + 4 * map_pos.1 + player.facing;
    println!("Part2: {}", ans);
}
