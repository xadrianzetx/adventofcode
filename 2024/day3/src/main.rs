use regex::Regex;

struct Interpreter {
    total: i32,
    conditionals_enabled: bool,
    instructions_enabled: bool,
}

impl Interpreter {
    fn with_conditionals_enabled() -> Self {
        Self {
            total: 0,
            conditionals_enabled: true,
            instructions_enabled: true,
        }
    }

    fn with_conditionals_disabled() -> Self {
        Self {
            total: 0,
            conditionals_enabled: false,
            instructions_enabled: true,
        }
    }

    fn do_(&mut self) {
        if self.conditionals_enabled {
            self.instructions_enabled = true;
        }
    }

    fn dont(&mut self) {
        if self.conditionals_enabled {
            self.instructions_enabled = false;
        }
    }

    fn add(&mut self, left: i32, right: i32) {
        if self.instructions_enabled {
            self.total += left * right;
        }
    }
}

fn run_instructions(program: &str, iterpreter: &mut Interpreter) {
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)|(do\(\))|(don't\(\))").unwrap();
    for elem in re.captures_iter(program) {
        match (elem.get(1), elem.get(2), elem.get(3), elem.get(4)) {
            (Some(l), Some(r), _, _) => {
                let left = l.as_str().parse::<i32>().unwrap();
                let right = r.as_str().parse::<i32>().unwrap();
                iterpreter.add(left, right);
            }
            (_, _, Some(_), _) => iterpreter.do_(),
            (_, _, _, Some(_)) => iterpreter.dont(),
            _ => unreachable!(),
        }
    }
}

fn main() {
    let program = include_str!("../input");

    let mut part_1 = Interpreter::with_conditionals_disabled();
    run_instructions(program, &mut part_1);
    println!("Part 1: {}", part_1.total);

    let mut part_2 = Interpreter::with_conditionals_enabled();
    run_instructions(program, &mut part_2);
    println!("Part 1: {}", part_2.total);
}
