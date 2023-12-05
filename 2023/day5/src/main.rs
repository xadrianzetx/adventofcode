struct Range {
    src: usize,
    dest: usize,
    length: usize,
}

impl From<Vec<usize>> for Range {
    fn from(value: Vec<usize>) -> Self {
        Range {
            src: value[1],
            dest: value[0],
            length: value[2],
        }
    }
}

impl Range {
    fn get_destination(&self, src: usize) -> Option<usize> {
        if self.src <= src && src < self.src + self.length {
            let offset = src - self.src;
            return Some(self.dest + offset);
        }
        None
    }

    fn get_source(&self, dest: usize) -> Option<usize> {
        if self.dest <= dest && dest < self.dest + self.length {
            let offset = dest - self.dest;
            return Some(self.src + offset);
        }
        None
    }
}

struct Map(Vec<Range>);

impl Map {
    fn new() -> Self {
        Map(Vec::new())
    }

    fn push(&mut self, range: Range) {
        self.0.push(range);
    }

    fn find_destination(&self, src: usize) -> usize {
        for rng in &self.0 {
            if let Some(dest) = rng.get_destination(src) {
                return dest;
            }
        }
        src
    }

    fn find_source(&self, dest: usize) -> usize {
        for rng in &self.0 {
            if let Some(src) = rng.get_source(dest) {
                return src;
            }
        }
        dest
    }
}

fn find_lowest_seed(maps: &[Map], seeds: &[usize]) -> usize {
    // Cheesing it at day 5! :^)
    let mut current = 0;
    loop {
        let mut seed = current;
        for m in maps.iter().rev() {
            seed = m.find_source(seed);
        }
        for seedrng in seeds.chunks(2) {
            if seedrng[0] <= seed && seed <= seedrng[1] + seedrng[0] {
                return current;
            }
        }
        current += 1;
    }
}

fn main() {
    let seeds = vec![
        1778931867, 1436999653, 3684516104, 2759374, 1192793053, 358764985, 1698790056, 76369598,
        3733854793, 214008036, 4054174000, 171202266, 3630057255, 25954395, 798587440, 316327323,
        290129780, 7039123, 3334326492, 246125391,
    ];

    // 0: seed-to-soil map
    // 1: soil-to-fertilizer map
    // 2: fertilizer-to-water map
    // 3: water-to-light map
    // 4: light-to-temperature map
    // 5: temperature-to-humidity map
    // 6: humidity-to-location map
    let mut maps = Vec::new();

    include_str!("../input").split("\n\n").for_each(|strmap| {
        let mut map = Map::new();
        strmap.lines().skip(1).for_each(|line| {
            let rng = line
                .split(' ')
                .filter_map(|num| num.parse::<usize>().ok())
                .collect::<Vec<usize>>();
            map.push(Range::from(rng));
        });
        maps.push(map);
    });

    let part1 = seeds
        .iter()
        .map(|seed| {
            let mut curr = *seed;
            for m in &maps {
                curr = m.find_destination(curr);
            }
            curr
        })
        .min()
        .unwrap();
    println!("Part 1: {part1}");
    println!("Part 2: {}", find_lowest_seed(&maps, &seeds));
}
