use geo::Within;
use geo::{LineString, Polygon};

#[derive(Debug)]
struct Tile {
    x: isize,
    y: isize,
}

impl From<&str> for Tile {
    fn from(value: &str) -> Self {
        let mut coords = value.split(',');
        Self {
            x: coords.next().unwrap().parse::<isize>().unwrap(),
            y: coords.next().unwrap().parse::<isize>().unwrap(),
        }
    }
}

fn find_biggest_rectangle(tiles: &[Tile]) -> isize {
    let mut max_area = 0;
    for (covered, left_tile) in tiles.iter().enumerate() {
        for right_tile in tiles.iter().skip(covered + 1) {
            let area =
                ((right_tile.x - left_tile.x).abs() + 1) * ((right_tile.y - left_tile.y).abs() + 1);
            if area > max_area {
                max_area = area;
            }
        }
    }

    max_area
}

fn find_biggest_contained_rectangle(tiles: &[Tile]) -> isize {
    let line_string: LineString<f32> = tiles
        .iter()
        .map(|t| (t.x as f32, t.y as f32))
        .collect::<Vec<(f32, f32)>>()
        .into();

    let poly = Polygon::new(line_string, vec![]);
    let mut max_area = 0;

    for (covered, left_tile) in tiles.iter().enumerate() {
        for right_tile in tiles.iter().skip(covered + 1) {
            let small_poly = Polygon::new(
                vec![
                    (left_tile.x as f32, left_tile.y as f32),
                    (right_tile.x as f32, left_tile.y as f32),
                    (right_tile.x as f32, right_tile.y as f32),
                    (left_tile.x as f32, right_tile.y as f32),
                ]
                .into(),
                vec![],
            );

            if small_poly.is_within(&poly) {
                let area = ((right_tile.x - left_tile.x).abs() + 1)
                    * ((right_tile.y - left_tile.y).abs() + 1);

                if area > max_area {
                    max_area = area;
                }
            }
        }
    }

    max_area
}

fn main() {
    let data = include_str!("input");

    let tiles = data.lines().map(Tile::from).collect::<Vec<Tile>>();

    let part_1 = find_biggest_rectangle(&tiles);
    println!("Part 1: {part_1}");

    let part_2 = find_biggest_contained_rectangle(&tiles);
    println!("Part 2: {part_2}");
}
