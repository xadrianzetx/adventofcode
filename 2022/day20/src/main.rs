#[derive(Clone)]
struct Value {
    index: usize,
    value: i64,
}

impl Value {
    fn with_original_index(index: usize, value: i64) -> Self {
        Value { index, value }
    }
}

fn prepare_for_mixing(data: &str, key: i64) -> Vec<Value> {
    data.lines()
        .enumerate()
        .map(|(index, line)| Value::with_original_index(index, line.parse::<i64>().unwrap() * key))
        .collect::<Vec<Value>>()
}

fn mix(values: Vec<Value>, n_iter: usize) -> i64 {
    let mut target = values.clone();
    for _ in 0..n_iter {
        for offset in values.iter() {
            let idx = target.iter().position(|v| v.index == offset.index).unwrap();
            let val = target.remove(idx);
            let tl = target.len() as i64;
            let newidx = ((idx as i64 + offset.value % tl) + tl) % tl;
            target.insert(newidx as usize, val);
        }
    }

    let zeropos = target.iter().position(|v| v.value == 0).unwrap();
    [1000, 2000, 3000]
        .iter()
        .map(|v| target[(zeropos + v) % target.len()].clone())
        .map(|v| v.value)
        .sum()
}

fn main() {
    let data = include_str!("../input");
    println!("Part1: {}", mix(prepare_for_mixing(data, 1), 1));
    println!("Part1: {}", mix(prepare_for_mixing(data, 811589153), 10));
}
