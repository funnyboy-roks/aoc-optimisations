use std::{collections::HashMap, convert::Infallible, str::FromStr, time::Instant};

#[derive(Debug)]
pub enum Value {
    Wire(String),
    Value(u16),
}

impl Value {
    pub fn resolve(
        &self,
        connections: &HashMap<String, Input>,
        memo: &mut HashMap<String, u16>,
    ) -> u16 {
        match self {
            Value::Wire(w) => {
                if let Some(memo) = memo.get(w) {
                    *memo
                } else {
                    let v = connections[w].resolve(connections, memo);
                    memo.insert(w.clone(), v);
                    v
                }
            }
            Value::Value(v) => *v,
        }
    }
}

impl FromStr for Value {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.parse()
            .map(Self::Value)
            .unwrap_or_else(|_| Value::Wire(s.into())))
    }
}

#[derive(Debug)]
pub enum Input {
    Value(Value),
    And(Value, Value),
    Or(Value, Value),
    LShift(Value, u16),
    RShift(Value, u16),
    Not(Value),
}

impl Input {
    pub fn resolve(
        &self,
        connections: &HashMap<String, Self>,
        memo: &mut HashMap<String, u16>,
    ) -> u16 {
        match self {
            Input::Value(v) => v.resolve(connections, memo),
            Input::And(x, y) => x.resolve(connections, memo) & y.resolve(connections, memo),
            Input::Or(x, y) => x.resolve(connections, memo) | y.resolve(connections, memo),
            Input::LShift(x, y) => x.resolve(connections, memo) << y,
            Input::RShift(x, y) => x.resolve(connections, memo) >> y,
            Input::Not(x) => !x.resolve(connections, memo),
        }
    }
}

fn parse(input: &str) -> HashMap<String, Input> {
    input
        .lines()
        .map(|l| {
            let parts: Vec<_> = l.split_whitespace().collect();
            match &*parts {
                [input, "->", out] => (out.to_string(), Input::Value(input.parse().unwrap())),
                [x, "AND", y, "->", out] => (
                    out.to_string(),
                    Input::And(x.parse().unwrap(), y.parse().unwrap()),
                ),
                [x, "OR", y, "->", out] => (
                    out.to_string(),
                    Input::Or(x.parse().unwrap(), y.parse().unwrap()),
                ),
                [x, "LSHIFT", y, "->", out] => (
                    out.to_string(),
                    Input::LShift(x.parse().unwrap(), y.parse().unwrap()),
                ),
                [x, "RSHIFT", y, "->", out] => (
                    out.to_string(),
                    Input::RShift(x.parse().unwrap(), y.parse().unwrap()),
                ),
                ["NOT", x, "->", out] => (out.to_string(), Input::Not(x.parse().unwrap())),
                _ => panic!("line = {:?}", parts),
            }
        })
        .collect()
}

fn part1(input: &str) -> u16 {
    let connections = parse(input);
    connections["a"].resolve(&connections, &mut HashMap::with_capacity(1000))
}

fn part2(input: &str) -> u16 {
    let mut connections = parse(input);
    let mut memo = HashMap::with_capacity(1000);
    let a = connections["a"].resolve(&connections, &mut memo);
    connections.insert("b".into(), Input::Value(Value::Value(a)));
    memo.clear();
    connections["a"].resolve(&connections, &mut memo)
}

fn main() {
    let file = std::fs::read_to_string("./input/day07.txt").unwrap();
    let file = file.trim_end();

    assert_eq!(part1(file), 16076);
    assert_eq!(part2(file), 2797);
}
