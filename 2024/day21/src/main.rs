use std::{cmp::min, collections::HashMap, collections::HashSet, collections::VecDeque};

// Up, right, down, left
const DIRECTIONS: &[(i32, i32)] = &[(-1, 0), (0, 1), (1, 0), (0, -1)];
const HEADINGS: &[char] = &['^', '>', 'v', '<'];

type Paths = HashMap<((i32, i32), (i32, i32)), Vec<Vec<char>>>;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Key {
    Up,
    Down,
    Left,
    Right,
    A,
}

impl From<char> for Key {
    fn from(value: char) -> Self {
        match value {
            '^' => Self::Up,
            'v' => Self::Down,
            '<' => Self::Left,
            '>' => Self::Right,
            'A' => Self::A,
            _ => unreachable!(),
        }
    }
}

fn click(key: Key, from: Key, depth: u8, cache: &mut HashMap<(Key, Key, u8), usize>) -> usize {
    use Key::*;

    if depth == 0 {
        return 1;
    }

    if let Some(c) = cache.get(&(key, from, depth)) {
        return *c;
    }

    let cost = match (key, from) {
        // From A
        (Left, A) => {
            let c1 = click(Down, A, depth - 1, cache)
                + click(Left, Down, depth - 1, cache)
                + click(Left, Left, depth - 1, cache)
                + click(A, Left, depth - 1, cache);
            let c2 = click(Left, A, depth - 1, cache)
                + click(Down, Left, depth - 1, cache)
                + click(Left, Down, depth - 1, cache)
                + click(A, Left, depth - 1, cache);
            min(c1, c2)
        }
        (Down, A) => {
            let c1 = click(Down, A, depth - 1, cache)
                + click(Left, Down, depth - 1, cache)
                + click(A, Left, depth - 1, cache);
            let c2 = click(Left, A, depth - 1, cache)
                + click(Down, Left, depth - 1, cache)
                + click(A, Down, depth - 1, cache);
            min(c1, c2)
        }
        (Right, A) => click(Down, A, depth - 1, cache) + click(A, Down, depth - 1, cache),
        (Up, A) => click(Left, A, depth - 1, cache) + click(A, Left, depth - 1, cache),
        (A, A) => click(A, A, depth - 1, cache),
        // From Up
        (A, Up) => click(Right, A, depth - 1, cache) + click(A, Right, depth - 1, cache),
        (Left, Up) => {
            click(Down, A, depth - 1, cache)
                + click(Left, Down, depth - 1, cache)
                + click(A, Left, depth - 1, cache)
        }
        (Down, Up) => click(Down, A, depth - 1, cache) + click(A, Down, depth - 1, cache),
        (Right, Up) => {
            let c1 = click(Right, A, depth - 1, cache)
                + click(Down, Right, depth - 1, cache)
                + click(A, Down, depth - 1, cache);
            let c2 = click(Down, A, depth - 1, cache)
                + click(Right, Down, depth - 1, cache)
                + click(A, Right, depth - 1, cache);
            min(c1, c2)
        }
        (Up, Up) => click(A, A, depth - 1, cache),
        // From left
        (Left, Left) => click(A, A, depth - 1, cache),
        (Down, Left) => click(Right, A, depth - 1, cache) + click(A, Right, depth - 1, cache),
        (Right, Left) => {
            click(Right, A, depth - 1, cache)
                + click(Right, Right, depth - 1, cache)
                + click(A, Right, depth - 1, cache)
        }
        (Up, Left) => {
            click(Right, A, depth - 1, cache)
                + click(Up, Right, depth - 1, cache)
                + click(A, Up, depth - 1, cache)
        }
        (A, Left) => {
            let c1 = click(Right, A, depth - 1, cache)
                + click(Right, Right, depth - 1, cache)
                + click(Up, Right, depth - 1, cache)
                + click(A, Up, depth - 1, cache);
            let c2 = click(Right, A, depth - 1, cache)
                + click(Up, Right, depth - 1, cache)
                + click(Right, Up, depth - 1, cache)
                + click(A, Right, depth - 1, cache);
            min(c1, c2)
        }
        // From down
        (Left, Down) => click(Left, A, depth - 1, cache) + click(A, Left, depth - 1, cache),
        (Down, Down) => click(A, A, depth - 1, cache),
        (Right, Down) => click(Right, A, depth - 1, cache) + click(A, Right, depth - 1, cache),
        (Up, Down) => click(Up, A, depth - 1, cache) + click(A, Up, depth - 1, cache),
        (A, Down) => {
            let c1 = click(Up, A, depth - 1, cache)
                + click(Right, Up, depth - 1, cache)
                + click(A, Right, depth - 1, cache);
            let c2 = click(Right, A, depth - 1, cache)
                + click(Up, Right, depth - 1, cache)
                + click(A, Up, depth - 1, cache);
            min(c1, c2)
        }
        // From right
        (Left, Right) => {
            click(Left, A, depth - 1, cache)
                + click(Left, Left, depth - 1, cache)
                + click(A, Left, depth - 1, cache)
        }
        (Down, Right) => click(Left, A, depth - 1, cache) + click(A, Left, depth - 1, cache),
        (Right, Right) => click(A, A, depth - 1, cache),
        (Up, Right) => {
            let c1 = click(Up, A, depth - 1, cache)
                + click(Left, Up, depth - 1, cache)
                + click(A, Left, depth - 1, cache);
            let c2 = click(Left, A, depth - 1, cache)
                + click(Up, Left, depth - 1, cache)
                + click(A, Up, depth - 1, cache);
            min(c1, c2)
        }
        (A, Right) => click(Up, A, depth - 1, cache) + click(A, Up, depth - 1, cache),
    };

    cache.insert((key, from, depth), cost);
    cost
}

