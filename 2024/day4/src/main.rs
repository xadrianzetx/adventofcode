use std::collections::HashMap;

fn parse_data(data: &str) -> HashMap<(i32, i32), char> {
    let mut parsed = HashMap::new();
    for (rownum, row) in data.lines().enumerate() {
        for (colnum, chr) in row.chars().enumerate() {
            parsed.insert((rownum as i32, colnum as i32), chr);
        }
    }
    parsed
}

fn get_at(row: i32, col: i32, data: &HashMap<(i32, i32), char>) -> char {
    *data.get(&(row, col)).unwrap_or(&char::default())
}

fn find_xmas(data: &HashMap<(i32, i32), char>) -> i32 {
    let mut total = 0;
    for (row, col) in data.keys() {
        let horizontal = [
            get_at(*row, *col, data),
            get_at(row + 1, *col, data),
            get_at(row + 2, *col, data),
            get_at(row + 3, *col, data),
        ]
        .iter()
        .collect::<String>();

        let vertical = [
            get_at(*row, *col, data),
            get_at(*row, col + 1, data),
            get_at(*row, col + 2, data),
            get_at(*row, col + 3, data),
        ]
        .iter()
        .collect::<String>();

        let diag_forward = [
            get_at(*row, *col, data),
            get_at(row + 1, col + 1, data),
            get_at(row + 2, col + 2, data),
            get_at(row + 3, col + 3, data),
        ]
        .iter()
        .collect::<String>();

        let diag_back = [
            get_at(*row, *col, data),
            get_at(row + 1, col - 1, data),
            get_at(row + 2, col - 2, data),
            get_at(row + 3, col - 3, data),
        ]
        .iter()
        .collect::<String>();

        for word in [horizontal, vertical, diag_forward, diag_back] {
            if word == "XMAS" || word == "SAMX" {
                total += 1;
            }
        }
    }
    total
}

fn find_crossed_mas(data: &HashMap<(i32, i32), char>) -> i32 {
    let mut total = 0;
    for (row, col) in data.keys() {
        let diag_forward = [
            get_at(*row, *col, data),
            get_at(row + 1, col + 1, data),
            get_at(row + 2, col + 2, data),
        ]
        .iter()
        .collect::<String>();

        let diag_back = [
            get_at(*row, col + 2, data),
            get_at(row + 1, col + 1, data),
            get_at(row + 2, *col, data),
        ]
        .iter()
        .collect::<String>();

        if (diag_forward == "MAS" || diag_forward == "SAM")
            && (diag_back == "MAS" || diag_back == "SAM")
        {
            total += 1;
        }
    }

    total
}

fn main() {
    let data = parse_data(include_str!("../input"));

    let part_1 = find_xmas(&data);
    println!("Part 1: {part_1}");

    let part_2 = find_crossed_mas(&data);
    println!("Part 2: {part_2}");
}
