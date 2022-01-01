fn simulate_lanternfish(init_state: &[usize], mut n_iter: usize) -> usize {
    let mut state = init_state.to_vec();
    while n_iter > 0 {
        state.rotate_left(1);
        state[6] += state[8];
        n_iter -= 1;
    }

    state.iter().sum()
}

fn main() {
    let mut state: Vec<usize> = vec![0; 9];
    include_str!("../d6.txt").trim().split(',').for_each(|e| {
        let idx: usize = e.parse().unwrap();
        state[idx] += 1;
    });

    let part1 = simulate_lanternfish(&state, 80);
    println!("{}", part1);

    let part2 = simulate_lanternfish(&state, 256);
    println!("{}", part2);
}
