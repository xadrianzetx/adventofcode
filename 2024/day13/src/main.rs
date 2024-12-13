use mathru::{
    algebra::linear::{
        matrix::{General, Solve},
        vector::Vector,
    },
    matrix, vector,
};

#[derive(Debug)]
struct Machine {
    button_a: (f64, f64),
    button_b: (f64, f64),
    prize: (f64, f64),
}

impl From<&str> for Machine {
    fn from(value: &str) -> Self {
        let lines = value.lines().collect::<Vec<&str>>();

        let bttna = lines[0].split_whitespace().collect::<Vec<&str>>();
        let button_a = (
            bttna[2]
                .strip_prefix("X+")
                .unwrap()
                .strip_suffix(',')
                .unwrap()
                .parse::<f64>()
                .unwrap(),
            bttna[3].strip_prefix("Y+").unwrap().parse::<f64>().unwrap(),
        );

        let bttnb = lines[1].split_whitespace().collect::<Vec<&str>>();
        let button_b = (
            bttnb[2]
                .strip_prefix("X+")
                .unwrap()
                .strip_suffix(',')
                .unwrap()
                .parse::<f64>()
                .unwrap(),
            bttnb[3].strip_prefix("Y+").unwrap().parse::<f64>().unwrap(),
        );

        let prz = lines[2].split_whitespace().collect::<Vec<&str>>();
        let prize = (
            prz[1]
                .strip_prefix("X=")
                .unwrap()
                .strip_suffix(',')
                .unwrap()
                .parse::<f64>()
                .unwrap(),
            prz[2].strip_prefix("Y=").unwrap().parse::<f64>().unwrap(),
        );

        Self {
            button_a,
            button_b,
            prize,
        }
    }
}

fn solve(machine: Machine, modifier: f64) -> Option<isize> {
    let a: General<f64> =
        matrix![machine.button_a.0, machine.button_b.0; machine.button_a.1, machine.button_b.1];
    let b: Vector<f64> = vector![machine.prize.0 + modifier; machine.prize.1 + modifier];
    let solution: Vector<f64> = a.solve(&b).unwrap();

    let press_a = solution[0].round() as isize;
    let press_b = solution[1].round() as isize;

    let a = machine.button_a.0 as isize * press_a + machine.button_b.0 as isize * press_b;
    let b = machine.button_a.1 as isize * press_a + machine.button_b.1 as isize * press_b;

    if a == (machine.prize.0 + modifier) as isize && b == (machine.prize.1 + modifier) as isize {
        return Some(3 * press_a + press_b);
    }
    None
}

fn main() {
    let part_1 = include_str!("../input")
        .split("\n\n")
        .map(Machine::from)
        .filter_map(|m| solve(m, 0.0))
        .sum::<isize>();
    println!("{:?}", part_1);

    let part_2 = include_str!("../input")
        .split("\n\n")
        .map(Machine::from)
        .filter_map(|m| solve(m, 10000000000000.0))
        .sum::<isize>();
    println!("{:?}", part_2);
}
