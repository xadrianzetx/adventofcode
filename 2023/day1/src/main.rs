fn main() {
    let input = include_str!("../input");
    let part1 = input
        .lines()
        .map(|line| {
            let numbers = line
                .chars()
                .filter_map(|chr| chr.to_digit(10))
                .collect::<Vec<u32>>();
            numbers.first().unwrap() * 10 + numbers.last().unwrap()
        })
        .sum::<u32>();

    println!("Part1: {part1}");

    let part2 = input
        .lines()
        .map(|line| {
            let mut numbers = Vec::new();
            numbers.extend(line.match_indices(char::is_numeric));

            for pattern in &[
                "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
            ] {
                numbers.extend(line.match_indices(pattern));
            }

            numbers.sort_by_key(|key| key.0);
            numbers
                .iter()
                .filter_map(|(_, num)| match *num {
                    "one" => Some(1),
                    "two" => Some(2),
                    "three" => Some(3),
                    "four" => Some(4),
                    "five" => Some(5),
                    "six" => Some(6),
                    "seven" => Some(7),
                    "eight" => Some(8),
                    "nine" => Some(9),
                    _ => num.parse().ok(),
                })
                .collect::<Vec<u32>>()
        })
        .map(|numbers| numbers.first().unwrap() * 10 + numbers.last().unwrap())
        .sum::<u32>();

    println!("Part2: {part2}");
}
