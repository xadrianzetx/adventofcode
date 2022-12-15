use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::{max, min};
use std::collections::HashSet;

struct Beacon(i64, i64);

#[derive(PartialEq, Eq, Hash)]
struct Sensor {
    x: i64,
    y: i64,
    dist: i64,
}

impl Sensor {
    fn with_beacon(x: i64, y: i64, beacon: &Beacon) -> Self {
        let dist = (x - beacon.0).abs() + (y - beacon.1).abs();
        Sensor { x, y, dist }
    }
}

struct Scan {
    sensors: HashSet<Sensor>,
}

impl Scan {
    fn coverage(&self, row: i64) -> i64 {
        let mut covers = Vec::new();
        for sensor in &self.sensors {
            for dx in 0..sensor.dist {
                let dy = sensor.dist - dx;
                if (sensor.y + dy) == row || (sensor.y - dy) == row {
                    covers.push((sensor.x - dx, sensor.x + dx));
                    break;
                }
            }
        }

        let rng = covers
            .into_iter()
            .reduce(|a, b| (min(a.0, b.0), max(a.1, b.1)))
            .unwrap();
        rng.1 - rng.0
    }

    fn check_outside_bounds(&self) -> i64 {
        let bound = 4000000;
        for sensor in &self.sensors {
            for dx in 0..=sensor.dist + 1 {
                let dy = sensor.dist + 1 - dx;
                for comb in [(1, 1), (-1, 1), (-1, -1), (1, -1)] {
                    let x = dx * comb.0;
                    let y = dy * comb.1;
                    let mut this = true;
                    for other in &self.sensors {
                        if (other.x - (sensor.x + x)).abs() + (other.y - (sensor.y + y)).abs()
                            <= other.dist
                        {
                            this = false;
                            break;
                        }
                    }
                    if this
                        && sensor.x + x <= bound
                        && sensor.x + x >= 0
                        && sensor.y + y <= bound
                        && sensor.y + y >= 0
                    {
                        return ((sensor.x + x) * 4000000) + sensor.y + y;
                    }
                }
            }
        }
        -1
    }
}

impl From<&str> for Scan {
    fn from(report: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"([-]?\d*)").unwrap();
        }

        let mut sensors = HashSet::new();
        report.lines().for_each(|line| {
            let split = line.split(':').collect::<Vec<&str>>();
            let sensor = RE
                .find_iter(split[0])
                .filter_map(|digits| digits.as_str().parse().ok())
                .collect::<Vec<i64>>();
            let beacon = RE
                .find_iter(split[1])
                .filter_map(|digits| digits.as_str().parse().ok())
                .collect::<Vec<i64>>();

            let s = Sensor::with_beacon(sensor[0], sensor[1], &Beacon(beacon[0], beacon[1]));
            sensors.insert(s);
        });

        Scan { sensors }
    }
}

fn main() {
    let data = include_str!("../input");
    let scan = Scan::from(data);
    println!("Part1: {}", scan.coverage(2000000));
    println!("Part2: {}", scan.check_outside_bounds());
}
