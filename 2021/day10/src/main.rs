use std::collections::HashMap;

#[derive(Default)]
struct Syntax {
    line: String,
    score: usize,
    penalty: usize,
}

impl From<&str> for Syntax {
    fn from(line: &str) -> Self {
        let line = String::from(line);
        Syntax {
            line,
            ..Default::default()
        }
    }
}

impl Syntax {
    fn check(&mut self, brackets: &HashMap<char, char>) {
        let mut stack: Vec<char> = Vec::new();
        for c in self.line.chars() {
            if ['(', '[', '{', '<'].contains(&c) {
                stack.push(c);
            } else {
                let opening = stack.pop().unwrap();
                let expected_closing = brackets.get(&opening).unwrap();
                if expected_closing != &c {
                    self.penalty = get_penalty(&c);
                    break;
                }
            }
        }

        if self.penalty == 0 {
            stack.reverse();
            for c in stack {
                self.score *= 5;
                self.score += get_reward(&c);
            }
        }
    }
}

fn get_penalty(c: &char) -> usize {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("Should not reach"),
    }
}

fn get_reward(c: &char) -> usize {
    match c {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => panic!("Should not reach"),
    }
}

fn main() {
    let brackets: HashMap<char, char> = [('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]
        .iter()
        .cloned()
        .collect();

    let mut penalties = 0;
    let mut scores: Vec<usize> = Vec::new();

    include_str!("../d10.txt")
        .lines()
        .map(Syntax::from)
        .collect::<Vec<Syntax>>()
        .into_iter()
        .for_each(|mut e| {
            e.check(&brackets);
            penalties += e.penalty;
            if e.score > 0 {
                scores.push(e.score);
            }
        });

    // part 1
    println!("{:?}", penalties);

    // part 2
    scores.sort_unstable();
    let midpoint = scores.len() / 2;
    println!("{:?}", scores[midpoint]);
}
