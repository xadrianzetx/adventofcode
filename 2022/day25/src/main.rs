fn snafu_to_decimal(snafu: &str) -> i64 {
    snafu
        .chars()
        .rev()
        .enumerate()
        .map(|(place, digit)| {
            let decoded = match digit {
                '-' => -1,
                '=' => -2,
                _ => (digit.to_digit(10).unwrap()) as i64,
            };
            decoded * 5_i64.pow(place as u32)
        })
        .sum()
}

fn decimal_to_snafu(mut decimal: i64) -> String {
    // https://en.wikipedia.org/wiki/Balanced_ternary
    let mut ans = String::new();
    while decimal != 0 {
        let rem = (decimal % 5) as usize;
        ans.insert(0, ['0', '1', '2', '=', '-'][rem]);
        decimal -= [0, 1, 2, -2, -1][rem];
        decimal /= 5;
    }
    ans
}

fn main() {
    let ans: i64 = include_str!("../input").lines().map(snafu_to_decimal).sum();
    println!("Part1: {}", decimal_to_snafu(ans));
}
