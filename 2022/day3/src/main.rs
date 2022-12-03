use std::collections::{HashMap, HashSet};

fn prepare_alphabet_lookup() -> HashMap<char, i32> {
    let mut map = HashMap::new();
    let alphabet = String::from_utf8((b'a'..=b'z').chain(b'A'..=b'Z').collect()).unwrap();
    for (num, letter) in alphabet.chars().enumerate() {
        map.insert(letter, num as i32);
    }
    map
}

fn main() {
    let mut total = 0;
    let alphabet = prepare_alphabet_lookup();
    include_str!("../test-input").lines().for_each(|line| {
        let mut hs = HashSet::new();
        let compartments = line.split_at(line.len() / 2);
        for letter in compartments.0.chars() {
            if compartments.1.contains(letter) {
                hs.insert(letter);
            }
        }
        for l in &hs {
            let cnt = alphabet.get(l).unwrap();
            total += cnt + 1;
        }
    });
    println!("{}", total);
}
