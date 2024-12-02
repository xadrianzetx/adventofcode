use std::collections::HashSet;

fn parse_report(report: &str) -> Vec<i32> {
    report
        .split_whitespace()
        .map(|elem| elem.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

fn mark_safe(report: Vec<i32>, allow_bad_level: bool) -> u32 {
    let diffs = report.windows(2).map(|items| items[1] - items[0]);
    let signs = diffs.clone().map(|d| d.signum()).collect::<HashSet<i32>>();
    let magnitudes = diffs
        .filter(|d| d.abs() < 1 || d.abs() > 3)
        .collect::<Vec<i32>>();

    if signs.len() > 1 || !magnitudes.is_empty() {
        if allow_bad_level {
            for i in 0..report.len() {
                let mut report_candidate = report.clone();
                report_candidate.remove(i);
                if mark_safe(report_candidate, false) != 0 {
                    return 1;
                }
            }
        }
        return 0;
    }
    1
}

fn main() {
    let reports = include_str!("../input")
        .lines()
        .map(parse_report)
        .collect::<Vec<Vec<i32>>>();

    let part_1 = reports
        .clone()
        .into_iter()
        .map(|report| mark_safe(report, false))
        .sum::<u32>();
    println!("Part 1: {part_1}");

    let part_1 = reports
        .clone()
        .into_iter()
        .map(|report| mark_safe(report, true))
        .sum::<u32>();
    println!("Part 2: {part_1}");
}
