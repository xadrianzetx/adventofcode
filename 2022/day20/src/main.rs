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

fn mix(mut values: Vec<Value>, n_iter: usize) -> i64 {
    let file_len = (values.len() as i64) - 1;
    for _ in 0..n_iter {
        for idx in 0..=file_len {
            let curridx = values.iter().position(|v| v.index == idx as usize).unwrap();
            let val = values.remove(curridx);
            let newidx = ((curridx as i64 + val.value % file_len) + file_len) % file_len;
            values.insert(newidx as usize, val);
        }
    }

    let zeropos = values.iter().position(|v| v.value == 0).unwrap();
    [1000, 2000, 3000]
        .iter()
        .map(|v| values[(zeropos + v) % values.len()].clone())
        .map(|v| v.value)
        .sum()
}

fn main() {
    let data = include_str!("../input");
    println!("Part1: {}", mix(prepare_for_mixing(data, 1), 1));
    println!("Part1: {}", mix(prepare_for_mixing(data, 811589153), 10));
}
