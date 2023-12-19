use std::collections::HashMap;

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
        let num = cmp
            .into_iter()
            .collect::<String>()
            .parse::<usize>()
            .unwrap();
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
        let matched = match self.category {
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
        };

        if matched {
            part.status = self.outcome.clone();
            return true;
        }
        false
    }
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
    ends_with: PartStatus,
}

impl Workflow {
    fn new(name: &str, raw_rules: &str) -> Self {
        let mut r = raw_rules.split(',').into_iter().collect::<Vec<&str>>();
        let ends_with = PartStatus::from(r.pop().unwrap());

        let mut rules = Vec::new();
        for raw_rule in r {
            rules.push(Rule::from(raw_rule));
        }

        Self {
            name: name.to_string(),
            rules,
            ends_with,
        }
    }

    fn inspect(&self, part: &mut Part) {
        let mut matched = false;
        for rule in &self.rules {
            if rule.matched(part) {
                matched = true;
                break;
            }
        }

        if !matched {
            part.status = self.ends_with.clone();
        }
    }
}

fn build_workflows(raw_workflows: &str) -> Workflows {
    let mut workflows = HashMap::new();
    for line in raw_workflows.lines() {
        let mut s = line.split('{');
        let name = s.next().unwrap();
        let raw_rules = s.next().unwrap().replace('}', "");
        workflows.insert(name.to_string(), Workflow::new(name, &raw_rules));
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
        let x = raw_categories
            .next()
            .unwrap()
            .matches(char::is_numeric)
            .collect::<String>()
            .parse::<usize>()
            .unwrap();
        let m = raw_categories
            .next()
            .unwrap()
            .matches(char::is_numeric)
            .collect::<String>()
            .parse::<usize>()
            .unwrap();
        let a = raw_categories
            .next()
            .unwrap()
            .matches(char::is_numeric)
            .collect::<String>()
            .parse::<usize>()
            .unwrap();
        let s = raw_categories
            .next()
            .unwrap()
            .matches(char::is_numeric)
            .collect::<String>()
            .parse::<usize>()
            .unwrap();

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

fn build_parts(raw_parts: &str) -> Vec<Part> {
    let mut parts = Vec::new();
    for line in raw_parts.lines() {
        parts.push(Part::from(line));
    }
    parts
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

    println!("{:?}", part_1);
}
