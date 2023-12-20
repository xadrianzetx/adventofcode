use std::{
    collections::{HashMap, VecDeque},
    fmt::Debug,
};

type Devices = HashMap<String, Box<dyn Device>>;

#[derive(Debug)]
struct Pulse {
    sender: String,
    receiver: String,
    high: bool,
}

impl Pulse {
    fn new(sender: String, receiver: String, high: bool) -> Self {
        Self {
            sender,
            receiver,
            high,
        }
    }
}

trait Device: Debug {
    fn recv(&mut self, pulse: &Pulse) -> Option<Vec<Pulse>>;
    fn declare_as_source(&self, _: &mut HashMap<String, Vec<String>>) {}
    fn set_sources(&mut self, _: &HashMap<String, Vec<String>>) {}
}

#[derive(Debug)]
struct FlipFlop {
    name: String,
    high: bool,
    destinations: Vec<String>,
}

impl From<&str> for FlipFlop {
    fn from(value: &str) -> Self {
        let tmp = value.replace('%', "");
        let mut name_dest = tmp.split(" -> ");
        let name = name_dest.next().unwrap();
        let dest = name_dest
            .next()
            .unwrap()
            .split(',')
            .map(|d| d.trim().to_string())
            .collect::<Vec<String>>();
        Self {
            name: name.to_string(),
            high: false,
            destinations: dest,
        }
    }
}

impl Device for FlipFlop {
    fn recv(&mut self, pulse: &Pulse) -> Option<Vec<Pulse>> {
        if !pulse.high {
            self.high = !self.high;
            let mut pulses = Vec::new();
            for dest in &self.destinations {
                pulses.push(Pulse::new(
                    self.name.to_string(),
                    dest.to_string(),
                    self.high,
                ));
            }
            return Some(pulses);
        }
        None
    }

    fn declare_as_source(&self, sources: &mut HashMap<String, Vec<String>>) {
        for dest in &self.destinations {
            sources
                .entry(dest.to_string())
                .and_modify(|e| e.push(self.name.clone()))
                .or_insert(vec![self.name.clone()]);
        }
    }
}

#[derive(Debug)]
struct Conjunction {
    name: String,
    memory: HashMap<String, bool>,
    destinations: Vec<String>,
}

impl From<&str> for Conjunction {
    fn from(value: &str) -> Self {
        let tmp = value.replace('&', "");
        let mut name_dest = tmp.split(" -> ");
        let name = name_dest.next().unwrap();
        let dest = name_dest
            .next()
            .unwrap()
            .split(',')
            .map(|d| d.trim().to_string())
            .collect::<Vec<String>>();
        let memory = HashMap::new();
        Self {
            name: name.to_string(),
            memory,
            destinations: dest,
        }
    }
}

impl Device for Conjunction {
    fn recv(&mut self, pulse: &Pulse) -> Option<Vec<Pulse>> {
        if let Some(high) = self.memory.get_mut(&pulse.sender) {
            *high = pulse.high;
        }

        let pulse = !self.memory.values().all(|v| *v);
        let mut pulses = Vec::new();
        for dest in &self.destinations {
            pulses.push(Pulse::new(self.name.to_string(), dest.to_string(), pulse));
        }
        Some(pulses)
    }

    fn declare_as_source(&self, sources: &mut HashMap<String, Vec<String>>) {
        for dest in &self.destinations {
            sources
                .entry(dest.to_string())
                .and_modify(|e| e.push(self.name.clone()))
                .or_insert(vec![self.name.clone()]);
        }
    }

    fn set_sources(&mut self, sources: &HashMap<String, Vec<String>>) {
        if let Some(srcv) = sources.get(&self.name) {
            for src in srcv {
                self.memory.insert(src.to_string(), false);
            }
        }
    }
}

#[derive(Debug)]
struct Broadcaster {
    name: String,
    destinations: Vec<String>,
}

impl From<&str> for Broadcaster {
    fn from(value: &str) -> Self {
        let mut name_dest = value.split(" -> ");
        let name = name_dest.next().unwrap();
        let dest = name_dest
            .next()
            .unwrap()
            .split(',')
            .map(|d| d.trim().to_string())
            .collect::<Vec<String>>();
        Self {
            name: name.to_string(),
            destinations: dest,
        }
    }
}

