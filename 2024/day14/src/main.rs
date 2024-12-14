use itertools::Itertools;
use std::collections::HashSet;

#[derive(Debug)]
struct Robot {
    p: (i32, i32),
    v: (i32, i32),
}

impl From<&str> for Robot {
    fn from(value: &str) -> Self {
        let pv = value.split_whitespace().collect::<Vec<&str>>();
        let p = pv[0]
            .strip_prefix("p=")
            .unwrap()
            .split(',')
            .map(|v| v.parse::<i32>().unwrap())
            .collect_tuple()
            .unwrap();
        let v = pv[1]
            .strip_prefix("v=")
            .unwrap()
            .split(',')
            .map(|v| v.parse::<i32>().unwrap())
            .collect_tuple()
            .unwrap();

        Robot { p, v }
    }
}

impl Robot {
    fn update_position(&mut self, width: i32, height: i32) {
        let dx = (self.p.0 + self.v.0) % width;
        if dx < 0 {
            self.p.0 = width - dx.abs();
        } else {
            self.p.0 = dx;
        }

        let dy = (self.p.1 + self.v.1) % height;
        if dy < 0 {
            self.p.1 = height - dy.abs();
        } else {
            self.p.1 = dy;
        }
    }

    fn get_quadrant(&self, width: i32, height: i32) -> Option<usize> {
        let vertical_midpoint = (width - 1) / 2;
        let horizontal_midpoint = (height - 1) / 2;

        if self.p.0 < vertical_midpoint && self.p.1 < horizontal_midpoint {
            return Some(1);
        } else if self.p.0 > vertical_midpoint && self.p.1 < horizontal_midpoint {
            return Some(2);
        } else if self.p.0 < vertical_midpoint && self.p.1 > horizontal_midpoint {
            return Some(3);
        } else if self.p.0 > vertical_midpoint && self.p.1 > horizontal_midpoint {
            return Some(4);
        }
        None
    }
}

fn render_robots(robots: &Vec<Robot>, width: i32, height: i32) {
    let mut positions = HashSet::new();
    for robot in robots {
        positions.insert(robot.p);
    }

    for py in 0..height {
        for px in 0..width {
            if positions.contains(&(px, py)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!()
    }
}

fn main() {
    let data = include_str!("../input");

    let mut robots = data.lines().map(Robot::from).collect::<Vec<Robot>>();

    // 10_403 iterations to cycle back to the initial state.
    for second in 1..10500 {
        for robot in &mut robots {
            robot.update_position(101, 103);
        }

        if second == 100 {
            let part_1 = robots
                .iter()
                .filter_map(|r| r.get_quadrant(101, 103))
                .counts()
                .into_values()
                .reduce(|l, r| l * r)
                .unwrap();
            println!("Part 1: {}", part_1);
        }

        // if (second - 28 + 1) % 101 == 0 || (second - 84 + 1) % 103 == 0 {
        //     // There are patterns forming every 101 and 103 seconds (for my input) with initial
        //     // offsets of 28 and 84. Eventually one of them forms the tree.
        //     std::process::Command::new("clear").status().unwrap();
        //     println!("Second {}", second + 1);
        //     render_robots(&robots, 101, 103);
        //     std::thread::sleep(std::time::Duration::from_millis(2000));
        // }

        if second == 7603 {
            // See above how this magic number was found.
            println!("Part 2: {}", second + 1);
            render_robots(&robots, 101, 103);
            break;
        }
    }
}
