#[derive(Debug, PartialEq)]
enum LockingMechanismType {
    Key,
    Lock,
}

#[derive(Debug)]
struct LockingMechanism {
    mechanism_type: LockingMechanismType,
    pin_heights: Vec<u8>,
}

impl LockingMechanism {
    fn matches(&self, other: &LockingMechanism) -> bool {
        for (pina, pinb) in self.pin_heights.iter().zip(other.pin_heights.iter()) {
            if pina + pinb > 7 {
                return false;
            }
        }
        true
    }
}

fn parse_mechanism(data: &str) -> LockingMechanism {
    let lines = data.lines().collect::<Vec<&str>>();
    let mechanism_type = match lines[0] {
        "#####" => LockingMechanismType::Lock,
        "....." => LockingMechanismType::Key,
        _ => unreachable!(),
    };

    let mut pin_heights = Vec::new();
    for col in 0..5 {
        let mut pin = 0;
        for line in lines.iter() {
            if line.as_bytes()[col] == b'#' {
                pin += 1;
            }
        }
        pin_heights.push(pin);
    }

    LockingMechanism {
        mechanism_type,
        pin_heights,
    }
}

fn main() {
    let keys_and_locks = include_str!("../input")
        .split("\n\n")
        .map(parse_mechanism)
        .collect::<Vec<LockingMechanism>>();

    let mut matches = 0;
    for lock in keys_and_locks
        .iter()
        .filter(|kl| kl.mechanism_type == LockingMechanismType::Key)
    {
        for key in keys_and_locks
            .iter()
            .filter(|kl| kl.mechanism_type == LockingMechanismType::Lock)
        {
            if key.matches(lock) {
                matches += 1;
            }
        }
    }

    println!("Part 1: {matches}");
}
