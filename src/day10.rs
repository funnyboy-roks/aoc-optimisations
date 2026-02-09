fn part1(input: &str) -> u32 {
    let mut next = String::with_capacity(4_000_000);
    let mut current = String::from(input);
    current.reserve(4_000_000);
    for _ in 0..40 {
        let mut rest = &*current;
        while !rest.is_empty() {
            let c = rest.chars().next().unwrap();
            let next_rest = rest.trim_start_matches(c);
            let len = rest.len() - next_rest.len();
            next.push(char::from(len as u8 + b'0'));
            next.push(c);
            rest = next_rest;
        }
        current.clear();
        current.push_str(&next);
        next.clear();
    }

    current.len() as _
}

fn part2(input: &str) -> u32 {
    let mut next = String::with_capacity(4_000_000);
    let mut current = String::from(input);
    current.reserve(4_000_000);
    for _ in 0..50 {
        let mut rest = &*current;
        while !rest.is_empty() {
            let c = rest.chars().next().unwrap();
            let next_rest = rest.trim_start_matches(c);
            let len = rest.len() - next_rest.len();
            next.push(char::from(len as u8 + b'0'));
            next.push(c);
            rest = next_rest;
        }
        current.clear();
        current.push_str(&next);
        next.clear();
    }

    current.len() as _
}

fn main() {
    let path = if cfg!(debug_assertions) {
        concat!("./input/", module_path!(), "-ex.txt")
    } else {
        concat!("./input/", module_path!(), ".txt")
    };
    let file = std::fs::read_to_string(path).unwrap();
    let file = file.trim_end();

    assert_eq!(part1(file), 252594);
    assert_eq!(part2(file), 3579328);
}
