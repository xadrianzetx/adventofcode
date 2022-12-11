use lazy_static::lazy_static;
use num::integer::lcm;
use regex::Regex;
use std::collections::VecDeque;

#[derive(Debug)]
enum Operations {
    Add(usize),
    Mul(usize),
    Sqr,
}

struct Item {
    to: usize,
    item: usize,
}

impl From<&str> for Operations {
    fn from(description: &str) -> Self {
        let raw_op = description
            .split_whitespace()
            .rev()
            .take(2)
            .collect::<Vec<&str>>();
        match raw_op[1] {
            "+" => Operations::Add(raw_op[0].parse::<usize>().unwrap()),
            "*" => {
                if let Ok(amt) = raw_op[0].parse::<usize>() {
                    return Self::Mul(amt);
                }
                Self::Sqr
            }
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<usize>,
    op: Operations,
    test_condition: usize,
    target_true: usize,
    target_false: usize,
    inspected: usize,
}

impl Monkey {
    fn inspect_and_throw(&mut self, lcm: usize) -> Vec<Item> {
        let mut items = Vec::new();
        while !self.items.is_empty() {
            let item = self.items.pop_front().unwrap();
            let worry = match self.op {
                Operations::Add(amt) => item + amt,
                Operations::Mul(amt) => item * amt,
                Operations::Sqr => item * item,
            };

            if worry % self.test_condition == 0 {
                items.push(Item {
                    to: self.target_true,
                    item: worry % lcm,
                });
            } else {
                items.push(Item {
                    to: self.target_false,
                    item: worry % lcm,
                });
            }
            self.inspected += 1;
        }
        items
    }

    fn catch(&mut self, item: usize) {
        self.items.push_back(item);
    }
}

impl From<&str> for Monkey {
    fn from(description: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\d+").unwrap();
        }

        let lines: Vec<&str> = description.lines().collect();
        let items = RE
            .find_iter(lines[1])
            .filter_map(|digits| digits.as_str().parse().ok())
            .collect::<VecDeque<usize>>();

        let op = Operations::from(lines[2]);

        let test_condition = RE
            .find(lines[3])
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();
        let target_true = RE
            .find(lines[4])
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();
        let target_false = RE
            .find(lines[5])
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();
        Monkey {
            items,
            op,
            test_condition,
            target_true,
            target_false,
            inspected: 0,
        }
    }
}

fn main() {
    let mut monkeys = Vec::new();
    include_str!("../input")
        .split("\n\n")
        .for_each(|description| {
            monkeys.push(Monkey::from(description));
        });

    let lowest_common_multiple = monkeys
        .iter()
        .map(|m| m.test_condition)
        .reduce(lcm)
        .unwrap();

    for _ in 0..10000 {
        for monkey in 0..monkeys.len() {
            let passed = monkeys[monkey].inspect_and_throw(lowest_common_multiple);
            passed.iter().for_each(|item| {
                monkeys[item.to].catch(item.item);
            })
        }
    }

    let mut inspected = monkeys.iter().map(|m| m.inspected).collect::<Vec<usize>>();
    inspected.sort_unstable();
    inspected.reverse();
    println!("Part2: {}", inspected[0] * inspected[1]);
}
