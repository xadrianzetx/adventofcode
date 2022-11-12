fn main() {
    let part1 = include_str!("../d1.txt")
        .lines()
        .map(|l| l.parse().unwrap())
        .collect::<Vec<i32>>()
        .windows(2)
        .into_iter()
        .map(|w| (w[1] > w[0]) as i32)
        .sum::<i32>();
    println!("{:?}", part1);

    let part2 = include_str!("../d1.txt")
        .lines()
        .map(|l| l.parse().unwrap())
        .collect::<Vec<i32>>()
        .windows(4)
        .into_iter()
        .map(|w| (w[3] > w[0]) as i32)
        .sum::<i32>();
    println!("{:?}", part2);
}
