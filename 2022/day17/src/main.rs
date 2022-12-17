use std::cmp::max;
use std::collections::HashSet;

#[derive(PartialEq, Debug, Clone, Copy)]
enum Rocks {
    TypeA,
    TypeB,
    TypeC,
    TypeD,
    TypeE,
}

static ROCKS: &[Rocks] = &[
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

#[derive(Default)]
struct Cycler {
    num_iter: i64,
    allows_register: bool,
    shadow_top: i64,
    residual_iterations: i64,
    // Next rock, next 10 moves as String, top value, iter
    states: Vec<(Rocks, String, i32, i32)>,
}

impl Cycler {
    fn with_num_iter(num_iter: i64) -> Self {
        Cycler {
            num_iter,
            allows_register: true,
            ..Default::default()
        }
    }

    fn register(&mut self, rock: Rocks, moves: String, top: i32, iter: i32) {
        if self.allows_register {
            self.states.push((rock, moves, top, iter));
        }
    }

    fn check_cycle(&mut self, rock: &Rocks, moves: &String, top: i32, iter: i32) {
        for state in self.states.iter() {
            if rock == &state.0 && moves == &state.1 && self.allows_register && iter > 2022 {
                let num_full_cycles_to_go = (self.num_iter - iter as i64) / (iter - state.3) as i64;
                let top_grows_during_cycle = top - state.2;

                self.shadow_top = num_full_cycles_to_go * top_grows_during_cycle as i64;
                self.residual_iterations =
                    ((self.num_iter - iter as i64) % (iter - state.3) as i64) + iter as i64;
                self.allows_register = false;
            }
        }
    }

    fn check_residual_iteration(&self, iter: i32, top: i32) {
        if iter as i64 == self.residual_iterations && !self.allows_register {
            println!("Part2: {}", self.shadow_top + top as i64);
        }
    }
}

fn encode_moves(moves: &[char], moveptr: &usize, n: usize) -> String {
    let mut next_moves = Vec::with_capacity(n);
    for i in 0..n {
        let next_ = (moveptr + i) % moves.len();
        next_moves.push(moves[next_]);
    }
    next_moves.iter().collect::<String>()
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
    let mut cycler = Cycler::with_num_iter(1000000000000);

    for i in 0..3000 {
        let mut rock = Rock::from_height(&ROCKS[rockptr], top);
        let moves = encode_moves(&dirs, &moveptr, 1000);
        cycler.check_cycle(&rock.rock_type, &moves, top, i);
        cycler.register(rock.rock_type, moves, top, i);
        cycler.check_residual_iteration(i, top);

        loop {
            if !rock.collides_with(&settled, &dirs[moveptr]) {
                rock.update_position(&dirs[moveptr]);
            }
            moveptr = (moveptr + 1) % dirs.len();

            if rock.collides_with(&settled, &'v') {
                for pos in rock.into_coordinates() {
                    settled.insert(pos);
                }
                top = find_new_highest(&settled);
                break;
            }
            rock.update_position(&'v');
        }
        rockptr = (rockptr + 1) % ROCKS.len();
        if i == 2022 - 1 {
            println!("Part1: {}", top);
        }
    }
}
