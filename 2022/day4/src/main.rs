struct Sections {
    lower: u32,
    upper: u32,
}

impl From<&str> for Sections {
    fn from(bounds: &str) -> Self {
        let mut split = bounds.split('-');
        let lower = split.next().unwrap().parse().unwrap();
        let upper = split.next().unwrap().parse().unwrap();
        Sections { lower, upper }
    }
}

impl Sections {
    fn contains(&self, other: &Sections) -> bool {
        other.lower >= self.lower && other.upper <= self.upper
    }

    fn disjoint_with(&self, other: &Sections) -> bool {
        other.lower > self.upper || other.upper < self.lower
    }
}

fn main() {
    let data = include_str!("../input");
    let paircount = data.lines().count() as u32;
    let ans = data
        .lines()
        .map(|line| {
            let mut ranges = line.split(',');
            let seca = Sections::from(ranges.next().unwrap());
            let secb = Sections::from(ranges.next().unwrap());
            (seca, secb)
        })
        .map(|sec| {
            let contained = (sec.0.contains(&sec.1) || sec.1.contains(&sec.0)) as u32;
            let disjoined = sec.0.disjoint_with(&sec.1) as u32;
            (contained, disjoined)
        })
        .reduce(|a, b| (a.0 + b.0, a.1 + b.1))
        .unwrap();

    println!("Part1: {}", ans.0);
    println!("Part2: {}", paircount - ans.1)
}
