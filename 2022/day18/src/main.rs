use std::collections::VecDeque;

#[derive(Debug, Clone, Copy)]
struct Cube {
    x: i32,
    y: i32,
    z: i32,
    sides: i32,
}

impl From<&str> for Cube {
    fn from(line: &str) -> Self {
        let coords = line
            .split(',')
            .map(|num| num.parse().unwrap())
            .collect::<Vec<i32>>();

        Cube {
            x: coords[0],
            y: coords[1],
            z: coords[2],
            sides: 6,
        }
    }
}

impl PartialEq for Cube {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Eq for Cube {}

impl Cube {
    fn from_coords(x: i32, y: i32, z: i32) -> Self {
        Cube { x, y, z, sides: 6 }
    }
    fn collides_with(&self, other: &Cube) -> bool {
        let onx = (self.x - 1 == other.x || self.x + 1 == other.x)
            && self.y == other.y
            && self.z == other.z;
        let ony = (self.y - 1 == other.y || self.y + 1 == other.y)
            && self.x == other.x
            && self.z == other.z;
        let onz = (self.z - 1 == other.z || self.z + 1 == other.z)
            && self.x == other.x
            && self.y == other.y;
        onx || ony || onz
    }

    fn get_neighbors(&self) -> Vec<Self> {
        vec![
            Cube::from_coords(self.x - 1, self.y, self.z),
            Cube::from_coords(self.x + 1, self.y, self.z),
            Cube::from_coords(self.x, self.y - 1, self.z),
            Cube::from_coords(self.x, self.y + 1, self.z),
            Cube::from_coords(self.x, self.y, self.z - 1),
            Cube::from_coords(self.x, self.y, self.z + 1),
        ]
    }
}

fn within_grid(point: &Cube, lower: &Cube, upper: &Cube) -> bool {
    let within_lower = point.x >= lower.x && point.y >= lower.y && point.z >= lower.z;
    let within_upper = point.x <= upper.x && point.y <= upper.y && point.z <= upper.z;
    within_lower && within_upper
}

fn find_air_pockets(start: Cube, end: Cube, cubes: &mut [Cube]) -> Vec<Cube> {
    let mut visited = Vec::new();
    let mut to_visit = VecDeque::new();
    to_visit.push_back(start);
    visited.push(start);

    while let Some(next_) = to_visit.pop_front() {
        for neighbor in next_.get_neighbors() {
            if !within_grid(&neighbor, &start, &end) {
                continue;
            }

            if !visited.contains(&neighbor) {
                if !cubes.contains(&neighbor) {
                    to_visit.push_back(neighbor);
                }
                visited.push(neighbor);
            }
        }
    }

    // Ugh.
    let mut air = vec![];
    for x in 0..end.x {
        for y in 0..end.y {
            for z in 0..end.z {
                let cand = Cube::from_coords(x, y, z);
                if !visited.contains(&cand) && !cubes.contains(&cand) {
                    air.push(cand);
                }
            }
        }
    }
    air
}

fn main() {
    let mut cubes = Vec::new();
    include_str!("../input").lines().for_each(|line| {
        let mut cube = Cube::from(line);
        for other_cube in &mut cubes.iter_mut() {
            if cube.collides_with(other_cube) {
                cube.sides -= 1;
                other_cube.sides -= 1;
            }
        }
        cubes.push(cube);
    });
    let total_droplet_surface = cubes.iter().map(|c| c.sides).sum::<i32>();
    println!("Part1: {}", total_droplet_surface);

    // This search space is big enough. :^)
    let mut air = find_air_pockets(
        Cube::from_coords(0, 0, 0),
        Cube::from_coords(21, 21, 21),
        &mut cubes,
    );

    let mut pocket = Vec::new();
    while let Some(mut bubble) = air.pop() {
        for other_bubble in &mut pocket.iter_mut() {
            if bubble.collides_with(other_bubble) {
                bubble.sides -= 1;
                other_bubble.sides -= 1;
            }
        }
        pocket.push(bubble);
    }
    let trapped_air_surface = pocket.iter().map(|c| c.sides).sum::<i32>();
    println!("Part2: {}", total_droplet_surface - trapped_air_surface);
}
