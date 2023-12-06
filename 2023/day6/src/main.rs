struct Race {
    time: usize,
    distance: usize,
}

impl From<&(usize, usize)> for Race {
    fn from(value: &(usize, usize)) -> Self {
        Race {
            time: value.0,
            distance: value.1,
        }
    }
}

impl Race {
    fn count_ways_to_win(&self) -> usize {
        let mut count = 0;
        for hold in 0..=self.time {
            if hold * (self.time - hold) > self.distance {
                count += 1;
            }
        }
        count
    }
}

fn main() {
    let races = vec![(42_usize, 284_usize), (68, 1005), (69, 1122), (85, 1341)]
        .iter()
        .map(Race::from)
        .collect::<Vec<Race>>();

    let part1 = races
        .iter()
        .map(|race| race.count_ways_to_win())
        .product::<usize>();
    println!("Part 1: {part1}");

    let race = Race::from(&(42686985_usize, 284100511221341_usize));
    println!("Part 2: {}", race.count_ways_to_win());
}
