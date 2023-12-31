use geo::{Area, LineString, Polygon};

// Up, right, down, left.
const DIRECTIONS: &[(f64, f64)] = &[(-1., 0.), (0., 1.), (1., 0.), (0., -1.)];

fn get_directions(raw_directions: &str) -> usize {
    match raw_directions {
        "U" => 0,
        "R" => 1,
        "D" => 2,
        "L" => 3,
        _ => unreachable!(),
    }
}

fn rotate_directions(raw_directions: usize) -> usize {
    (raw_directions + 1) % 4
}

fn parse_small_plan(raw_plan: &str) -> Vec<(usize, f64)> {
    raw_plan
        .lines()
        .map(|line| {
            let mut plan = line.split(' ');
            let direction = get_directions(plan.next().unwrap());
            let distance = plan.next().unwrap().parse::<f64>().unwrap();
            (direction, distance)
        })
        .collect::<Vec<(usize, f64)>>()
}

fn parse_big_plan(raw_plan: &str) -> Vec<(usize, f64)> {
    raw_plan
        .lines()
        .map(|line| {
            let plan = line.split(' ');
            let mut hex = plan.last().unwrap().replace(['(', ')', '#'], "");
            let direction = rotate_directions(hex.pop().unwrap().to_digit(10).unwrap() as usize);
            let distance = i64::from_str_radix(&hex, 16).unwrap() as f64;
            (direction, distance)
        })
        .collect::<Vec<(usize, f64)>>()
}

fn calculate_lagoon_area(vertices: Vec<(f64, f64)>) -> usize {
    let polygon = Polygon::new(LineString::from(vertices), vec![]);
    polygon.unsigned_area() as usize
}

fn calculate_lagoon_capacity(plan: Vec<(usize, f64)>) -> usize {
    let mut buff = Vec::new();
    let mut curr = (0., 0.);
    let mut perimeter = 0;

    for (direction, distance) in plan {
        perimeter += distance as usize;
        buff.push((curr.0, curr.1));

        curr = (
            curr.0 + (DIRECTIONS[direction].0 * distance),
            curr.1 + (DIRECTIONS[direction].1 * distance),
        );
    }

    calculate_lagoon_area(buff) + (perimeter / 2) + 1
}

fn main() {
    let raw_plan = include_str!("../input");

    println!(
        "Part 1: {}",
        calculate_lagoon_capacity(parse_small_plan(raw_plan))
    );

    println!(
        "Part 2: {}",
        calculate_lagoon_capacity(parse_big_plan(raw_plan))
    );
}
