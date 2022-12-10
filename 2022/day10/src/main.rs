enum Instructions {
    Noop,
    Addx(i32),
}

impl From<&str> for Instructions {
    fn from(line: &str) -> Self {
        let line = line.split_whitespace().collect::<Vec<&str>>().join("");
        let (instruction, data) = line.split_at(4);

        match instruction {
            "noop" => Self::Noop,
            "addx" => Self::Addx(data.parse::<i32>().unwrap()),
            _ => panic!(),
        }
    }
}

trait OnClockTick {
    fn sample(&mut self, tick: usize, register: i32);
}

struct CpuProbe {
    register_state: Vec<i32>,
    sampling_points: Vec<usize>,
}

impl CpuProbe {
    fn with_sampling_points(sampling_points: Vec<usize>) -> Self {
        CpuProbe {
            register_state: Vec::new(),
            sampling_points,
        }
    }

    fn total_signal_strength(&self) -> i32 {
        self.register_state
            .iter()
            .zip(&self.sampling_points)
            .map(|pair| pair.0 * *pair.1 as i32)
            .sum()
    }
}

impl OnClockTick for CpuProbe {
    fn sample(&mut self, tick: usize, register: i32) {
        if self.sampling_points.contains(&tick) {
            self.register_state.push(register);
        }
    }
}

struct Crt {
    current_drawn: i32,
    screen_buffer: Vec<char>,
}

impl Crt {
    fn new() -> Self {
        Crt {
            current_drawn: 0,
            screen_buffer: Vec::new(),
        }
    }

    fn render_image(&self) {
        self.screen_buffer
            .chunks(40)
            .into_iter()
            .for_each(|chunk| println!("{}", chunk.iter().collect::<String>()));
    }
}

impl OnClockTick for Crt {
    fn sample(&mut self, _: usize, register: i32) {
        if [register - 1, register, register + 1].contains(&self.current_drawn) {
            // This font is more readable than one suggested by AOC. :^)
            self.screen_buffer.push('█');
        } else {
            self.screen_buffer.push(' ');
        }
        self.current_drawn = (self.current_drawn + 1) % 40;
    }
}

struct Cpu<'a> {
    clock: usize,
    regx: i32,
    devices: Vec<&'a mut dyn OnClockTick>,
}

impl<'a> Cpu<'a> {
    fn new() -> Self {
        Cpu {
            clock: 0,
            regx: 1,
            devices: Vec::new(),
        }
    }

    fn share_clock_with(&mut self, device: &'a mut dyn OnClockTick) {
        self.devices.push(device);
    }

    fn tick(&mut self) {
        self.clock += 1;
        self.devices
            .iter_mut()
            .for_each(|d| d.sample(self.clock, self.regx));
    }

    fn execute_instruction(&mut self, instruction: &Instructions) {
        match instruction {
            Instructions::Noop => self.tick(),
            Instructions::Addx(data) => {
                self.tick();
                self.tick();
                self.regx += data;
            }
        }
    }
}

fn main() {
    let mut cpu = Cpu::new();
    let mut crt = Crt::new();
    let mut cpu_probe = CpuProbe::with_sampling_points(vec![20, 60, 100, 140, 180, 220]);

    cpu.share_clock_with(&mut crt);
    cpu.share_clock_with(&mut cpu_probe);

    include_str!("../input").lines().for_each(|line| {
        let instruction = Instructions::from(line);
        cpu.execute_instruction(&instruction);
    });

    println!("Part1: {}", cpu_probe.total_signal_strength());
    println!("Part2:");
    crt.render_image();
}
