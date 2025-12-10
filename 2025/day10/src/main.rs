use std::collections::HashMap;

#[derive(Debug)]
struct Machine {
    desired_state: Vec<bool>,
    buttons: Vec<Vec<usize>>,
}

impl From<&str> for Machine {
    fn from(value: &str) -> Self {
        let mut desired_state = Vec::new();
        let mut buttons = Vec::new();

        for elem in value.split(' ') {
            if elem.starts_with('[') {
                desired_state = parse_lights(elem);
            } else if elem.starts_with('(') {
                buttons.push(parse_buttons(elem));
            }
        }

        Self {
            desired_state,
            buttons,
        }
    }
}

fn parse_lights(data: &str) -> Vec<bool> {
    let mut lights = Vec::new();
    for chr in data.chars() {
        if chr == '.' {
            lights.push(false);
        } else if chr == '#' {
            lights.push(true);
        }
    }
    lights
}

fn parse_buttons(data: &str) -> Vec<usize> {
    let mut buttons = Vec::new();
    for num in data
        .strip_prefix('(')
        .unwrap()
        .strip_suffix(')')
        .unwrap()
        .split(',')
    {
        buttons.push(num.parse::<usize>().unwrap());
    }
    buttons
}

impl Machine {
    fn find_start_sequence(&self) -> Option<usize> {
        let mut m = HashMap::new();
        let state = vec![false; self.desired_state.len()];
        self.find_start_sequence_iter(state, 0, &mut m)
    }

    fn find_start_sequence_iter(
        &self,
        state: Vec<bool>,
        num_presses: usize,
        m: &mut HashMap<Vec<bool>, usize>,
    ) -> Option<usize> {
        if state == self.desired_state {
            return Some(num_presses);
        }

        if let Some(seen) = m.get(&state)
            && *seen < num_presses
        {
            return None;
        }

        // Advent of cheese.
        if num_presses == 10 {
            return None;
        }

        m.insert(state.clone(), num_presses);

        let mut res = None;
        for buttons in &self.buttons {
            let new_state = push_buttons(&state, buttons);
            if let Some(val) = self.find_start_sequence_iter(new_state, num_presses + 1, m) {
                if let Some(best) = res {
                    if val < best {
                        res = Some(val);
                    }
                } else {
                    res = Some(val);
                }
            }
        }
        res
    }
}

fn push_buttons(state: &[bool], buttons: &[usize]) -> Vec<bool> {
    let mut new_state = state.to_owned();
    for button in buttons {
        new_state[*button] = !new_state[*button];
    }
    new_state
}

fn main() {
    let data = include_str!("input");

    let mut res = 0;
    for line in data.lines() {
        let machine = Machine::from(line);
        let m_tot = machine.find_start_sequence();
        res += m_tot.unwrap();
    }

    println!("Part 1: {res}");
}
