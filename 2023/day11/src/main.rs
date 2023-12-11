type Galaxies = Vec<Galaxy>;

#[derive(Debug)]
struct Galaxy(i64, i64);

impl From<(i64, i64)> for Galaxy {
    fn from(value: (i64, i64)) -> Self {
        Galaxy(value.0, value.1)
    }
}

impl Galaxy {
    fn maybe_expand_vertically(&mut self, reference: i64, by: i64) {
        if self.0 > reference {
            self.0 += by;
        }
    }

    fn maybe_expand_horizontally(&mut self, reference: i64, by: i64) {
        if self.1 > reference {
            self.1 += by;
        }
    }
}

fn expand(galaxies: &mut Galaxies, raw_map: &str, factor: i64) {
    let mut expanded_row = 0;
    for line in raw_map.lines() {
        if line.chars().all(|char| char == '.') {
            galaxies
                .iter_mut()
                .for_each(|galaxy| galaxy.maybe_expand_vertically(expanded_row, factor));
            expanded_row += factor;
        }
        expanded_row += 1;
    }

    let unrolled = raw_map
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let line_len = unrolled[0].len();
    let mut expand_col = 0;

    for col in 0..line_len {
        let target = unrolled
            .iter()
            .map(|line| line[col])
            .all(|char| char == '.');
        if target {
            galaxies
                .iter_mut()
                .for_each(|galaxy| galaxy.maybe_expand_horizontally(expand_col, factor));
            expand_col += factor;
        }
        expand_col += 1;
    }
}

fn find_galaxies(raw_map: &str) -> Galaxies {
    let mut map = Vec::new();
    for (row, line) in raw_map.lines().enumerate() {
        for (col, char) in line.chars().enumerate() {
            if char == '#' {
                map.push(Galaxy::from((row as i64, col as i64)));
            }
        }
    }
    map
}

fn get_total_shortest_distance(galaxies: &Galaxies) -> i64 {
    let mut total_distance = 0;
    for i in 0..galaxies.len() {
        let start = &galaxies[i];
        let destinations = &galaxies[i + 1..];
        for dest in destinations {
            total_distance += (start.1 - dest.1).abs() + (start.0 - dest.0).abs();
        }
    }
    total_distance
}

fn main() {
    let raw_map = include_str!("../input");

    let mut expanded_to_one = find_galaxies(raw_map);
    expand(&mut expanded_to_one, raw_map, 2 - 1);
    println!("Part 1: {}", get_total_shortest_distance(&expanded_to_one));

    let mut expanded_to_mil = find_galaxies(raw_map);
    expand(&mut expanded_to_mil, raw_map, 1_000_000 - 1);
    println!("Part 2: {}", get_total_shortest_distance(&expanded_to_mil));
}
