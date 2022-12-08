use std::cmp::max;
use std::collections::HashMap;

fn main() {
    let mut trees = HashMap::new();
    let mut rowcount: usize = 0;
    let mut colcount: usize = 0;
    include_str!("../input").lines().for_each(|line| {
        colcount = line.chars().count();
        line.chars().enumerate().for_each(|c| {
            trees.insert((rowcount, c.0), c.1.to_digit(10).unwrap());
        });
        rowcount += 1;
    });

    let ans = trees
        .iter()
        .map(|t| {
            let coord = t.0;
            let tree = t.1;
            let ans = vec![
                (0..coord.0)
                    .into_iter()
                    .map(|c| trees.get(&(c, coord.1)).unwrap().to_owned())
                    .rev()
                    .collect::<Vec<u32>>(),
                (coord.0 + 1..rowcount)
                    .map(|c| trees.get(&(c, coord.1)).unwrap().to_owned())
                    .collect(),
                (0..coord.1)
                    .map(|c| trees.get(&(coord.0, c)).unwrap().to_owned())
                    .rev()
                    .collect(),
                (coord.1 + 1..colcount)
                    .map(|c| trees.get(&(coord.0, c)).unwrap().to_owned())
                    .collect(),
            ]
            .iter()
            .map(|neighbors| {
                let mut visible = true;
                let mut score: usize = 0;
                neighbors.iter().for_each(|t| {
                    if visible {
                        score += 1;
                    }
                    if t >= tree {
                        visible = false;
                    }
                });
                (visible, score)
            })
            .reduce(|a, b| (a.0 | b.0, a.1 * b.1))
            .unwrap();
            (ans.0 as usize, ans.1)
        })
        .reduce(|a, b| (a.0 + b.0, max(a.1, b.1)))
        .unwrap();

    println!("Part1: {}", ans.0);
    println!("Part2: {}", ans.1);
}
