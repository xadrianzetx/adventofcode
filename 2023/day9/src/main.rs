fn predict(report: &mut [i32]) -> (i32, i32) {
    let last = report.last().unwrap();
    let first = report.first().unwrap();

    let mut diffs = report
        .windows(2)
        .map(|window| window[1] - window[0])
        .collect::<Vec<i32>>();

    if diffs.iter().all(|n| n == &0) {
        return (*first, *last);
    }

    let (predicted_first, predicted_last) = predict(&mut diffs);
    (first - predicted_first, predicted_last + last)
}

fn main() {
    let ans = include_str!("../input")
        .lines()
        .map(|line| {
            line.split(' ')
                .filter_map(|num| num.parse::<i32>().ok())
                .collect::<Vec<i32>>()
        })
        .map(|mut report| predict(&mut report))
        .collect::<Vec<(i32, i32)>>()
        .into_iter()
        .reduce(|a, b| (a.0 + b.0, a.1 + b.1))
        .unwrap();

    println!("Part 1: {}", ans.0);
    println!("Part 2: {}", ans.1);
}
