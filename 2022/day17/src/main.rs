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
    positions: Vec<(i64, i64)>,
}

impl Rock {
    fn from_height(typ_: &Rocks, height: i64) -> Self {
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

    fn collides_with(&self, settled: &HashSet<(i64, i64)>, move_: &char) -> bool {
        for pos in self.positions.iter() {
            let updated = match move_ {
                '>' => (pos.0 + 1, pos.1),
                '<' => (pos.0 - 1, pos.1),
                'v' => (pos.0, pos.1 - 1),
                _ => panic!(),
            };

            if hits_wall(&updated) || settled.contains(&updated) {
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

    fn into_coordinates(self) -> Vec<(i64, i64)> {
        self.positions
    }
}

#[derive(Default)]
struct Cycler {
    num_iter: i64,
    allows_register: bool,
    shadow_top: i64,
    residual_iterations: i64,
    // Next rock, next 10 moves as String, top value, iter.
    states: Vec<(Rocks, String, i64, i64)>,
}

impl Cycler {
    fn with_num_iter(num_iter: i64) -> Self {
        Cycler {
            num_iter,
            allows_register: true,
            ..Default::default()
        }
    }

    fn register(&mut self, rock: Rocks, moves: String, top: i64, iter: i64) {
        if self.allows_register {
            self.states.push((rock, moves, top, iter));
        }
    }

    fn check_cycle(&mut self, rock: &Rocks, moves: &String, top: i64, iter: i64) {
        for state in self.states.iter() {
            if rock == &state.0 && moves == &state.1 && self.allows_register && iter > 2022 {
                let num_full_cycles_to_go = (self.num_iter - iter) / (iter - state.3);
                let top_grows_during_cycle = top - state.2;

                self.shadow_top = num_full_cycles_to_go * top_grows_during_cycle;
                self.residual_iterations = ((self.num_iter - iter) % (iter - state.3)) + iter;
                self.allows_register = false;
            }
        }
    }

    fn check_residual_iteration(&self, iter: i64, top: i64) {
        if iter == self.residual_iterations && !self.allows_register {
            println!("Part2: {}", self.shadow_top + top);
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

fn find_new_highest(settled: &HashSet<(i64, i64)>) -> i64 {
    settled.iter().map(|e| e.1).max().unwrap() + 1
}

fn hits_wall(element: &(i64, i64)) -> bool {
    element.0 < 0 || element.0 > 6 || element.1 < 0
}

fn main() {
    let moves = include_str!("../input").chars().collect::<Vec<char>>();
    let mut moveptr = 0;
    let mut rockptr = 0;
    let mut top = 0;
    let mut settled = HashSet::new();
    let mut cycler = Cycler::with_num_iter(1000000000000);

    for i in 0..3000 {
        let mut rock = Rock::from_height(&ROCKS[rockptr], top);
        let moves_ahead = encode_moves(&moves, &moveptr, 1000);

        cycler.check_cycle(&rock.rock_type, &moves_ahead, top, i);
        cycler.register(rock.rock_type, moves_ahead, top, i);
        cycler.check_residual_iteration(i, top);

        loop {
            if !rock.collides_with(&settled, &moves[moveptr]) {
                rock.update_position(&moves[moveptr]);
            }
            moveptr = (moveptr + 1) % moves.len();

            if rock.collides_with(&settled, &'v') {
                settled.extend(rock.into_coordinates());
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
