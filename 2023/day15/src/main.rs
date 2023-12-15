use std::collections::HashMap;

#[derive(Debug)]
struct Lens {
    label: String,
    focal: usize,
}

impl Lens {
    fn new(label: &str, focal: usize) -> Self {
        Lens {
            label: label.to_string(),
            focal,
        }
    }
}

#[derive(Debug)]
struct Boxes(HashMap<usize, Vec<Lens>>);

impl Boxes {
    fn new() -> Self {
        Boxes(HashMap::new())
    }

    fn get_or_make_box(&mut self, box_id: usize) -> &mut Vec<Lens> {
        self.0.entry(box_id).or_insert_with(Vec::new)
    }

    fn add(&mut self, box_id: usize, new_lens: Lens) {
        let lens_box = self.get_or_make_box(box_id);
        let to_update = lens_box
            .iter()
            .enumerate()
            .find(|(_, lens)| lens.label == new_lens.label);

        match to_update {
            Some((index, _)) => {
                let lens = lens_box.get_mut(index).unwrap();
                lens.focal = new_lens.focal;
            }
            None => lens_box.push(new_lens),
        }
    }

    fn remove(&mut self, box_id: usize, label: String) {
        let lens_box = self.get_or_make_box(box_id);
        let to_remove = lens_box
            .iter()
            .enumerate()
            .find(|(_, lens)| lens.label == label);

        if let Some((index, _)) = to_remove {
            lens_box.remove(index);
        }
    }

    fn get_focusing_power(&self) -> usize {
        let mut focusing_power = 0;
        for (box_id, lenses) in &self.0 {
            for (slot, lens) in lenses.iter().enumerate() {
                focusing_power += (box_id + 1) * (slot + 1) * lens.focal;
            }
        }
        focusing_power
    }
}

fn hash(step: &str) -> usize {
    let mut hash = 0;
    for char in step.chars() {
        hash = ((hash + char as usize) * 17) % 256;
    }
    hash
}

fn arrange_lenses(init_sequence: &str) -> Boxes {
    let mut boxes = Boxes::new();
    for step in init_sequence.split(',') {
        if step.contains('-') {
            let label = step.replace('-', "");
            boxes.remove(hash(&label), label);
        } else {
            let mut label_and_length = step.split('=');
            let label = label_and_length.next().unwrap();
            let focal = label_and_length.next().unwrap().parse::<usize>().unwrap();
            boxes.add(hash(label), Lens::new(label, focal));
        }
    }
    boxes
}

fn main() {
    let init_sequence = include_str!("../input");
    let part_1 = init_sequence.split(',').map(hash).sum::<usize>();
    println!("Part 1: {part_1}");

    let boxes = arrange_lenses(init_sequence);
    println!("Part 2: {}", boxes.get_focusing_power());
}