impl Device for Broadcaster {
    fn recv(&mut self, pulse: &Pulse) -> Option<Vec<Pulse>> {
        let mut pulses = Vec::new();
        for dest in &self.destinations {
            pulses.push(Pulse::new(
                self.name.to_string(),
                dest.to_string(),
                pulse.high,
            ));
        }
        Some(pulses)
    }
}

fn build_circuit(raw_circuit: &str) -> Devices {
    let mut csources: HashMap<String, Vec<String>> = HashMap::new();
    let mut devices: Devices = HashMap::new();
    for line in raw_circuit.lines() {
        if line.starts_with('%') {
            let flip_flop = FlipFlop::from(line);
            devices.insert(flip_flop.name.clone(), Box::new(flip_flop));
        } else if line.starts_with('&') {
            let conjunction = Conjunction::from(line);
            devices.insert(conjunction.name.clone(), Box::new(conjunction));
        } else if line.starts_with('b') {
            let broadcaster = Broadcaster::from(line);
            devices.insert(broadcaster.name.clone(), Box::new(broadcaster));
        } else {
        }
    }

    for device in devices.values() {
        device.as_ref().declare_as_source(&mut csources);
    }

    for device in devices.values_mut() {
        device.as_mut().set_sources(&csources);
    }
    devices
}

#[derive(Debug, Default)]
struct PulseCounter {
    high: usize,
    low: usize,
}

impl PulseCounter {
    fn count_pulse(&mut self, pulse: &Pulse) {
        if pulse.high {
            self.high += 1;
        } else {
            self.low += 1;
        }
    }

    fn summary(&self) -> usize {
        self.high * self.low
    }
}

#[derive(Debug)]
struct CycleCouter {
    counter: usize,
    cycle_lengths: HashMap<String, usize>,
}

impl CycleCouter {
    fn new() -> Self {
        let cycle_lengths = HashMap::new();
        CycleCouter {
            counter: 1,
            cycle_lengths,
        }
    }

    fn increment(&mut self) {
        self.counter += 1;
    }

    fn register(&mut self, pulse: &Pulse) {
        // Search for cycles in tj's gates.
        // tj -> rx
        // tj: {"kk": 0, "sk": 0, "vt": 0, "xc": 0}
        if pulse.receiver == "tj" && pulse.high {
            self.cycle_lengths
                .insert(pulse.sender.clone(), self.counter);
        }
    }

    fn is_full(&self) -> bool {
        self.cycle_lengths.len() == 4
    }

    fn get_cycle_length(&self) -> usize {
        let lengths = self.cycle_lengths.values().copied().collect::<Vec<usize>>();
        lcm(&lengths)
    }
}

// Again, too lazy to write these, so credit where credit's due.
// https://github.com/TheAlgorithms/Rust/blob/7d2aa9e8be79cd23c36aa99cbfa66b520b132035/src/math/lcm_of_n_numbers.rs
fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd(a, b)
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn push_the_button(
    devices: &mut Devices,
    pulse_counter: &mut PulseCounter,
    cycle_counter: &mut CycleCouter,
) {
    let mut bus = VecDeque::new();
    bus.push_back(Pulse::new(
        "button".to_string(),
        "broadcaster".to_string(),
        false,
    ));

    while let Some(pulse) = bus.pop_front() {
        pulse_counter.count_pulse(&pulse);
        cycle_counter.register(&pulse);

        if let Some(device) = devices.get_mut(&pulse.receiver) {
            if let Some(new_pulses) = device.as_mut().recv(&pulse) {
                for p in new_pulses {
                    bus.push_back(p);
                }
            }
        }
    }
}

fn mash_the_button(devices: &mut Devices) {
    let mut pulse_counter = PulseCounter::default();
    let mut cycle_counter = CycleCouter::new();

    let mut step = 1;
    loop {
        push_the_button(devices, &mut pulse_counter, &mut cycle_counter);
        if step == 1000 {
            println!("Part 1: {}", pulse_counter.summary());
        }

        if cycle_counter.is_full() {
            println!("Part 2: {}", cycle_counter.get_cycle_length());
            break;
        }

        cycle_counter.increment();
        step += 1;
    }
}

fn main() {
    let raw_circuit = include_str!("../input");
    let mut devices = build_circuit(raw_circuit);
    mash_the_button(&mut devices);
}
