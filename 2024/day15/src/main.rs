use std::collections::{HashMap, HashSet, VecDeque};

type Map = HashMap<(i32, i32), char>;

fn make_small_warehouse(data: &str) -> (Map, (i32, i32)) {
    let mut warehouse = HashMap::new();
    let mut robot = (0, 0);

    for (row, line) in data.lines().enumerate() {
        for (col, chr) in line.chars().enumerate() {
            if chr == '@' {
                warehouse.insert((row as i32, col as i32), '.');
                robot = (row as i32, col as i32);
            } else {
                warehouse.insert((row as i32, col as i32), chr);
            }
        }
    }

    (warehouse, robot)
}

fn make_big_warehouse(data: &str) -> (Map, (i32, i32)) {
    let mut warehouse = HashMap::new();
    let mut robot = (0, 0);
    let mut coord = (0, 0);

    for line in data.lines() {
        for col in line.chars() {
            match col {
                '#' => {
                    warehouse.insert((coord.0, coord.1), '#');
                    warehouse.insert((coord.0, coord.1 + 1), '#');
                }
                'O' => {
                    warehouse.insert((coord.0, coord.1), '[');
                    warehouse.insert((coord.0, coord.1 + 1), ']');
                }
                '.' => {
                    warehouse.insert((coord.0, coord.1), '.');
                    warehouse.insert((coord.0, coord.1 + 1), '.');
                }
                '@' => {
                    robot = coord;
                    warehouse.insert((coord.0, coord.1), '.');
                    warehouse.insert((coord.0, coord.1 + 1), '.');
                }
                _ => unreachable!(),
            }

            coord = (coord.0, coord.1 + 2);
        }
        coord = (coord.0 + 1, 0);
    }

    (warehouse, robot)
}

fn move_robot_and_small_boxes(robot: &mut (i32, i32), warehouse: &mut Map, movement: char) {
    let direction = match movement {
        '>' => (0, 1),
        'v' => (1, 0),
        '<' => (0, -1),
        '^' => (-1, 0),
        '\n' => return,
        _ => unreachable!(),
    };

    let mut to_move = Vec::new();
    let mut total_direction = direction;
    while let Some(element) =
        warehouse.get(&(robot.0 + total_direction.0, robot.1 + total_direction.1))
    {
        match element {
            '#' => return,
            '.' => break,
            'O' => {
                to_move.push((robot.0 + total_direction.0, robot.1 + total_direction.1));
            }
            _ => unreachable!(),
        }
        total_direction = (
            total_direction.0 + direction.0,
            total_direction.1 + direction.1,
        );
    }

    *robot = (robot.0 + direction.0, robot.1 + direction.1);
    for box_ in to_move.iter().rev() {
        warehouse.insert(*box_, '.');
        warehouse.insert((box_.0 + direction.0, box_.1 + direction.1), 'O');
    }
}

fn move_robot_and_big_boxes(robot: &mut (i32, i32), warehouse: &mut Map, movement: char) {
    let direction = match movement {
        '>' => (0, 1),
        'v' => (1, 0),
        '<' => (0, -1),
        '^' => (-1, 0),
        '\n' => return,
        _ => unreachable!(),
    };

    match movement {
        '<' | '>' => make_left_right_move(robot, warehouse, direction),
        '^' | 'v' => make_up_down_move(robot, warehouse, direction),
        _ => unreachable!(),
    }
}

fn make_left_right_move(robot: &mut (i32, i32), warehouse: &mut Map, direction: (i32, i32)) {
    let mut to_move = Vec::new();
    let mut total_direction = direction;
    while let Some(element) =
        warehouse.get(&(robot.0 + total_direction.0, robot.1 + total_direction.1))
    {
        match element {
            '#' => return,
            '.' => break,
            '[' | ']' => {
                to_move.push((
                    robot.0 + total_direction.0,
                    robot.1 + total_direction.1,
                    *element,
                ));
            }
            _ => unreachable!(),
        }
        total_direction = (
            total_direction.0 + direction.0,
            total_direction.1 + direction.1,
        );
    }

    *robot = (robot.0 + direction.0, robot.1 + direction.1);
    for box_ in to_move.iter().rev() {
        warehouse.insert((box_.0, box_.1), '.');
        warehouse.insert((box_.0 + direction.0, box_.1 + direction.1), box_.2);
    }
}

fn make_up_down_move(robot: &mut (i32, i32), warehouse: &mut Map, direction: (i32, i32)) {
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();
    queue.push_back((robot.0 + direction.0, robot.1 + direction.1));

    while let Some(pos) = queue.pop_front() {
        match warehouse.get(&pos).unwrap() {
            '#' => return,
            '.' => {}
            '[' => {
                if !seen.contains(&(pos.0, pos.1 + 1)) {
                    queue.push_back((pos.0, pos.1 + 1));
                }
                queue.push_back((pos.0 + direction.0, pos.1 + direction.1));
                seen.insert(pos);
            }
            ']' => {
                if !seen.contains(&(pos.0, pos.1 - 1)) {
                    queue.push_back((pos.0, pos.1 - 1));
                }
                queue.push_back((pos.0 + direction.0, pos.1 + direction.1));
                seen.insert(pos);
            }
            _ => unreachable!(),
        }
    }

    let mut to_move = seen.iter().collect::<Vec<&(i32, i32)>>();
    to_move.sort_by_key(|k| k.0);
    if direction == (1, 0) {
        to_move.reverse();
    }

    for box_part_pos in to_move {
        let box_part = *warehouse.get(box_part_pos).unwrap();
        warehouse.insert(*box_part_pos, '.');
        warehouse.insert(
            (box_part_pos.0 + direction.0, box_part_pos.1 + direction.1),
            box_part,
        );
    }

    *robot = (robot.0 + direction.0, robot.1 + direction.1);
}

fn calc_coordinates(warehouse: &Map, box_: char) -> i32 {
    let mut total = 0;
    for (coord, item) in warehouse {
        if item == &box_ {
            total += coord.0 * 100 + coord.1;
        }
    }
    total
}

fn move_around_small_warehouse(data: &[&str]) -> i32 {
    let (mut warehouse, mut robot) = make_small_warehouse(data[0]);

    for movement in data[1].chars() {
        move_robot_and_small_boxes(&mut robot, &mut warehouse, movement);
    }

    calc_coordinates(&warehouse, 'O')
}

fn move_around_big_warehouse(data: &[&str]) -> i32 {
    let (mut warehouse, mut robot) = make_big_warehouse(data[0]);

    for movement in data[1].chars() {
        move_robot_and_big_boxes(&mut robot, &mut warehouse, movement);
    }

    calc_coordinates(&warehouse, '[')
}

fn main() {
    let data = include_str!("../input")
        .split("\n\n")
        .collect::<Vec<&str>>();

    let part_1 = move_around_small_warehouse(&data);
    println!("{part_1}");

    let part_2 = move_around_big_warehouse(&data);
    println!("{part_2}");
}
