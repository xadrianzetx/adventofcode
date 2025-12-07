use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
enum ManifoldItem {
    // Beam has number of timelines it exists in.
    Beam(usize),
    // Slitter has hit latch.
    Splitter(bool),
}

impl From<char> for ManifoldItem {
    fn from(value: char) -> Self {
        match value {
            'S' | '|' => Self::Beam(1),
            '^' => Self::Splitter(false),
            _ => unreachable!(),
        }
    }
}

type Manifold = HashMap<(usize, usize), ManifoldItem>;

fn make_manifold(data: &str) -> (Manifold, (usize, usize)) {
    let mut map = HashMap::new();

    let mut height = 0;
    let mut width = 0;

    for (row, line) in data.lines().enumerate() {
        for (col, chr) in line.chars().enumerate() {
            width = col;
            if chr != '.' {
                map.insert((row, col), ManifoldItem::from(chr));
            }
        }
        height = row;
    }

    (map, (height, width))
}

fn trace_beam(m: &mut Manifold, dimensions: (usize, usize)) {
    for row in 1..=dimensions.0 {
        for col in 0..=dimensions.1 {
            trace_beam_at_position((row, col), m);
        }
    }
}

fn trace_beam_at_position(position: (usize, usize), m: &mut Manifold) {
    if let Some(ManifoldItem::Beam(timelines)) = m.get(&(position.0 - 1, position.1)).cloned() {
        if let Some(ManifoldItem::Splitter(hit)) = m.get_mut(&position) {
            *hit = true;

            continue_beam((position.0, position.1 - 1), timelines, m);
            continue_beam((position.0, position.1 + 1), timelines, m);
        } else {
            continue_beam(position, timelines, m);
        }
    }
}

fn continue_beam(position: (usize, usize), timelines: usize, m: &mut Manifold) {
    m.entry(position)
        .and_modify(|entry| {
            if let ManifoldItem::Beam(existing_timelines) = entry {
                *existing_timelines += timelines;
            }
        })
        .or_insert(ManifoldItem::Beam(timelines));
}

fn main() {
    let data = include_str!("input");

    let (mut map, dimensions) = make_manifold(data);
    trace_beam(&mut map, dimensions);

    let part_1 = map
        .values()
        .filter_map(|manifold_item| {
            if let ManifoldItem::Splitter(true) = *manifold_item {
                return Some(1);
            }
            None
        })
        .sum::<usize>();

    println!("Part 1: {part_1}");

    let part_2 = map
        .iter()
        .filter(|(position, _)| position.0 == dimensions.0)
        .filter_map(|(_, manifold_item)| {
            if let ManifoldItem::Beam(timelines) = *manifold_item {
                return Some(timelines);
            }
            None
        })
        .sum::<usize>();

    println!("Part 2: {part_2}");
}
