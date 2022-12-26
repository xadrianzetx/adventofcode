use std::collections::VecDeque;

static OFFSETS: &[(i32, i32); 4] = &[(1, 0), (0, 1), (-1, 0), (0, -1)];

#[derive(PartialEq)]
enum Directions {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq)]
struct Blizzard {
    position: (i32, i32),
    direction: Directions,
}

impl PartialEq<(i32, i32)> for Blizzard {
    fn eq(&self, other: &(i32, i32)) -> bool {
        self.position.0 == other.0 && self.position.1 == other.1
    }
}

impl Blizzard {
    fn with_direction(position: (i32, i32), direction: &char) -> Self {
        match direction {
            '^' => Blizzard {
                position,
                direction: Directions::Up,
            },
            'v' => Blizzard {
                position,
                direction: Directions::Down,
            },
            '<' => Blizzard {
                position,
                direction: Directions::Left,
            },
            '>' => Blizzard {
                position,
                direction: Directions::Right,
            },
            _ => unreachable!(),
        }
    }

    fn update_position(&mut self, bounds: &(i32, i32)) {
        let maxrow = bounds.0 + 1;
        let maxcol = bounds.1 + 1;
        match self.direction {
            Directions::Up => {
                let newrow = ((self.position.0 - 1 % maxrow) + maxrow) % maxrow;
                self.position = (newrow, self.position.1);
            }
            Directions::Down => {
                let newrow = (self.position.0 + 1) % maxrow;
                self.position = (newrow, self.position.1);
            }
            Directions::Left => {
                let newcol = ((self.position.1 - 1 % maxcol) + maxcol) % maxcol;
                self.position = (self.position.0, newcol);
            }
            Directions::Right => {
                let newcol = (self.position.1 + 1) % maxcol;
                self.position = (self.position.0, newcol);
            }
        }
    }
}

fn traverse(blizzards: &mut [Blizzard], start: (i32, i32), exit: (i32, i32)) -> i32 {
    let mut minute = 0;
    let bounds = find_exit(blizzards);
    let mut to_visit = VecDeque::new();
    to_visit.push_back(start);

    loop {
        minute += 1;
        advance_blizzards(blizzards, &bounds);
        let mut next_to_visit = VecDeque::new();
        while let Some(curr_pos) = to_visit.pop_front() {
            for offset in OFFSETS.iter() {
                let cand = (curr_pos.0 + offset.0, curr_pos.1 + offset.1);
                if cand.0 < 0 || cand.1 < 0 || cand.1 > bounds.1 || cand.0 > bounds.0 {
                    continue;
                }

                if !blizzards.iter().any(|b| b == &cand) {
                    if cand.0 == exit.0 && cand.1 == exit.1 {
                        advance_blizzards(blizzards, &bounds);
                        return minute + 1;
                    }

                    if !next_to_visit.contains(&cand) {
                        next_to_visit.push_back(cand);
                    }
                }
            }

            if !blizzards.iter().any(|b| b == &curr_pos) {
                next_to_visit.push_back(curr_pos)
            }
        }
        assert!(!next_to_visit.is_empty());
        to_visit.append(&mut next_to_visit);
    }
}

fn advance_blizzards(blizzards: &mut [Blizzard], bounds: &(i32, i32)) {
    blizzards.iter_mut().for_each(|e| e.update_position(bounds));
}

fn find_exit(blizzards: &[Blizzard]) -> (i32, i32) {
    let maxrow = blizzards.iter().map(|b| b.position.0).max().unwrap();
    let maxcol = blizzards.iter().map(|b| b.position.1).max().unwrap();
    (maxrow, maxcol)
}

fn main() {
    let mut blizzards = Vec::new();
    include_str!("../input")
        .lines()
        .enumerate()
        .for_each(|(row, line)| {
            line.chars().enumerate().for_each(|(col, char)| {
                if ['^', 'v', '<', '>'].contains(&char) {
                    let pos = ((row - 1) as i32, (col - 1) as i32);
                    blizzards.push(Blizzard::with_direction(pos, &char));
                }
            });
        });

    // There are no optimizations applied to this BFS, so it's a bit slow.
    // All 3 trips take about 3 seconds total in release mode.
    let exit = find_exit(&blizzards);
    let trip1 = traverse(&mut blizzards, (-1, 0), exit);
    let trip2 = traverse(&mut blizzards, (exit.0 + 1, exit.1), (0, 0));
    let trip3 = traverse(&mut blizzards, (-1, 0), exit);
    println!("Part1: {}", trip1);
    println!("Part2: {}", trip1 + trip2 + trip3);
}
