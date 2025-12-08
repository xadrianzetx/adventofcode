use {std::collections::HashMap, std::collections::HashSet};

#[derive(Debug)]
struct JunctionBox {
    x: isize,
    y: isize,
    z: isize,

    connection_index: Option<usize>,
}

impl From<&str> for JunctionBox {
    fn from(value: &str) -> Self {
        let mut coords = value.split(',');
        Self {
            x: coords.next().unwrap().parse::<isize>().unwrap(),
            y: coords.next().unwrap().parse::<isize>().unwrap(),
            z: coords.next().unwrap().parse::<isize>().unwrap(),
            connection_index: None,
        }
    }
}

impl JunctionBox {
    fn euclidean_distance(&self, other: &JunctionBox) -> f32 {
        let d = (self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2);
        (d as f32).sqrt()
    }

    fn set_connection(&mut self, index: usize) {
        self.connection_index = Some(index);
    }
}

#[derive(Debug)]
struct BoxDistance {
    left_box_index: usize,
    right_box_index: usize,
    distance: f32,
}

fn get_distances(junction_boxes: &[JunctionBox]) -> Vec<BoxDistance> {
    let num_boxes = junction_boxes.len();
    let mut distances = Vec::new();

    for (idx_left, left_junction_box) in junction_boxes.iter().enumerate() {
        for (idx_right, right_junction_box) in junction_boxes
            .iter()
            .enumerate()
            .take(num_boxes)
            .skip(idx_left + 1)
        {
            let d = BoxDistance {
                left_box_index: idx_left,
                right_box_index: idx_right,
                distance: left_junction_box.euclidean_distance(right_junction_box),
            };
            distances.push(d);
        }
    }

    distances.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());
    distances
}

fn get_biggest_circuits(junction_boxes: &[JunctionBox]) -> i32 {
    let mut counts = HashMap::new();
    for junction_box in junction_boxes.iter() {
        if let Some(connection_id) = junction_box.connection_index {
            counts
                .entry(connection_id)
                .and_modify(|entry| *entry += 1)
                .or_insert(1);
        }
    }

    let mut circuit_sizes = counts.values().collect::<Vec<&i32>>();
    circuit_sizes.sort();
    circuit_sizes.reverse();

    let mut acc = 1;
    for el in circuit_sizes.iter().take(3) {
        acc *= *el;
    }

    acc
}

fn main() {
    let data = include_str!("input");

    let mut junction_boxes = data
        .lines()
        .map(JunctionBox::from)
        .collect::<Vec<JunctionBox>>();

    let distances = get_distances(&junction_boxes);

    let mut current_connection_id = 0;
    let mut connections_made = 0;

    for d in distances.iter() {
        let l = &junction_boxes[d.left_box_index].connection_index;
        let r = &junction_boxes[d.right_box_index].connection_index;

        if l.is_none() && r.is_none() {
            junction_boxes[d.left_box_index].set_connection(current_connection_id);
            junction_boxes[d.right_box_index].set_connection(current_connection_id);

            current_connection_id += 1;
            connections_made += 1;
        } else if l.is_none() && r.is_some() {
            let carry_id = r.unwrap();
            junction_boxes[d.left_box_index].set_connection(carry_id);
            connections_made += 1;
        } else if l.is_some() && r.is_none() {
            let carry_id = l.unwrap();
            junction_boxes[d.right_box_index].set_connection(carry_id);
            connections_made += 1;
        } else {
            if l.unwrap() != r.unwrap() {
                let carry_id = l.unwrap();
                let old_id = r.unwrap();
                for b in junction_boxes.iter_mut() {
                    if let Some(current_id) = b.connection_index {
                        if current_id == old_id {
                            b.set_connection(carry_id);
                        }
                    }
                }
            }
            connections_made += 1;
        }

        if connections_made == 1000 {
            println!("Part 1: {}", get_biggest_circuits(&junction_boxes));
        }

        let mut all_connections = HashSet::new();
        for b in junction_boxes.iter() {
            all_connections.insert(b.connection_index);
        }

        if all_connections.len() == 1 && connections_made > 1 {
            let x_distance =
                junction_boxes[d.left_box_index].x * junction_boxes[d.right_box_index].x;

            println!("Part 2: {x_distance}");
            break;
        }
    }
}
