use std::collections::HashMap;

struct Springs(HashMap<(usize, usize, usize), usize>);

impl Springs {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn count_arrangements(&mut self, config: &str, groups: &[usize]) -> usize {
        self.arrange(config, groups, 0, 0, 0)
    }

    fn arrange(
        &mut self,
        config: &str,
        groups: &[usize],
        cidx: usize,
        gidx: usize,
        group_len: usize,
    ) -> usize {
        if let Some(partial) = self.0.get(&(cidx, gidx, group_len)) {
            return *partial;
        }

        if cidx == config.len() {
            if gidx == groups.len() - 1 && (group_len == groups[gidx] || group_len == 0) {
                return 1;
            }
            return 0;
        }

        if gidx > groups.len() - 1 || groups[gidx] < group_len {
            return 0;
        }

        let current_char = config.chars().nth(cidx).unwrap();
        let mut partial = 0;

        if (current_char == '.' || current_char == '?')
            && (group_len == 0 || group_len == groups[gidx])
        {
            partial += self.arrange(config, groups, cidx + 1, gidx, 0);
        }

        if current_char == '#' || current_char == '?' {
            let mut new_gidx = gidx;
            if group_len == 0 {
                new_gidx += 1;
            }
            partial += self.arrange(config, groups, cidx + 1, new_gidx, group_len + 1);
        }

        // State representation inspired by jonathanpaulson.
        // https://github.com/jonathanpaulson/AdventOfCode/blob/5ac1fc19f9eaa84e43ccad5e13778cd3a1919a92/2023/12.py#L16
        self.0.insert((cidx, gidx, group_len), partial);
        partial
    }
}

fn parse_springs(raw_spring: &str, repeat: usize) -> (String, Vec<usize>) {
    let mut records = raw_spring.split(' ');
    let config = vec![records.next().unwrap(); repeat].join("?");

    let mut groups = records
        .next()
        .unwrap()
        .split(',')
        .filter_map(|num| num.parse().ok())
        .collect::<Vec<usize>>();

    groups = groups.repeat(repeat);
    groups.insert(0, 0);

    (config, groups)
}

fn main() {
    let raw_springs = include_str!("../input").lines().collect::<Vec<&str>>();

    let part1 = raw_springs
        .iter()
        .map(|raw_spring| {
            let (config, groups) = parse_springs(raw_spring, 1);
            let mut springs = Springs::new();
            springs.count_arrangements(&config, &groups)
        })
        .sum::<usize>();
    println!("Part 1: {part1}");

    let part2 = raw_springs
        .iter()
        .map(|raw_spring| {
            let (config, groups) = parse_springs(raw_spring, 5);
            let mut springs = Springs::new();
            springs.count_arrangements(&config, &groups)
        })
        .sum::<usize>();
    println!("Part 2: {part2}");
}
