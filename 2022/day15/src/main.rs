use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Beacon(i32, i32);

#[derive(Debug, PartialEq, Eq, Hash)]
struct Sensor {
    x: i32,
    y: i32,
    dist: i32,
}

impl Sensor {
    fn with_beacon(x: i32, y: i32, beacon: &Beacon) -> Self {
        let dist = (x - beacon.0).abs() + (y - beacon.1).abs();
        Sensor { x, y, dist }
    }
}

#[derive(Debug)]
struct Scan {
    sensors: HashSet<Sensor>,
    beacons: HashSet<Beacon>,
}

impl Scan {
    fn coverage(&self, y: i32) -> i32 {
        let mut cnt = 0;
        for x in -2_000_000..=5_000_000 {
            if !self.beacons.contains(&Beacon(x, y)) {
                for sensor in &self.sensors {
                    if (sensor.x - x).abs() + (sensor.y - y).abs() <= sensor.dist {
                        cnt += 1;
                        break;
                    }
                }
            }
        }
        cnt
    }
}

impl From<&str> for Scan {
    fn from(report: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"([-]?\d*)").unwrap();
        }

        let mut sensors = HashSet::new();
        let mut beacons = HashSet::new();
        report.lines().for_each(|line| {
            let split = line.split(':').collect::<Vec<&str>>();
            let sensor = RE
                .find_iter(split[0])
                .filter_map(|digits| digits.as_str().parse().ok())
                .collect::<Vec<i32>>();
            let beacon = RE
                .find_iter(split[1])
                .filter_map(|digits| digits.as_str().parse().ok())
                .collect::<Vec<i32>>();

            let b = Beacon(beacon[0], beacon[1]);
            let s = Sensor::with_beacon(sensor[0], sensor[1], &b);
            beacons.insert(b);
            sensors.insert(s);
        });

        Scan { sensors, beacons }
    }
}

fn main() {
    let data = include_str!("../input");
    let scan = Scan::from(data);
    println!("Part1: {:?}", scan.coverage(2000000));
}
