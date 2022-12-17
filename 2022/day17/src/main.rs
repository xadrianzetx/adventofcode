use std::cmp::max;
use std::collections::HashSet;

enum Rocks {
    TypeA,
    TypeB,
    TypeC,
    TypeD,
    TypeE,
}

static ROCKS: &'static [Rocks] = &[
    Rocks::TypeA,
    Rocks::TypeB,
    Rocks::TypeC,
    Rocks::TypeD,
    Rocks::TypeE,
];

struct Rock {
    rock_type: Rocks,
    positions: Vec<(i32, i32)>,
}

impl Rock {
    fn from_height(typ_: &Rocks, height: i32) -> Self {
        match typ_ {
            // ####
            Rocks::TypeA => {
                let positions = vec![
                    (2, height + 3),
                    (3, height + 3),
                    (4, height + 3),
                    (5, height + 3),
                ];
                Rock {
                    rock_type: Rocks::TypeA,
                    positions,
                }
            }

            // .#.
            // ###
            // .#.
            Rocks::TypeB => {
                let positions = vec![
                    (2, height + 4),
                    (3, height + 4),
                    (4, height + 4),
                    (3, height + 3),
                    (3, height + 5),
                ];
                Rock {
                    rock_type: Rocks::TypeB,
                    positions,
                }
            }

            // ..#
            // ..#
            // ###
            Rocks::TypeC => {
                let positions = vec![
                    (2, height + 3),
                    (3, height + 3),
                    (4, height + 3),
                    (4, height + 4),
                    (4, height + 5),
                ];
                Rock {
                    rock_type: Rocks::TypeC,
                    positions,
                }
            }

            // #
            // #
            // #
            // #
            Rocks::TypeD => {
                let positions = vec![
                    (2, height + 3),
                    (2, height + 4),
                    (2, height + 5),
                    (2, height + 6),
                ];
                Rock {
                    rock_type: Rocks::TypeD,
                    positions,
                }
            }

            // ##
            // ##
            Rocks::TypeE => {
                let positions = vec![
                    (2, height + 3),
                    (2, height + 4),
                    (3, height + 3),
                    (3, height + 4),
                ];
                Rock {
                    rock_type: Rocks::TypeE,
                    positions,
                }
            }
        }
    }

    fn collides_with(&self, map: &HashSet<(i32, i32)>, move_: &char) -> bool {
        for pos in self.positions.iter() {
            let updated = match move_ {
                '>' => (pos.0 + 1, pos.1),
                '<' => (pos.0 - 1, pos.1),
                'v' => (pos.0, pos.1 - 1),
                _ => panic!(),
            };

            if updated.0 < 0 || updated.0 > 6 || updated.1 < 0 {
                return true;
            }

            if map.contains(&updated) {
                // println!("hit {:?}", updated);
                return true;
            }
        }
        false
    }

    fn update_position(&mut self, move_: &char) {
        for pos in self.positions.iter_mut() {
            match move_ {
                '>' => pos.0 += 1,
                '<' => pos.0 -= 1,
                'v' => pos.1 -= 1,
                _ => panic!(),
            }
        }
    }

    fn into_coordinates(self) -> Vec<(i32, i32)> {
        self.positions
    }
}

fn find_new_highest(map: &HashSet<(i32, i32)>) -> i32 {
    let mut top = 0;
    for elem in map.iter() {
        top = max(top, elem.1);
    }
    top + 1
}

fn main() {
    let dirs = include_str!("../input").chars().collect::<Vec<char>>();
    let mut moveptr = 0;
    let mut rockptr = 0;
    let mut top = 0;
    let mut settled = HashSet::new();

    for _ in 0..2022 {
        let mut rock = Rock::from_height(&ROCKS[rockptr], top);
        loop {
            // println!("iter top at {} rock at {:?}", top, rock.positions);
            if !rock.collides_with(&settled, &dirs[moveptr]) {
                // println!("moving {} {}", i, &dirs[moveptr]);
                rock.update_position(&dirs[moveptr]);
            }
            moveptr = (moveptr + 1) % dirs.len();

            if rock.collides_with(&settled, &'v') {
                // println!("{:?}", rock.positions);
                for pos in rock.into_coordinates() {
                    settled.insert(pos);
                }
                top = find_new_highest(&settled);
                break;
            }
            // println!("moving v");
            rock.update_position(&'v');
        }
        rockptr = (rockptr + 1) % ROCKS.len();
    }

    // println!("{:?}", settled);
    println!("{:?}", top);
}
