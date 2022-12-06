use std::collections::HashSet;

fn find_marker(data: &str, seqlen: usize) -> Option<usize> {
    let chars = data.chars().into_iter().collect::<Vec<char>>();
    for (counter, window) in chars.windows(seqlen).enumerate() {
        let set: HashSet<&char> = HashSet::from_iter(window);
        if set.len() == seqlen {
            return Some(counter + seqlen);
        }
    }
    None
}

fn main() {
    let data = include_str!("../input");
    println!("Part1: {}", find_marker(data, 4).unwrap());
    println!("Part2: {}", find_marker(data, 14).unwrap());
}
