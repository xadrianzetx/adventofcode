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
        let mut ca = 0;
        let mut cm = false;
        (0..loc.0)
            .into_iter()
            .map(|r| trees.get(&(r, loc.1)).unwrap())
            .rev()
            .for_each(|t| {
                if !cm {
                    ca += 1;
                }
                if t >= tree {
                    cm = true
                }
            });
        
        let mut cb = 0;
        cm = false;
        (loc.0 + 1..rowcount)
            .into_iter()
            .map(|r| trees.get(&(r, loc.1)).unwrap())
            .for_each(|t| {
                if !cm {
                    cb += 1;
                }
                if t >= tree {
                    cm = true;
                }
            });
        
        let mut cc = 0;
        cm = false;
        (0..loc.1)
            .into_iter()
            .map(|r| trees.get(&(loc.0, r)).unwrap())
            .rev()
            .for_each(|t| {
                if !cm {
                    cc += 1;
                }
                if t >= tree {
                    cm = true;
                }
            });
        
        let mut cd = 0;
        cm = false;
        (loc.1 + 1..colcount)
            .into_iter()
            .map(|r| trees.get(&(loc.0, r)).unwrap())
            .for_each(|t| {
                if !cm {
                    cd += 1;
                }
                if t >= tree {
                    cm = true;
                }
            });
        
        let dist = ca * cb * cc * cd;
        if dist > totals {
            totals = dist;
        }
    }
    println!("{}", totals);
}
