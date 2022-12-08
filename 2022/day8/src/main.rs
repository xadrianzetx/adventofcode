use std::collections::HashMap;

fn main() {
    let mut trees = HashMap::new();
    let mut rowcount: usize = 0;
    let mut colcount: usize = 0;
    include_str!("../input").lines().for_each(|line| {
        colcount = line.chars().count();
        line.chars().enumerate().for_each(|c| {
            trees.insert((rowcount, c.0), c.1.to_digit(10).unwrap() as i32);
        });
        rowcount += 1;
    });

    let mut totals = 0;
    for (loc, tree) in &trees {
        let a = ((0..loc.0)
            .into_iter()
            .map(|r| trees.get(&(r, loc.1)).unwrap())
            .max()
            .unwrap_or(&-1)
            < tree) as usize;

        let b = ((loc.0 + 1..rowcount)
            .into_iter()
            .map(|r| trees.get(&(r, loc.1)).unwrap())
            .max()
            .unwrap_or(&-1)
            < tree) as usize;

        let c = ((0..loc.1)
            .into_iter()
            .map(|r| trees.get(&(loc.0, r)).unwrap())
            .max()
            .unwrap_or(&-1)
            < tree) as usize;

        let d = ((loc.1 + 1..colcount)
            .into_iter()
            .map(|r| trees.get(&(loc.0, r)).unwrap())
            .max()
            .unwrap_or(&-1)
            < tree) as usize;

        if a + b + c + d > 0 {
            totals += 1;
            // println!("{:?} visible", loc);
        }
    }
    println!("{}", totals);
}
