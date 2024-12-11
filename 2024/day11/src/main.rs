use std::collections::HashMap;

fn populate_stones(data: &str) -> HashMap<String, usize> {
    let mut stones = HashMap::new();
    for num in data.split_whitespace() {
        stones
            .entry(num.to_string())
            .and_modify(|v| *v += 1)
            .or_insert(1);
    }
    stones
}

fn blink(init_stones: &HashMap<String, usize>, times: usize) -> usize {
    let mut stones = init_stones.clone();

    for _ in 0..times {
        let mut new_stones: HashMap<String, usize> = HashMap::new();
        for (stone, cnt) in stones.iter() {
            if stone == "0" {
                new_stones
                    .entry("1".to_string())
                    .and_modify(|v| *v += cnt)
                    .or_insert(*cnt);
            } else if stone.len() % 2 == 0 {
                let (left, right) = stone.split_at(stone.len() / 2);
                new_stones
                    .entry(left.to_string())
                    .and_modify(|v| *v += cnt)
                    .or_insert(*cnt);

                let right_trimmed = right.trim_start_matches('0').to_string();
                if right_trimmed.is_empty() {
                    new_stones
                        .entry("0".to_string())
                        .and_modify(|v| *v += cnt)
                        .or_insert(*cnt);
                } else {
                    new_stones
                        .entry(right_trimmed)
                        .and_modify(|v| *v += cnt)
                        .or_insert(*cnt);
                }
            } else {
                let n = stone.parse::<isize>().unwrap() * 2024;
                new_stones
                    .entry(n.to_string())
                    .and_modify(|v| *v += cnt)
                    .or_insert(*cnt);
            }
        }

        stones = new_stones;
    }

    stones.values().sum()
}

fn main() {
    let data = include_str!("../input");

    let stones = populate_stones(data);

    let part_1 = blink(&stones, 25);
    println!("Part 1: {part_1}");

    let part_2 = blink(&stones, 75);
    println!("Part 2: {part_2}");
}
