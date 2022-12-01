fn main() {
    let mut count = 0;
    let mut max = 0;
    include_str!("../in").lines().for_each(|line| {
        if line == "" {
            count = 0;
        } else {
            let calories: i32 = line.parse().unwrap();
            count += calories;
            if count > max {
                max = count;
            }
        }
    });
    println!("{}", max);
}
