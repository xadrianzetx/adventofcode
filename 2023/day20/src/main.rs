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
        Self { sender, receiver, high }
    }
}

trait Device: Debug {
    fn recv(&mut self, pulse: &Pulse) -> Option<Vec<Pulse>>;
    fn declare_as_source(&self, _: &mut HashMap<String, Vec<String>>) {}
    fn set_sources(&mut self, _: &HashMap<String, Vec<String>>) {}
    fn dbg(&self);
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
                pulses.push(Pulse::new(self.name.to_string(), dest.to_string(), self.high));
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

    fn dbg(&self) {
        print!("{}: {}, ", self.name, self.high as usize);
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

    fn dbg(&self) {
        let mut dbgmem = HashMap::new();
        for (k, v) in &self.memory {
            dbgmem.insert(k, *v as usize);
        }
        print!("{}: {:?}, ", self.name, dbgmem);
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
            pulses.push(Pulse::new(self.name.to_string(), dest.to_string(), pulse.high));
        }
        Some(pulses)
    }

    fn dbg(&self) {
        
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
    low: usize
}

impl PulseCounter {
    fn count_pulse(&mut self, pulse: &Pulse) {
        if pulse.high {
            self.high += 1;
        } else {
            self.low += 1;
        }
    }

    fn add(&mut self, other: &PulseCounter) {
        self.high += other.high;
        self.low += other.low;
    }

    fn summary(&self) -> usize {
        self.high * self.low
    }
}

fn push_the_button(devices: &mut Devices) -> PulseCounter {
    let mut pulse_counter = PulseCounter::default();
    let mut bus = VecDeque::new();
    bus.push_back(Pulse::new("button".to_string(), "broadcaster".to_string(), false));

    while let Some(pulse) = bus.pop_front() {
        pulse_counter.count_pulse(&pulse);
        if let Some(device) = devices.get_mut(&pulse.receiver) {
            if let Some(new_pulses) = device.as_mut().recv(&pulse) {
                for p in new_pulses {
                    bus.push_back(p);
                }
            }
        }
    }

    pulse_counter
}

fn mash_the_button(devices: &mut Devices) -> PulseCounter {
    let mut pulse_counter = PulseCounter::default();
    for _ in 0..1000 {
        pulse_counter.add(&push_the_button(devices));
        // println!("after {}", i + 1);
        // for d in devices.values() {
        //     d.as_ref().dbg();
        // }
        // println!();
        // println!("#############");
    }
    pulse_counter
}

fn main() {
    let raw_circuit = include_str!("../input");
    // println!("{raw_circuit}");
    let mut devices = build_circuit(raw_circuit);
    // println!("default");
    // println!("{devices:?}");
    // for d in devices.values() {
    //     d.as_ref().dbg();
    // }
    // println!();
    // println!("#########");
    println!("{}", mash_the_button(&mut devices).summary());
}
