use std::collections::HashMap;

#[derive(Debug)]
pub enum Value {
    Wire(&'static str),
    Value(u16),
}

impl Value {
    pub fn resolve(
        &self,
        connections: &HashMap<&'static str, Input>,
        memo: &mut HashMap<&'static str, u16>,
    ) -> u16 {
        match self {
            Value::Wire(w) => {
                if let Some(memo) = memo.get(w) {
                    *memo
                } else {
                    let v = connections[w].resolve(connections, memo);
                    memo.insert(w, v);
                    v
                }
            }
            Value::Value(v) => *v,
        }
    }

    fn from_str(s: &'static str) -> Self {
        s.parse()
            .map(Self::Value)
            .unwrap_or_else(|_| Value::Wire(s))
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
        connections: &HashMap<&'static str, Self>,
        memo: &mut HashMap<&'static str, u16>,
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

fn parse(input: &'static str) -> HashMap<&'static str, Input> {
    input
        .lines()
        .map(|l| {
            let parts: Vec<_> = l.split_whitespace().collect();
            match parts[..] {
                [input, "->", out] => (out, Input::Value(Value::from_str(input))),
                [x, "AND", y, "->", out] => {
                    (out, Input::And(Value::from_str(x), Value::from_str(y)))
                }
                [x, "OR", y, "->", out] => (out, Input::Or(Value::from_str(x), Value::from_str(y))),
                [x, "LSHIFT", y, "->", out] => {
                    (out, Input::LShift(Value::from_str(x), y.parse().unwrap()))
                }
                [x, "RSHIFT", y, "->", out] => {
                    (out, Input::RShift(Value::from_str(x), y.parse().unwrap()))
                }
                ["NOT", x, "->", out] => (out, Input::Not(Value::from_str(x))),
                _ => panic!("line = {:?}", parts),
            }
        })
        .collect()
}

fn part1(input: &'static str) -> u16 {
    let connections = parse(input);
    connections["a"].resolve(&connections, &mut HashMap::with_capacity(1000))
}

fn part2(input: &'static str) -> u16 {
    let mut connections = parse(input);
    let mut memo = HashMap::with_capacity(1000);
    let a = connections["a"].resolve(&connections, &mut memo);
    connections.insert("b", Input::Value(Value::Value(a)));
    memo.clear();
    connections["a"].resolve(&connections, &mut memo)
}

fn main() {
    let file: &'static str = std::fs::read_to_string("./input/day07.txt")
        .unwrap()
        .leak()
        .trim_end();

    assert_eq!(part1(file), 16076);
    assert_eq!(part2(file), 2797);
}
