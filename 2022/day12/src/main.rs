use petgraph::graphmap::DiGraphMap;
use std::collections::VecDeque;
use std::collections::{HashMap, HashSet};

static OFFSETS: &[(i32, i32)] = &[(0, 1), (0, -1), (1, 0), (-1, 0)];
type Steps = HashMap<(i32, i32), (i32, i32)>;

fn parse_height(chr: char) -> i32 {
    (chr as u8 - b'a') as i32
}

fn bfs(g: &DiGraphMap<(i32, i32), ()>, start: &(i32, i32)) -> Steps {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut steps = HashMap::new();

    queue.push_back(*start);
    visited.insert(*start);
    while let Some(node) = queue.pop_front() {
        let neighbors = g.neighbors(node);
        neighbors.for_each(|n| {
            if !visited.contains(&n) {
                steps.insert(n, node);
                queue.push_back(n);
                visited.insert(n);
            }
        });
    }
    steps
}

fn walk_back(steps: &Steps, finish: &(i32, i32)) -> Option<i32> {
    let mut next = finish;
    if !steps.contains_key(next) {
        return None;
    }

    let mut pth = 0;
    while let Some(step) = steps.get(next) {
        pth += 1;
        next = step;
    }
    Some(pth)
}

fn main() {
    // TODO(xadrianzetx) Refactor this mess.
    let mut start = (0, 0);
    let mut top = (0, 0);
    let mut start_points = Vec::new();
    let mut terrain = HashMap::new();
    include_str!("../input")
        .lines()
        .enumerate()
        .for_each(|(row, line)| {
            line.chars().enumerate().for_each(|(col, mut chr)| {
                if chr == 'S' {
                    start = (row as i32, col as i32);
                    chr = 'a';
                }
                if chr == 'E' {
                    top = (row as i32, col as i32);
                    chr = 'z';
                }
                if chr == 'a' {
                    start_points.push((row as i32, col as i32));
                }
                terrain.insert((row as i32, col as i32), parse_height(chr));
            });
        });

    let mut g = DiGraphMap::<(i32, i32), ()>::new();
    terrain.iter().for_each(|(node, weight)| {
        OFFSETS.iter().for_each(|offset| {
            let target = (node.0 + offset.0, node.1 + offset.1);
            if terrain.contains_key(&target) {
                let delta = terrain.get(&target).unwrap() - weight;
                if delta <= 1 {
                    g.add_node(*node);
                    g.add_node(target);
                    g.add_edge(target, *node, ());
                }
            }
        })
    });

    let paths = bfs(&g, &top);
    let distances = start_points
        .iter()
        .filter_map(|f| walk_back(&paths, f).map(|path| (f, path)))
        .collect::<HashMap<&(i32, i32), i32>>();

    println!("Part1: {}", distances.get(&start).unwrap());
    println!("Part2: {}", distances.values().min().unwrap());
}
