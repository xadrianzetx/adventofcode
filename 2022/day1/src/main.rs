fn main() {
    let mut count = 0;
    let mut calorie_vec = Vec::new();
    include_str!("../in").lines().for_each(|line| {
        if line.is_empty() {
            calorie_vec.push(count);
            count = 0;
        } else {
            let calories: i32 = line.parse().unwrap();
            count += calories;
        }
    });
    calorie_vec.sort_unstable();
    calorie_vec.reverse();
    println!("Part1: {}", calorie_vec[0]);
    println!("Part2: {}", &calorie_vec[0..3].iter().sum::<i32>());
}
