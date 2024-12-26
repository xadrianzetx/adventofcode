use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
enum Gate {
    And,
    Or,
    Xor,
}

impl From<&str> for Gate {
    fn from(value: &str) -> Self {
        match value {
            "AND" => Self::And,
            "OR" => Self::Or,
            "XOR" => Self::Xor,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Connection {
    inp0: String,
    inp1: String,
    out: String,
    gate: Gate,
}

impl From<&str> for Connection {
    fn from(value: &str) -> Self {
        let values = value.split_whitespace().collect::<Vec<&str>>();
        let gate = Gate::from(values[1]);

        Self {
            inp0: values[0].to_string(),
            inp1: values[2].to_string(),
            out: values[4].to_string(),
            gate,
        }
    }
}

fn init_state(data: &str) -> HashMap<String, u8> {
    let mut circuit = HashMap::new();
    for values in data.lines() {
        let v = values.split_whitespace().collect::<Vec<&str>>();
        let gate = v[0].strip_suffix(':').unwrap().to_string();
        let value = v[1].parse::<u8>().unwrap();
        circuit.insert(gate, value);
    }

    circuit
}

fn run(state: &mut HashMap<String, u8>, connections: Vec<Connection>) {
    let mut queue = VecDeque::from(connections);
    while let Some(c) = queue.pop_front() {
        if state.contains_key(&c.inp0) && state.contains_key(&c.inp1) {
            let inp0 = state.get(&c.inp0).unwrap();
            let inp1 = state.get(&c.inp1).unwrap();

            let res = match c.gate {
                Gate::And => inp0 & inp1,
                Gate::Or => inp0 | inp1,
                Gate::Xor => inp0 ^ inp1,
            };

            state.insert(c.out, res);
        } else {
            queue.push_back(c);
        }
    }
}

fn to_usize(state: &HashMap<String, u8>, wire: char) -> usize {
    let mut outgates = state
        .iter()
        .filter(|(k, _)| k.starts_with(wire))
        .collect::<Vec<(&String, &u8)>>();

    outgates.sort_by_key(|elem| elem.0);
    let mut out: usize = 0;
    for (shift, gate) in outgates.iter().enumerate() {
        out |= (*gate.1 as usize) << shift;
    }
    out
}

fn run_unaltered_circuit(data: &[&str]) {
    let mut state = init_state(data[0]);
    let connections = data[1]
        .lines()
        .map(Connection::from)
        .collect::<Vec<Connection>>();

    run(&mut state, connections);
    let part_1 = to_usize(&state, 'z');
    println!("Part 1: {part_1}");
}

fn alter_connections(connections: &mut [Connection]) {
    // Drawing out the actual circuit was tedious, but made discovering swapped wires really easy. :^)
    for c in connections {
        match (c.inp0.as_str(), c.inp1.as_str(), &c.gate) {
            // pair 1 - dqd XOR dnn -> ffj; dnn AND dqd -> z08; should be dqd XOR dnn -> z08; dnn AND dqd -> ffj
            ("dqd", "dnn", Gate::Xor) => c.out = "z08".to_string(),
            ("dnn", "dqd", Gate::And) => c.out = "ffj".to_string(),
            // pair 2 - x15 AND y15 -> dwp; y15 XOR x15 -> kfm; should be x15 AND y15 -> kfm; x15 XOR y15 -> dwp
            ("x15", "y15", Gate::And) => c.out = "kfm".to_string(),
            ("y15", "x15", Gate::Xor) => c.out = "dwp".to_string(),
            // pair 3 - pgt XOR hgq -> gjh; y22 AND x22 -> z22; should be pgt XOR hgq -> z22; y22 AND x22 -> gjh
            ("pgt", "hgq", Gate::Xor) => c.out = "z22".to_string(),
            ("y22", "x22", Gate::And) => c.out = "gjh".to_string(),
            // pair 4 - rns XOR hnn -> jdr; ctt OR vhw -> z31; should be rns XOR hnn -> z31; ctt OR vhw -> jdr
            ("rns", "hnn", Gate::Xor) => c.out = "z31".to_string(),
            ("ctt", "vhw", Gate::Or) => c.out = "jdr".to_string(),
            _ => (),
        }
    }
    println!("Part 2: dwp,ffj,gjh,jdr,kfm,z08,z22,z31");
}

fn run_altered_circuit(data: &[&str]) {
    let mut state = init_state(data[0]);
    let mut connections = data[1]
        .lines()
        .map(Connection::from)
        .collect::<Vec<Connection>>();

    let lhs = to_usize(&state, 'x');
    let rhs = to_usize(&state, 'y');

    alter_connections(&mut connections);
    run(&mut state, connections);

    assert_eq!(lhs + rhs, to_usize(&state, 'z'));
}

fn main() {
    let data = include_str!("../input")
        .split("\n\n")
        .collect::<Vec<&str>>();

    run_unaltered_circuit(&data);
    run_altered_circuit(&data);
}
