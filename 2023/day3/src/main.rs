use std::collections::HashMap;

#[derive(Debug)]
struct Number {
    number: usize,
    line: i32,
    starts: i32,
    ends: i32,
}

fn find_numbers(map: &str) -> Vec<Number> {
    let mut numbers = Vec::new();
    for (lino, line) in map.lines().enumerate() {
        let mut buff = String::new();
        let mut start = 0;
        let lenght = line.len();
        for (pos, chr) in line.chars().enumerate() {
            if chr.is_numeric() {
                if buff.is_empty() {
                    start = pos as i32;
                }
                buff.push(chr);
            }

            if (!chr.is_numeric() || pos + 1 == lenght) && !buff.is_empty() {
                let number = buff.parse::<usize>().unwrap();
                numbers.push(Number {
                    number,
                    line: lino as i32,
                    starts: start,
                    ends: (pos - 1) as i32,
                });
                buff.clear();
            }
        }
    }
    numbers
}

fn find_symbols(map: &str) -> HashMap<(i32, i32), &str> {
    let mut symbols = HashMap::new();
    for (lino, line) in map.lines().enumerate() {
        let indices = line.match_indices(|chr: char| !chr.is_numeric() && chr != '.');
        for (idx, sym) in indices {
            symbols.insert((lino as i32, idx as i32), sym);
        }
    }
    symbols
}

fn find_part_numbers(numbers: &[Number], symbols: &HashMap<(i32, i32), &str>) -> usize {
    let mut tot = 0;
    numbers.iter().for_each(|number| {
        for i in number.starts - 1..=number.ends + 1 {
            for j in number.line - 1..=number.line + 1 {
                if symbols.contains_key(&(j, i)) {
                    tot += number.number;
                }
            }
        }
    });
    tot
}

#[derive(Debug)]
struct Gears(usize, usize);

fn find_gear_ratios(numbers: &[Number], symbols: &HashMap<(i32, i32), &str>) -> usize {
    let mut gears: HashMap<(i32, i32), Gears> = HashMap::new();
    numbers.iter().for_each(|number| {
        for i in number.starts - 1..=number.ends + 1 {
            for j in number.line - 1..=number.line + 1 {
                if let Some(&"*") = symbols.get(&(j, i)) {
                    gears
                        .entry((j, i))
                        .and_modify(|entry| {
                            entry.0 += 1;
                            entry.1 *= number.number;
                        })
                        .or_insert(Gears(1, number.number));
                }
            }
        }
    });

    gears
        .values()
        .filter(|gear| gear.0 == 2)
        .map(|gear| gear.1)
        .sum()
}

fn main() {
    let map = include_str!("../input");
    let numbers = find_numbers(map);
    let symbols = find_symbols(map);

    let part_1 = find_part_numbers(&numbers, &symbols);
    println!("{part_1}");

    let part2 = find_gear_ratios(&numbers, &symbols);
    println!("{part2}");
}
