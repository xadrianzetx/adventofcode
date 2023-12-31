use std::collections::HashMap;

trait Rated {
    fn rate(&self, part: &Part) -> Status;
    fn count_accepted(&self, part_range: PartRange) -> usize;
}

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

impl From<&Category> for usize {
    fn from(value: &Category) -> Self {
        match value {
            Category::X => 0,
            Category::M => 1,
            Category::A => 2,
            Category::S => 3,
        }
    }
}

enum Condition {
    Greater,
    Less,
}

impl From<char> for Condition {
    fn from(value: char) -> Self {
        match value {
            '>' => Self::Greater,
            '<' => Self::Less,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy)]
enum Status {
    Accepted,
    Rejected,
}

impl Rated for Status {
    fn rate(&self, _: &Part) -> Status {
        *self
    }

    fn count_accepted(&self, part_range: PartRange) -> usize {
        match self {
            Self::Rejected => 0,
            Self::Accepted => part_range.0.iter().map(|rng| rng.1 - rng.0 + 1).product(),
        }
    }
}

impl Status {
    fn is_accepted(&self) -> bool {
        match self {
            Self::Accepted => true,
            Self::Rejected => false,
        }
    }
}

struct Rule {
    category: Category,
    condition: Condition,
    value: usize,
    on_pass: Box<dyn Rated>,
    on_fail: Box<dyn Rated>,
}

impl Rule {
    fn new_with_outcomes(raw_rule: &str, on_pass: Box<dyn Rated>, on_fail: Box<dyn Rated>) -> Self {
        let mut chars = raw_rule.chars();
        let category = Category::from(chars.next().unwrap());
        let condition = Condition::from(chars.next().unwrap());
        let value = chars.collect::<String>().parse::<usize>().unwrap();

        Self {
            category,
            condition,
            value,
            on_pass,
            on_fail,
        }
    }
}

impl Rated for Rule {
    fn rate(&self, part: &Part) -> Status {
        let passes = match self.condition {
            Condition::Greater => part.get_value(&self.category) > self.value,
            Condition::Less => part.get_value(&self.category) < self.value,
        };

        if passes {
            return self.on_pass.rate(part);
        }
        self.on_fail.rate(part)
    }

    fn count_accepted(&self, part_range: PartRange) -> usize {
        let mut passing = part_range.clone();
        let mut failing = part_range.clone();
        let rng = part_range.get_range(&self.category);

        match self.condition {
            Condition::Greater => {
                passing.set_range(self.value + 1, rng.1, &self.category);
                failing.set_range(rng.0, self.value, &self.category);
            }
            Condition::Less => {
                passing.set_range(rng.0, self.value - 1, &self.category);
                failing.set_range(self.value, rng.1, &self.category);
            }
        };

        self.on_pass.count_accepted(passing) + self.on_fail.count_accepted(failing)
    }
}

fn build_workflows_lookup(raw_workflows: &str) -> HashMap<String, String> {
    let mut workflows_lut = HashMap::new();
    for line in raw_workflows.lines() {
        let workflow_and_rules = line.split('{').collect::<Vec<&str>>();

        let workflow_name = workflow_and_rules[0];
        let raw_rules = workflow_and_rules[1];

        for (idx, rule) in raw_rules[..raw_rules.len() - 1].split(',').enumerate() {
            let flat_worfklow_name = format!("{workflow_name}{idx}");
            if rule.contains('>') || rule.contains('<') {
                let next = idx + 1;
                if rule.ends_with('A') || rule.ends_with('R') {
                    workflows_lut
                        .insert(flat_worfklow_name, format!("{rule}:{workflow_name}{next}"));
                } else {
                    workflows_lut
                        .insert(flat_worfklow_name, format!("{rule}0:{workflow_name}{next}"));
                }
            } else if rule.ends_with('A') || rule.ends_with('R') {
                workflows_lut.insert(flat_worfklow_name, rule.to_string());
            } else {
                workflows_lut.insert(flat_worfklow_name, format!("{rule}0"));
            }
        }
    }

    workflows_lut
}

fn add_rule(name: &str, workflows_lut: &HashMap<String, String>) -> Box<dyn Rated> {
    match name {
        "A" => Box::new(Status::Accepted),
        "R" => Box::new(Status::Rejected),
        _ => {
            let entry = workflows_lut.get(name).unwrap();
            let parts = entry.split(':').collect::<Vec<&str>>();

            if parts.len() == 1 {
                return add_rule(parts[0], workflows_lut);
            }

            let raw_rule = parts[0];
            let on_pass = add_rule(parts[1], workflows_lut);
            let on_fail = add_rule(parts[2], workflows_lut);

            Box::new(Rule::new_with_outcomes(raw_rule, on_pass, on_fail))
        }
    }
}

fn build_workflow_tree(workflows_lut: &HashMap<String, String>) -> Box<dyn Rated> {
    add_rule("in0", workflows_lut)
}

// The order is x, m, a, s of course!
struct Part(Vec<usize>);

impl From<&str> for Part {
    fn from(value: &str) -> Self {
        let raw_numbers = value.replace(['{', '}', 'x', 'm', 'a', 's', '='], "");
        let numbers = raw_numbers
            .split(',')
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        Self(numbers)
    }
}

impl Part {
    fn sum_rating_numbers(&self) -> usize {
        self.0.iter().sum()
    }

    fn get_value(&self, category: &Category) -> usize {
        let idx: usize = category.into();
        *self.0.get(idx).unwrap()
    }
}

#[derive(Clone)]
struct PartRange(Vec<(usize, usize)>);

impl PartRange {
    fn new_with_range(low: usize, high: usize) -> Self {
        Self(vec![(low, high); 4])
    }

    fn get_range(&self, category: &Category) -> &(usize, usize) {
        let idx: usize = category.into();
        self.0.get(idx).unwrap()
    }

    fn set_range(&mut self, low: usize, high: usize, category: &Category) {
        let idx: usize = category.into();
        self.0[idx] = (low, high);
    }
}

fn main() {
    let workflows_parts = include_str!("../input")
        .split("\n\n")
        .collect::<Vec<&str>>();

    let workflows_lut = build_workflows_lookup(workflows_parts[0]);
    let workflows = build_workflow_tree(&workflows_lut);

    let part_1 = workflows_parts[1]
        .lines()
        .map(Part::from)
        .filter(|p| workflows.rate(p).is_accepted())
        .map(|p| p.sum_rating_numbers())
        .sum::<usize>();
    println!("{part_1}");

    let part_range = PartRange::new_with_range(1, 4000);
    let part_2 = workflows.count_accepted(part_range);
    println!("{part_2}");
}
