fn parse(input: &str) -> i32 {
    match input {
        "A" | "X" => 0,
        "B" | "Y" => 1,
        "C" | "Z" => 2,
        _ => panic!(),
    }
}

fn main() {
    let mut total_known_move = 0;
    let mut total_known_outcome = 0;
    include_str!("../input").lines().for_each(|line| {
        let mut split = line.split(' ');
        let elf = parse(split.next().unwrap());
        let player = parse(split.next().unwrap());
        total_known_move += (player - elf + 1).rem_euclid(3) * 3 + player + 1;
        total_known_outcome += (elf + player - 1).rem_euclid(3) + 1 + player * 3;
    });
    println!("Part1: {}", total_known_move);
    println!("Part2: {}", total_known_outcome);
}
