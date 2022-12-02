#[derive(PartialEq)]
enum Moves {
    Rock,
    Paper,
    Scissors,
}

impl From<&str> for Moves {
    fn from(played: &str) -> Self {
        match played {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => panic!(),
        }
    }
}

fn score_shape(mov: &Moves) -> i32 {
    match mov {
        Moves::Rock => 1,
        Moves::Paper => 2,
        Moves::Scissors => 3,
    }
}

fn check_win(left: &Moves, right: &Moves) -> i32 {
    if left == right {
        return 3;
    }
    if right == &Moves::Rock && left == &Moves::Scissors {
        return 6;
    }
    if right == &Moves::Paper && left == &Moves::Rock {
        return 6;
    }
    if right == &Moves::Scissors && left == &Moves::Paper {
        return 6;
    }
    0
}

fn main() {
    let mut total = 0;
    include_str!("../input").lines().for_each(|line| {
        let mut split = line.split(' ');
        let elf = Moves::from(split.next().unwrap());
        let player = Moves::from(split.next().unwrap());
        total += score_shape(&player) + check_win(&elf, &player);
    });
    println!("Part1: {}", total);
}
