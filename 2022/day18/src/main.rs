#[derive(Debug)]
struct Cube(i32, i32, i32);

impl From<&str> for Cube {
    fn from(line: &str) -> Self {
        let coords = line
            .split(',')
            .map(|num| num.parse().unwrap())
            .collect::<Vec<i32>>();
        Cube(coords[0], coords[1], coords[2])
    }
}

impl Cube {
    fn collides_with(&self, other: &Cube) -> bool {
        let onx = (self.0 - 1 == other.0 || self.0 + 1 == other.0)
            && self.1 == other.1
            && self.2 == other.2;
        let ony = (self.1 - 1 == other.1 || self.1 + 1 == other.1)
            && self.0 == other.0
            && self.2 == other.2;
        let onz = (self.2 - 1 == other.2 || self.2 + 1 == other.2)
            && self.0 == other.0
            && self.1 == other.1;
        onx || ony || onz
    }
}

fn main() {
    let mut cubes = Vec::new();
    let mut sides = 0;
    include_str!("../input").lines().for_each(|line| {
        let cube = Cube::from(line);
        let mut toto = 6;
        for other_cube in &cubes {
            if cube.collides_with(other_cube) {
                toto -= 2;
            }
        }
        cubes.push(cube);
        sides += toto;
    });
    println!("{:?}", sides);
}
