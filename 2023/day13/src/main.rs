fn split_rows(pattern: &str) -> Vec<String> {
    pattern.lines().map(String::from).collect::<Vec<String>>()
}

fn split_cols(pattern: &str) -> Vec<String> {
    let mut buff = Vec::new();
    let width = pattern.lines().next().unwrap().len();
    for idx in 0..width {
        let chars = pattern
            .lines()
            .map(|line| line.chars().nth(idx).unwrap())
            .collect::<Vec<char>>();
        buff.push(String::from_iter(chars.iter()));
    }
    buff
}

struct Midpoint(usize, bool);

impl Midpoint {
    fn new(value: usize, had_smudge: bool) -> Self {
        Midpoint(value, had_smudge)
    }
}

fn find_split(pattern: &[String], part2: bool) -> Option<usize> {
    let mut midpoints = Vec::new();
    for cand in 1..pattern.len() {
        let upper = &pattern[..cand];
        let lower = &pattern[cand..];
        if let Some(midpoint) = find_midpoint(upper, lower) {
            midpoints.push(midpoint);
        }
    }

    midpoints
        .iter()
        .filter(|cand| cand.1 == part2)
        .map(|cand| cand.0)
        .next()
}

fn find_midpoint(upper: &[String], lower: &[String]) -> Option<Midpoint> {
    let mut had_smudge = false;
    for (idx, up) in upper.iter().rev().enumerate() {
        if let Some(down) = lower.get(idx) {
            if cleaned_smudge(up, down) {
                had_smudge = true;
                continue;
            }
            if up != down {
                return None;
            }
        }
    }
    Some(Midpoint::new(upper.len(), had_smudge))
}

fn cleaned_smudge(upper: &str, lower: &str) -> bool {
    let off_chars = upper
        .chars()
        .zip(lower.chars())
        .filter(|(u, d)| u != d)
        .count();

    off_chars == 1
}

fn find_reflection(pattern: &str, part2: bool) -> usize {
    if let Some(horizontal) = find_split(&split_rows(pattern), part2) {
        return 100 * horizontal;
    }
    find_split(&split_cols(pattern), part2).unwrap()
}

fn main() {
    let patterns = include_str!("../input")
        .split("\n\n")
        .collect::<Vec<&str>>();

    let part1 = patterns
        .iter()
        .map(|p| find_reflection(p, false))
        .sum::<usize>();
    println!("Part 1: {part1}");

    let part2 = patterns
        .iter()
        .map(|p| find_reflection(p, true))
        .sum::<usize>();
    println!("Part 2: {part2}");
}
