enum SteeringOps {
    Forward,
    Up,
    Down,
}

impl From<&str> for SteeringOps {
    fn from(opstring: &str) -> Self {
        match opstring {
            "up" => Self::Up,
            "down" => Self::Down,
            "forward" => Self::Forward,
            _ => panic!("should not reach"),
        }
    }
}

struct SteeringInput {
    op: SteeringOps,
    amt: i32,
}

impl From<&str> for SteeringInput {
    fn from(direction: &str) -> Self {
        let data: Vec<&str> = direction.split(' ').collect();
        let op = SteeringOps::from(data[0]);
        let amt = data[1].parse::<i32>().unwrap();
        SteeringInput { op, amt }
    }
}

#[derive(Default)]
struct SubmarinePt1 {
    horizontal: i32,
    depth: i32,
}

#[derive(Default)]
struct SubmarinePt2 {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

trait Controls {
    fn update_position(&mut self, input: &SteeringInput);
    fn get_position(&self) -> i32;
}

impl Controls for SubmarinePt1 {
    fn update_position(&mut self, input: &SteeringInput) {
        match input.op {
            SteeringOps::Forward => self.horizontal += input.amt,
            SteeringOps::Down => self.depth += input.amt,
            SteeringOps::Up => self.depth -= input.amt,
        }
    }

    fn get_position(&self) -> i32 {
        self.depth * self.horizontal
    }
}

impl Controls for SubmarinePt2 {
    fn update_position(&mut self, input: &SteeringInput) {
        match input.op {
            SteeringOps::Forward => {
                self.horizontal += input.amt;
                self.depth += self.aim * input.amt;
            }
            SteeringOps::Down => self.aim += input.amt,
            SteeringOps::Up => self.aim -= input.amt,
        }
    }

    fn get_position(&self) -> i32 {
        self.depth * self.horizontal
    }
}

fn main() {
    let mut pt1 = SubmarinePt1::default();
    let mut pt2 = SubmarinePt2::default();

    include_str!("../d2.txt")
        .lines()
        .map(SteeringInput::from)
        .collect::<Vec<SteeringInput>>()
        .iter()
        .for_each(|e| {
            pt1.update_position(e);
            pt2.update_position(e);
        });

    println!("{}", pt1.get_position());
    println!("{}", pt2.get_position());
}
