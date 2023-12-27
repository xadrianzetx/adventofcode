#[derive(Debug)]
struct Hailstone {
    position: (f64, f64, f64),
    velocity: (f64, f64, f64),
}

impl From<&str> for Hailstone {
    fn from(value: &str) -> Self {
        let mut pv = value.split(" @ ");
        let p = pv
            .next()
            .unwrap()
            .split(',')
            .into_iter()
            .map(|p| p.trim().parse::<f64>().unwrap())
            .collect::<Vec<f64>>();

        let v = pv
            .next()
            .unwrap()
            .split(',')
            .into_iter()
            .map(|v| v.trim().parse::<f64>().unwrap())
            .collect::<Vec<f64>>();

        Self {
            position: (p[0], p[1], p[2]),
            velocity: (v[0], v[1], v[2]),
        }
    }
}

impl Hailstone {
    fn collides_within_area_2d(&self, other: &Hailstone, area: &(f64, f64)) -> bool {
        let p1 = (
            self.position.0 + self.velocity.0,
            self.position.1 + self.velocity.1,
        );
        let p2 = (
            other.position.0 + other.velocity.0,
            other.position.1 + other.velocity.1,
        );

        let s1 = (p1.1 - self.position.1) / (p1.0 - self.position.0);
        let s2 = (p2.1 - other.position.1) / (p2.0 - other.position.0);

        let i1 = self.position.1 - s1 * self.position.0;
        let i2 = other.position.1 - s2 * other.position.0;

        let cx = (i2 - i1) / (s1 - s2);
        let cy = s1 * cx + i1;

        if self.position.0 < cx && self.velocity.0 < 0.0
            || other.position.0 < cx && other.velocity.0 < 0.0
            || self.position.0 > cx && self.velocity.0 > 0.0
            || other.position.0 > cx && other.velocity.0 > 0.0
            || self.position.1 < cy && self.velocity.1 < 0.0
            || other.position.1 < cy && other.velocity.1 < 0.0
            || self.position.1 > cy && self.velocity.1 > 0.0
            || other.position.1 > cy && other.velocity.1 > 0.0
        {
            // Collision before current x or y pos.
            return false;
        }

        area.0 <= cx && cx <= area.1 && area.0 <= cy && cy <= area.1
    }
}

fn count_collisions_2d(hailstones: &Vec<Hailstone>, area: &(f64, f64)) -> usize {
    let mut cnt = 0;
    for i in 0..hailstones.len() {
        let h = hailstones.get(i).unwrap();
        let others = &hailstones[i + 1..];
        for other in others {
            if h.collides_within_area_2d(other, area) {
                cnt += 1;
            }
        }
    }
    cnt
}

fn main() {
    let hailstones = include_str!("../input")
        .lines()
        .map(Hailstone::from)
        .collect::<Vec<Hailstone>>();

    let part_1 = count_collisions_2d(&hailstones, &(200000000000000., 400000000000000.));
    println!("{part_1}");
}
