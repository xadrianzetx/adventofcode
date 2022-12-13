use itertools::EitherOrBoth::{Both, Left, Right};
use itertools::Itertools;
use serde::Deserialize;
use std::cmp::Ordering;

#[derive(PartialEq, Eq)]
enum PacketOrdering {
    Ok,
    Wrong,
    Undefined,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum Packet {
    Val(u8),
    Arr(Vec<Packet>),
}

fn compare(left: &Packet, right: &Packet) -> PacketOrdering {
    // TODO(xadrianzetx) Impl Ord
    // https://github.com/timvisee/advent-of-code-2022/blob/master/day13b/src/main.rs
    use Packet::*;
    match (left, right) {
        (Arr(l), Arr(r)) => {
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
        (Val(l), Val(r)) => match l.cmp(r) {
            Ordering::Less => PacketOrdering::Ok,
            Ordering::Greater => PacketOrdering::Wrong,
            Ordering::Equal => PacketOrdering::Undefined,
        },
        (Val(l), Arr(_)) => compare(&Arr(vec![Val(*l)]), right),
        (Arr(_), Val(r)) => compare(left, &Arr(vec![Val(*r)])),
    }
}

fn is_divider_packet(packets: &Packet) -> bool {
    use Packet::*;
    match packets {
        &Val(n) => [2, 6].contains(&n),
        Arr(a) => {
            if a.is_empty() || a.len() > 1 {
                return false;
            }
            is_divider_packet(&a[0])
        }
    }
}

fn part1(data: &str) {
    let ans = data
        .split("\n\n")
        .enumerate()
        .filter_map(|(index, pair)| {
            let lr = pair
                .split('\n')
                .map(|row| serde_json::from_str::<Packet>(row).unwrap())
                .collect::<Vec<Packet>>();

            match compare(&lr[0], &lr[1]) {
                PacketOrdering::Ok => Some(index + 1),
                _ => None,
            }
        })
        .sum::<usize>();
    println!("Part1: {}", ans);
}

fn part2(data: &str) {
    let mut packets: Vec<Packet> = Vec::new();
    data.lines().filter(|l| !l.is_empty()).for_each(|row| {
        let packet = serde_json::from_str::<Packet>(row).unwrap();
        packets.push(packet);
    });
    packets.push(serde_json::from_str::<Packet>("[[2]]").unwrap());
    packets.push(serde_json::from_str::<Packet>("[[6]]").unwrap());

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
