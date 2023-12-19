use std::collections::{HashMap, VecDeque};

type Workflows = HashMap<String, Workflow>;

#[derive(Debug, Clone, PartialEq, Eq)]
enum PartStatus {
    Accepted,
    Rejected,
    Inspect(String),
}

impl From<&str> for PartStatus {
    fn from(value: &str) -> Self {
        match value {
            "A" => Self::Accepted,
            "R" => Self::Rejected,
            _ => Self::Inspect(value.to_string()),
        }
    }
}

#[derive(Debug)]
enum Order {
    Greater,
    Less,
}

impl From<char> for Order {
    fn from(value: char) -> Self {
        match value {
            '>' => Self::Greater,
            '<' => Self::Less,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
enum Category {
    X,
    M,
    A,
    S,
}

impl From<char> for Category {
    fn from(value: char) -> Self {
        match value {
            'x' => Self::X,
            'm' => Self::M,
            'a' => Self::A,
            's' => Self::S,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Rule {
    category: Category,
    order: Order,
    value: usize,
    outcome: PartStatus,
}

impl From<&str> for Rule {
    fn from(value: &str) -> Self {
        let mut s = value.split(':');
        let mut cmp = s.next().unwrap().chars();

        let category = Category::from(cmp.next().unwrap());
        let order = Order::from(cmp.next().unwrap());
        let num = cmp.collect::<String>().parse::<usize>().unwrap();
        let outcome = PartStatus::from(s.next().unwrap());

        Self {
            category,
            order,
            value: num,
            outcome,
        }
    }
}

impl Rule {
    fn matched(&self, part: &mut Part) -> bool {
        match self.category {
            Category::X => match self.order {
                Order::Greater => part.x > self.value,
                Order::Less => part.x < self.value,
            },
            Category::M => match self.order {
                Order::Greater => part.m > self.value,
                Order::Less => part.m < self.value,
            },
            Category::A => match self.order {
                Order::Greater => part.a > self.value,
                Order::Less => part.a < self.value,
            },
            Category::S => match self.order {
                Order::Greater => part.s > self.value,
                Order::Less => part.s < self.value,
            },
        }
    }

    fn probe(&self, probe: &mut Probe) -> Probe {
        let mut new_probe = probe.clone();
        match self.category {
            Category::X => match self.order {
                Order::Greater => {
                    new_probe.x = (self.value + 1, probe.x.1);
                    probe.x = (probe.x.0, self.value);
                }
                Order::Less => {
                    new_probe.x = (probe.x.0, self.value - 1);
                    probe.x = (self.value, probe.x.1);
                }
            },
            Category::M => match self.order {
                Order::Greater => {
                    new_probe.m = (self.value + 1, probe.m.1);
                    probe.m = (probe.m.0, self.value);
                }
                Order::Less => {
                    new_probe.m = (probe.m.0, self.value - 1);
                    probe.m = (self.value, probe.m.1);
                }
            },
            Category::A => match self.order {
                Order::Greater => {
                    new_probe.a = (self.value + 1, probe.a.1);
                    probe.a = (probe.a.0, self.value);
                }
                Order::Less => {
                    new_probe.a = (probe.a.0, self.value - 1);
                    probe.a = (self.value, probe.a.1);
                }
            },
            Category::S => match self.order {
                Order::Greater => {
                    new_probe.s = (self.value + 1, probe.s.1);
                    probe.s = (probe.s.0, self.value);
                }
                Order::Less => {
                    new_probe.s = (probe.s.0, self.value - 1);
                    probe.s = (self.value, probe.s.1);
                }
            },
        };

        new_probe.status = self.outcome.clone();
        new_probe
    }
}

#[derive(Debug)]
struct Workflow {
    rules: Vec<Rule>,
    ends_with: PartStatus,
}

impl Workflow {
    fn new(raw_rules: &str) -> Self {
        let mut r = raw_rules.split(',').collect::<Vec<&str>>();
        let ends_with = PartStatus::from(r.pop().unwrap());

        let mut rules = Vec::new();
        for raw_rule in r {
            rules.push(Rule::from(raw_rule));
        }

        Self { rules, ends_with }
    }

    fn inspect(&self, part: &mut Part) {
        for rule in &self.rules {
            if rule.matched(part) {
                part.status = rule.outcome.clone();
                return;
            }
        }

        part.status = self.ends_with.clone();
    }

    fn probe(&self, mut probe: Probe) -> Vec<Probe> {
        let mut new_probes = Vec::new();
        for rule in &self.rules {
            new_probes.push(rule.probe(&mut probe));
        }

        probe.status = self.ends_with.clone();
        new_probes.push(probe);
        new_probes
    }
}

fn build_workflows(raw_workflows: &str) -> Workflows {
    let mut workflows = HashMap::new();
    for line in raw_workflows.lines() {
        let mut s = line.split('{');
        let name = s.next().unwrap();
        let raw_rules = s.next().unwrap().replace('}', "");
        workflows.insert(name.to_string(), Workflow::new(&raw_rules));
    }
    workflows
}

#[derive(Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
    status: PartStatus,
}

impl From<&str> for Part {
    fn from(value: &str) -> Self {
        let strp = value.replace(['{', '}'], "");
        let mut raw_categories = strp.split(',');

        let x = to_numeric(raw_categories.next().unwrap());
        let m = to_numeric(raw_categories.next().unwrap());
        let a = to_numeric(raw_categories.next().unwrap());
        let s = to_numeric(raw_categories.next().unwrap());

        Self {
            x,
            m,
            a,
            s,
            status: PartStatus::Inspect("in".to_string()),
        }
    }
}

impl Part {
    fn inspect(&mut self, workflows: &Workflows) {
        while let PartStatus::Inspect(step) = &mut self.status {
            workflows.get(step).unwrap().inspect(self);
        }
    }

    fn get_total_rating(&self) -> usize {
        self.x + self.m + self.a + self.s
    }

    fn is_accepted(&self) -> bool {
        self.status == PartStatus::Accepted
    }
}

fn to_numeric(value: &str) -> usize {
    value
        .matches(char::is_numeric)
        .collect::<String>()
        .parse::<usize>()
        .unwrap()
}

fn build_parts(raw_parts: &str) -> Vec<Part> {
    let mut parts = Vec::new();
    for line in raw_parts.lines() {
        parts.push(Part::from(line));
    }
    parts
}

#[derive(Debug, Clone)]
struct Probe {
    x: (usize, usize),
    m: (usize, usize),
    a: (usize, usize),
    s: (usize, usize),
    status: PartStatus,
}

impl Probe {
    fn new() -> Self {
        Self {
            x: (1, 4000),
            m: (1, 4000),
            a: (1, 4000),
            s: (1, 4000),
            status: PartStatus::Inspect("in".to_string()),
        }
    }

    fn count_combinations(&self) -> usize {
        (self.x.1 - self.x.0 + 1)
            * (self.m.1 - self.m.0 + 1)
            * (self.a.1 - self.a.0 + 1)
            * (self.s.1 - self.s.0 + 1)
    }
}

fn probe_combinations(workflows: &Workflows) -> usize {
    let mut probed = Vec::new();
    let initial_probe = Probe::new();
    let mut to_probe = VecDeque::new();
    to_probe.push_back(initial_probe);

    while let Some(probe) = to_probe.pop_front() {
        if let PartStatus::Inspect(workflow) = &probe.status {
            let new_probes = workflows.get(workflow).unwrap().probe(probe);
            for new_probe in new_probes {
                match new_probe.status {
                    PartStatus::Accepted => probed.push(new_probe),
                    PartStatus::Rejected => (),
                    PartStatus::Inspect(_) => to_probe.push_back(new_probe),
                }
            }
        }
    }

    probed.iter().map(|p| p.count_combinations()).sum::<usize>()
}

fn main() {
    let mut workflows_parts = include_str!("../input").split("\n\n");
    let raw_workflows = workflows_parts.next().unwrap();
    let raw_parts = workflows_parts.next().unwrap();

    let workflows = build_workflows(raw_workflows);

    let mut parts = build_parts(raw_parts);
    for part in &mut parts {
        part.inspect(&workflows);
    }

    let part_1 = parts
        .iter()
        .filter(|part| part.is_accepted())
        .map(|part| part.get_total_rating())
        .sum::<usize>();

    println!("Part 1: {part_1}");

    let part_2 = probe_combinations(&workflows);
    println!("Part 2: {part_2}");
}
