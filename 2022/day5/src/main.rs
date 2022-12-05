use lazy_static::lazy_static;
use regex::Regex;

struct Moves {
    qty: usize,
    from: usize,
    to: usize,
}

impl From<&str> for Moves {
    fn from(line: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\d+").unwrap();
        }
        let moves = RE
            .find_iter(line)
            .filter_map(|digits| digits.as_str().parse().ok())
            .collect::<Vec<usize>>();

        Moves {
            qty: moves[0],
            from: moves[1] - 1,
            to: moves[2] - 1,
        }
    }
}

fn make_stacks() -> Vec<Vec<char>> {
    //     [V] [G]             [H]
    // [Z] [H] [Z]         [T] [S]
    // [P] [D] [F]         [B] [V] [Q]
    // [B] [M] [V] [N]     [F] [D] [N]
    // [Q] [Q] [D] [F]     [Z] [Z] [P] [M]
    // [M] [Z] [R] [D] [Q] [V] [T] [F] [R]
    // [D] [L] [H] [G] [F] [Q] [M] [G] [W]
    // [N] [C] [Q] [H] [N] [D] [Q] [M] [B]
    //  1   2   3   4   5   6   7   8   9
    vec![
        "NDMQBPZ".chars().collect(),
        "CLZQMDHV".chars().collect(),
        "QHRDVFZG".chars().collect(),
        "HGDFN".chars().collect(),
        "NFQ".chars().collect(),
        "DQVZFBT".chars().collect(),
        "QMTZDVSH".chars().collect(),
        "MGFPNQ".chars().collect(),
        "BWRM".chars().collect(),
    ]
}

fn move_crates(data: &str, part1: bool) -> String {
    let mut stacks = make_stacks();
    data.lines().for_each(|line| {
        let moves = Moves::from(line);
        let qty_moved = stacks[moves.from].len().saturating_sub(moves.qty);
        let mut crane = stacks[moves.from].split_off(qty_moved);

        if part1 {
            crane.reverse();
        }
        stacks[moves.to].extend_from_slice(&crane);
    });

    let mut buff = String::new();
    stacks.iter_mut().for_each(|s| buff.push(s.pop().unwrap()));
    buff
}

fn main() {
    let data = include_str!("../input");
    println!("Part1: {}", move_crates(data, true));
    println!("Part2: {}", move_crates(data, false));
}
