use itertools::EitherOrBoth::*;
use itertools::Itertools;
use serde::Deserialize;
use std::cmp::Ordering;

#[derive(Deserialize, PartialEq, Eq)]
#[serde(untagged)]
enum Packet {
    Val(u8),
    Arr(Vec<Packet>),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        use Packet::*;
        match (self, other) {
            (Val(left), Val(right)) => left.cmp(right),
            (Val(left), Arr(_)) => Arr(vec![Val(*left)]).cmp(other),
            (Arr(_), Val(right)) => self.cmp(&Arr(vec![Val(*right)])),
            (Arr(left), Arr(right)) => {
                for lr in left.iter().zip_longest(right.iter()) {
                    let ord = match lr {
                        Left(_) => Ordering::Greater,
                        Right(_) => Ordering::Less,
                        Both(a, b) => a.cmp(b),
                    };
                    if !ord.is_eq() {
                        return ord;
                    }
                }
                Ordering::Equal
            }
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn part1(data: &str) {
    let ans = data
        .split("\n\n")
        .enumerate()
        .filter(|(_, pair)| {
            let (left, right) = pair
                .split('\n')
                .map(|row| serde_json::from_str::<Packet>(row).unwrap())
                .collect_tuple::<(Packet, Packet)>()
                .unwrap();
            left < right
        })
        .map(|(index, _)| index + 1)
        .sum::<usize>();
    println!("Part1: {}", ans);
}

fn part2(data: &str) {
    use Packet::*;
    let mut packets: Vec<Packet> = Vec::new();
    data.lines().filter(|l| !l.is_empty()).for_each(|row| {
        let packet = serde_json::from_str::<Packet>(row).unwrap();
        packets.push(packet);
    });

    packets.push(serde_json::from_str::<Packet>("[[2]]").unwrap());
    packets.push(serde_json::from_str::<Packet>("[[6]]").unwrap());
    packets.sort_unstable();

    let ans: usize = [2, 6]
        .iter()
        .map(|divider| {
            packets
                .iter()
                .filter(|p| *p < &Arr(vec![Arr(vec![Val(*divider)])]))
                .count()
                + 1
        })
        .product();
    println!("Part2: {}", ans);
}

fn main() {
    let data = include_str!("../input");
    part1(data);
    part2(data);
}