fn find_paths_between(map: &HashMap<(i32, i32), char>) -> Paths {
    let mut paths = HashMap::new();
    for src in map.keys() {
        for dst in map.keys() {
            if src == dst {
                paths.insert((*src, *dst), [Vec::from(['A'])].to_vec());
                continue;
            }

            let valid_paths = bfs(map, src, dst);
            let min_length = valid_paths.iter().map(|p| p.len()).min().unwrap();
            let shortest_paths = valid_paths
                .into_iter()
                .filter(|p| p.len() == min_length)
                .collect::<Vec<Vec<char>>>();
            paths.insert((*src, *dst), shortest_paths);
        }
    }
    paths
}

fn bfs(map: &HashMap<(i32, i32), char>, start: &(i32, i32), end: &(i32, i32)) -> Vec<Vec<char>> {
    let mut queue = VecDeque::new();
    queue.push_back((start.0, start.1, Vec::new(), HashSet::from([*start])));

    let mut seen = HashSet::new();
    seen.insert(*start);

    let mut valid_paths = Vec::new();

    while let Some(elem) = queue.pop_front() {
        for (idx, direction) in DIRECTIONS.iter().enumerate() {
            let newpos = (elem.0 + direction.0, elem.1 + direction.1);
            let newheading = HEADINGS[idx];
            if &newpos == end {
                let mut valid_path = elem.2.clone();
                valid_path.push(newheading);
                valid_path.push('A');
                valid_paths.push(valid_path);
                continue;
            }

            if elem.3.contains(&newpos) {
                continue;
            }

            if map.get(&newpos).is_some() {
                let mut newpth = elem.2.clone();
                let mut newseen = elem.3.clone();
                newpth.push(newheading);
                newseen.insert(newpos);
                queue.push_back((newpos.0, newpos.1, newpth, newseen));
                seen.insert(newpos);
            }
        }
    }

    valid_paths
}

fn prepare_initial_paths(code: &str, paths_lookup: &Paths) -> Vec<Vec<Key>> {
    let numeric_keypad_flipped = HashMap::from([
        ('7', (0, 0)),
        ('8', (0, 1)),
        ('9', (0, 2)),
        ('4', (1, 0)),
        ('5', (1, 1)),
        ('6', (1, 2)),
        ('1', (2, 0)),
        ('2', (2, 1)),
        ('3', (2, 2)),
        ('0', (3, 1)),
        ('A', (3, 2)),
    ]);

    let mut pathbuff: Vec<Vec<char>> = Vec::new();
    for w in code.chars().collect::<Vec<char>>().windows(2) {
        let src = numeric_keypad_flipped.get(&w[0]).unwrap();
        let dest = numeric_keypad_flipped.get(&w[1]).unwrap();
        let p = paths_lookup.get(&(*src, *dest)).unwrap();

        if pathbuff.is_empty() {
            pathbuff = p.to_vec();
            continue;
        }

        let mut newpathbuff = Vec::new();
        for pth in pathbuff.iter() {
            for pp in p {
                let mut updated = pth.clone();
                updated.append(&mut pp.clone());
                newpathbuff.push(updated);
            }
        }
        pathbuff = newpathbuff;
    }

    pathbuff
        .into_iter()
        .map(|p| p.into_iter().map(Key::from).collect::<Vec<Key>>())
        .collect::<Vec<Vec<Key>>>()
}

fn main() {
    use Key::*;

    let numeric_keypad = HashMap::from([
        ((0, 0), '7'),
        ((0, 1), '8'),
        ((0, 2), '9'),
        ((1, 0), '4'),
        ((1, 1), '5'),
        ((1, 2), '6'),
        ((2, 0), '1'),
        ((2, 1), '2'),
        ((2, 2), '3'),
        ((3, 1), '0'),
        ((3, 2), 'A'),
    ]);

    let numeric_paths = find_paths_between(&numeric_keypad);
    let codes = ["A279A", "A286A", "A508A", "A463A", "A246A"];

    let mut part_1 = 0;
    let mut part_2 = 0;

    for code in codes {
        let mut initial_paths = prepare_initial_paths(code, &numeric_paths);
        let mut costs_p1 = Vec::new();
        let mut costs_p2 = Vec::new();

        for path in initial_paths.iter_mut() {
            path.insert(0, A);
            let mut cost_p1 = 0;
            let mut cost_p2 = 0;
            let mut cache = HashMap::new();
            for items in path.windows(2) {
                cost_p1 += click(items[1], items[0], 2, &mut cache);
                cost_p2 += click(items[1], items[0], 25, &mut cache);
            }
            costs_p1.push(cost_p1);
            costs_p2.push(cost_p2);
        }

        let min_cost_p1 = costs_p1.iter().min().unwrap();
        let min_cost_p2 = costs_p2.iter().min().unwrap();
        let c = code.to_string();
        let multiplier = c
            .strip_prefix('A')
            .unwrap()
            .to_string()
            .strip_suffix('A')
            .unwrap()
            .parse::<usize>()
            .unwrap();

        part_1 += multiplier * min_cost_p1;
        part_2 += multiplier * min_cost_p2;
    }

    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");
}
