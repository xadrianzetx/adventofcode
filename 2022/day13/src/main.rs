use itertools::EitherOrBoth::{Both, Left, Right};
use itertools::Itertools;
use serde_json::Value;
use std::cmp::Ordering;

#[derive(PartialEq, Eq, Debug)]
enum PacketOrdering {
    Ok,
    Wrong,
    Undefined,
}

fn compare(left: &Value, right: &Value) -> PacketOrdering {
    match (left, right) {
        (Value::Array(l), Value::Array(r)) => {
            for lr in l.iter().zip_longest(r.iter()) {
                let ord = match lr {
                    Left(_) => PacketOrdering::Wrong,
                    Right(_) => PacketOrdering::Ok,
                    Both(a, b) => compare(a, b),
                };
                if ord != PacketOrdering::Undefined {
                    return ord;
                }
            }
            PacketOrdering::Undefined
        }
        (Value::Number(l), Value::Number(r)) => match l.as_i64().cmp(&r.as_i64()) {
            Ordering::Less => PacketOrdering::Ok,
            Ordering::Greater => PacketOrdering::Wrong,
            Ordering::Equal => PacketOrdering::Undefined,
        },
        (Value::Number(l), Value::Array(_)) => {
            let x = serde_json::json!([l]);
            compare(&x, right)
        }
        (Value::Array(_), Value::Number(r)) => {
            let x = serde_json::json!([r]);
            compare(left, &x)
        }
        _ => panic!(),
    }
}

fn is_divider_packet(packets: &Value) -> bool {
    match packets {
        Value::Number(n) => {
            if let Some(a) = n.as_i64() {
                return [2, 6].contains(&a);
            }
            false
        }
        Value::Array(a) => {
            if a.is_empty() || a.len() > 1 {
                return false;
            }
            is_divider_packet(&a[0])
        }
        _ => panic!(),
    }
}

fn part1(data: &str) {
    let ans = data
        .split("\n\n")
        .enumerate()
        .filter_map(|(index, pair)| {
            let lr = pair
                .split('\n')
                .map(|row| serde_json::from_str(row).unwrap())
                .collect::<Vec<Value>>();

            match compare(&lr[0], &lr[1]) {
                PacketOrdering::Ok => Some(index + 1),
                _ => None,
            }
        })
        .sum::<usize>();
    println!("Part1: {}", ans);
}

fn part2(data: &str) {
    let mut packets: Vec<Value> = Vec::new();
    data.lines().filter(|l| !l.is_empty()).for_each(|row| {
        let packet = serde_json::from_str(row).unwrap();
        packets.push(packet);
    });
    packets.push(serde_json::from_str("[[2]]").unwrap());
    packets.push(serde_json::from_str("[[6]]").unwrap());

    loop {
        let mut swapped = false;
        for i in 0..packets.len() - 1 {
            if let PacketOrdering::Wrong = compare(&packets[i], &packets[i + 1]) {
                packets.swap(i, i + 1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }

    let ans = packets
        .iter()
        .enumerate()
        .filter_map(|(idx, p)| {
            if is_divider_packet(p) {
                return Some(idx + 1);
            };
            None
        })
        .collect::<Vec<usize>>();
    println!("Part2: {}", ans[0] * ans[1]);
}

fn main() {
    let data = include_str!("../input");
    part1(data);
    part2(data);
}
