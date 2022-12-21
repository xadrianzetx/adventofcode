use std::collections::HashMap;

enum MathOps {
    Add,
    Sub,
    Mul,
    Div,
}

impl From<&str> for MathOps {
    fn from(op: &str) -> Self {
        match op {
            "+" => Self::Add,
            "-" => Self::Sub,
            "*" => Self::Mul,
            "/" => Self::Div,
            _ => panic!(),
        }
    }
}

struct Monkey {
    name: String,
    number: Option<i64>,
    left: Option<Box<Monkey>>,
    right: Option<Box<Monkey>>,
    op: Option<MathOps>,
}

impl Monkey {
    fn tree_from_lookup(lookup: &HashMap<&str, &str>, name: &str) -> Self {
        let job = lookup.get(&name).unwrap();
        if job.parse::<i64>().is_ok() {
            return Monkey {
                name: String::from(name),
                number: Some(job.parse().unwrap()),
                left: None,
                right: None,
                op: None,
            };
        }

        let split = job.split_whitespace().collect::<Vec<&str>>();
        let left = Monkey::tree_from_lookup(lookup, split[0]);
        let op = MathOps::from(split[1]);
        let right = Monkey::tree_from_lookup(lookup, split[2]);
        Monkey {
            name: String::from(name),
            number: None,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
            op: Some(op),
        }
    }

    fn yell(&self) -> i64 {
        if let Some(num) = self.number {
            return num;
        }

        let l = self.left.as_ref().unwrap().yell();
        let r = self.right.as_ref().unwrap().yell();

        match self.op.as_ref().unwrap() {
            MathOps::Add => l + r,
            MathOps::Sub => l - r,
            MathOps::Mul => l * r,
            MathOps::Div => l / r,
        }
    }

    fn pass_root_check(&self) {
        let l = self.left.as_ref().unwrap().yell();
        let r = self.right.as_ref().unwrap().yell();
        self.left.as_ref().unwrap().adjust_to(r);
        self.right.as_ref().unwrap().adjust_to(l);
    }

    fn adjust_to(&self, result: i64) {
        if self.name == "humn" {
            println!("Part2: {}", result);
        }

        if self.number.is_some() {
            return;
        }

        let l = self.left.as_ref().unwrap().yell();
        let r = self.right.as_ref().unwrap().yell();

        match self.op.as_ref().unwrap() {
            MathOps::Add => {
                self.left.as_ref().unwrap().adjust_to(result - r);
                self.right.as_ref().unwrap().adjust_to(result - l);
            }
            MathOps::Sub => {
                self.left.as_ref().unwrap().adjust_to(r + result);
                self.right.as_ref().unwrap().adjust_to(l - result);
            }
            MathOps::Mul => {
                self.left.as_ref().unwrap().adjust_to(result / r);
                self.right.as_ref().unwrap().adjust_to(result / l);
            }
            MathOps::Div => {
                self.left.as_ref().unwrap().adjust_to(result * r);
                self.right.as_ref().unwrap().adjust_to(result / l);
            }
        };
    }
}

fn main() {
    let mut monkey_lut = HashMap::new();
    include_str!("../input").lines().for_each(|line| {
        let split = line.split(':').collect::<Vec<&str>>();
        monkey_lut.insert(split[0], split[1].trim());
    });

    let monkey_tree = Monkey::tree_from_lookup(&monkey_lut, "root");
    println!("Part1: {}", monkey_tree.yell());
    monkey_tree.pass_root_check();
}
