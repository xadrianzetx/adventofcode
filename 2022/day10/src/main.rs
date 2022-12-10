#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(i64),
}

impl From<&str> for Instruction {
    fn from(line: &str) -> Self {
        let line = line.split_whitespace().collect::<Vec<&str>>().join("");
        let (instruction, data) = line.split_at(4);

        match instruction {
            "noop" => Self::Noop,
            "addx" => Self::Addx(data.parse::<i64>().unwrap()),
            _ => panic!(),
        }
    }
}

trait Probe {
    fn sample(&mut self, tick: u64, register: i64);
}

struct CPUProbe {
    register_state: Vec<i64>,
    sample_points: Vec<u64>,
}

impl CPUProbe {
    fn with_sample_points(sample_points: Vec<u64>) -> Self {
        CPUProbe {
            register_state: vec![],
            sample_points,
        }
    }

    fn measure_signal_strength(&self) -> i64 {
        self.register_state
            .iter()
            .zip(&self.sample_points)
            .map(|pari| pari.0 * *pari.1 as i64)
            .sum()
    }
}

impl Probe for CPUProbe {
    fn sample(&mut self, tick: u64, register: i64) {
        if self.sample_points.contains(&tick) {
            self.register_state.push(register);
        }
    }
}

struct CPU<'a, T> {
    clock: u64,
    regx: i64,
    probes: Option<Vec<&'a mut T>>,
}

impl<'a, T: Probe> CPU<'a, T> {
    fn new() -> Self {
        CPU {
            clock: 0,
            regx: 1,
            probes: None,
        }
    }

    fn add_probe(&mut self, probe: &'a mut T) {
        if let Some(probes) = &mut self.probes {
            probes.push(probe);
        } else {
            self.probes = Some(vec![probe]);
        }
    }

    fn tick(&mut self) {
        self.clock += 1;
        if let Some(probes) = &mut self.probes {
            probes
                .iter_mut()
                .for_each(|p| p.sample(self.clock, self.regx));
        }
    }

    fn execute_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Noop => self.tick(),
            Instruction::Addx(data) => {
                self.tick();
                self.tick();
                self.regx += data;
            }
        }
    }
}

fn main() {
    let mut cpu = CPU::new();
    // (that is, during the 20th, 60th, 100th, 140th, 180th, and 220th cycles)
    let mut cpu_probe = CPUProbe::with_sample_points(vec![20, 60, 100, 140, 180, 220]);
    cpu.add_probe(&mut cpu_probe);
    include_str!("../input").lines().for_each(|line| {
        let instruction = Instruction::from(line);
        cpu.execute_instruction(&instruction);
    });
    println!("{:?}", cpu_probe.measure_signal_strength());
}
