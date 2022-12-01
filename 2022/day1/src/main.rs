fn main() {
    let mut calories = Vec::new();
    include_str!("../input").split("\n\n").for_each(|group| {
        let calorie_count = group
            .lines()
            .map(|line| line.parse::<i32>().unwrap())
            .sum::<i32>();
        calories.push(calorie_count);
    });

    calories.sort_unstable();
    calories.reverse();
    println!("Part1: {}", calories[0]);
    println!("Part2: {}", &calories[0..3].iter().sum::<i32>());
}
