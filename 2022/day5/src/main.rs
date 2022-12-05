use regex::Regex;

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

fn move_crates(part: u32) {
    let mut stacks = make_stacks();
    let re = Regex::new(r"\d+").unwrap();
    include_str!("../input").lines().for_each(|line| {
        let moves = re
            .find_iter(line)
            .filter_map(|digits| digits.as_str().parse().ok())
            .collect::<Vec<usize>>();

        let qty_moved = stacks[moves[1] - 1].len().saturating_sub(moves[0]);
        let mut crane = stacks[moves[1] - 1].split_off(qty_moved);

        if part == 1 {
            crane.reverse();
        }
        stacks[moves[2] - 1].extend_from_slice(&crane);
    });

    print!("Part{}: ", part);
    stacks
        .iter_mut()
        .for_each(|s| print!("{}", s.pop().unwrap()));
    println!();
}

fn main() {
    move_crates(1);
    move_crates(2);
}
