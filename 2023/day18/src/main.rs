use std::collections::HashSet;

// Up, right, down, left.
const DIRECTIONS: &[(i64, i64)] = &[(-1, 0), (0, 1), (1, 0), (0, -1)];

fn get_directions(raw_directions: &str) -> usize {
    match raw_directions {
        "U" => 0,
        "R" => 1,
        "D" => 2,
        "L" => 3,
        _ => unreachable!(),
    }
}

fn rotate_directions(raw_directions: usize) -> usize {
    (raw_directions + 1) % 4
}

fn measure_lagoon(vertices: Vec<(i64, i64)>) -> i64 {
    let mut rows = vertices
        .iter()
        .map(|e| e.0)
        .collect::<HashSet<i64>>()
        .into_iter()
        .collect::<Vec<i64>>();
    rows.sort();

    let mut nextrows = rows.clone();
    nextrows.push(rows.last().unwrap() - 1);

    let mut ranges: Vec<(i64, i64)> = Vec::new();
    let mut cnt = 0;

    for (row, nextrow) in rows.iter().zip(nextrows.iter().skip(1)) {
        let mut r = vertices
            .iter()
            .filter(|e| e.0 == *row)
            .collect::<Vec<&(i64, i64)>>();
        r.sort_by_key(|k| k.1);

        for pair in r.chunks(2) {
            let newrng = (pair[0].1, pair[1].1);
            let mut toadd = true;
            let mut splitat = None;
            let mut splitrng = None;
            let mut ending = None;
            for (idx, range) in ranges.iter_mut().enumerate() {
                if range == &newrng {
                    // Range ends
                    if ranges.len() > 1 {
                        ending = Some(idx);
                    }
                    toadd = false;
                    break;
                }

                if newrng.0 == range.1 {
                    // Extending right.
                    *range = (range.0, newrng.1);
                    toadd = false;
                } else if newrng.1 == range.0 {
                    // Extending left.
                    *range = (newrng.0, range.1);
                    toadd = false;
                } else if newrng.0 == range.0 {
                    // Tighten left
                    cnt += (newrng.1 - newrng.0).abs();
                    *range = (newrng.1, range.1);
                    toadd = false;
                } else if newrng.1 == range.1 {
                    // Tighten right
                    cnt += (newrng.1 - newrng.0).abs();
                    *range = (range.0, newrng.0);
                    toadd = false;
                } else if newrng.0 > range.0 && newrng.1 < range.1 {
                    // Split
                    // println!("SPLITS {range:?}, {newrng:?}");
                    cnt += (newrng.1 - newrng.0).abs();
                    splitat = Some(idx);
                    splitrng = Some(vec![(range.0, newrng.0), (newrng.1, range.1)]);
                    toadd = false;
                } else {
                    // Shitcode!
                }
            }

            if toadd {
                ranges.push(newrng);
            }
            if let Some(ended) = ending {
                cnt += (ranges[ended].1 - ranges[ended].0).abs();
                ranges.remove(ended);
            }
            if let Some(splitidx) = splitat {
                ranges = [
                    &ranges[..splitidx],
                    &splitrng.unwrap(),
                    &ranges[splitidx + 1..],
                ]
                .concat();
            }
            ranges.sort_by_key(|k| k.0);
            // Range concatenated
            let mut i = 0;
            while i < ranges.len() - 1 {
                if ranges[i].1 >= ranges[i + 1].0 {
                    // println!("CONCAT {:?} {:?}", ranges[i], ranges[i + 1]);
                    let a = ranges.remove(i);
                    let b = ranges.remove(i);
                    ranges.push((a.0, b.1));
                    break;
                } else {
                    i += 1;
                }
            }
        }

        let mul = (nextrow - row).abs();
        cnt += ranges
            .iter()
            .map(|rng| ((rng.1 - rng.0).abs() + 1) * mul)
            .sum::<i64>();
    }

    cnt
}

fn small_plan(raw_plan: &str) -> Vec<(i64, i64)> {
    let mut buff = Vec::new();
    let mut curr = (0, 0);
    for line in raw_plan.lines() {
        let mut plan = line.split(' ');
        let direction = get_directions(plan.next().unwrap());
        let distance = plan.next().unwrap().parse::<i64>().unwrap();
        buff.push((curr.0, curr.1));
        curr = (
            curr.0 + (DIRECTIONS[direction].0 * distance),
            curr.1 + (DIRECTIONS[direction].1 * distance),
        );
    }
    buff
}

fn big_plan(raw_plan: &str) -> Vec<(i64, i64)> {
    let mut buff = Vec::new();
    let mut curr = (0, 0);
    for line in raw_plan.lines() {
        let plan = line.split(' ');
        let mut hex = plan.last().unwrap().replace(['(', ')', '#'], "");
        let direction = rotate_directions(hex.pop().unwrap().to_digit(10).unwrap() as usize);
        let distance = i64::from_str_radix(&hex, 16).unwrap();
        buff.push((curr.0, curr.1));
        curr = (
            curr.0 + (DIRECTIONS[direction].0 * distance),
            curr.1 + (DIRECTIONS[direction].1 * distance),
        );
    }
    buff
}

fn main() {
    let raw_plan = include_str!("../input");
    println!("Part 1: {}", measure_lagoon(small_plan(raw_plan)));
    println!("Part 2: {}", measure_lagoon(big_plan(raw_plan)));
}
