#[derive(Debug, PartialEq)]
enum Colors {
    Red,
    Green,
    Blue,
}

impl From<&str> for Colors {
    fn from(value: &str) -> Self {
        match value {
            "red" => Self::Red,
            "green" => Self::Green,
            "blue" => Self::Blue,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Cubes {
    color: Colors,
    count: u32,
}

impl From<&str> for Cubes {
    fn from(value: &str) -> Self {
        let values = value.trim().split(' ').collect::<Vec<&str>>();
        let count = values.first().unwrap().parse::<u32>().unwrap();
        let color = Colors::from(*values.last().unwrap());
        Cubes { color, count }
    }
}

impl Cubes {
    fn is_possible(&self) -> bool {
        match self.color {
            Colors::Red => self.count <= 12,
            Colors::Green => self.count <= 13,
            Colors::Blue => self.count <= 14,
        }
    }
}

#[derive(Debug)]
struct Set(Vec<Cubes>);

impl From<&str> for Set {
    fn from(value: &str) -> Self {
        Set(value.split(',').map(Cubes::from).collect())
    }
}

impl Set {
    fn is_possible(&self) -> bool {
        self.0.iter().all(|cube| cube.is_possible())
    }

    fn find_count(&self, color: &Colors) -> u32 {
        self.0
            .iter()
            .find(|cube| &cube.color == color)
            .map(|cube| cube.count)
            .unwrap_or(0)
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    draws: Vec<Set>,
}

impl From<&str> for Game {
    fn from(value: &str) -> Self {
        let mut record = value.split(':');
        let id = record
            .next()
            .unwrap()
            .split(' ')
            .last()
            .unwrap()
            .parse::<u32>()
            .unwrap();
        let draws = record
            .next()
            .unwrap()
            .split(';')
            .map(Set::from)
            .collect::<Vec<Set>>();
        Game { id, draws }
    }
}

impl Game {
    fn is_possible(&self) -> bool {
        self.draws.iter().all(|draw| draw.is_possible())
    }

    fn fewest_required(&self, color: Colors) -> u32 {
        self.draws
            .iter()
            .map(|set| set.find_count(&color))
            .max()
            .unwrap()
    }

    fn power(&self) -> u32 {
        self.fewest_required(Colors::Red)
            * self.fewest_required(Colors::Green)
            * self.fewest_required(Colors::Blue)
    }
}

fn main() {
    let games = include_str!("../input")
        .lines()
        .map(Game::from)
        .collect::<Vec<Game>>();

    let part1 = games
        .iter()
        .filter(|game| game.is_possible())
        .map(|game| game.id)
        .sum::<u32>();

    println!("{part1}");

    let part2 = games.iter().map(|game| game.power()).sum::<u32>();
    println!("{part2}")
}
