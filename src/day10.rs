fn part1(input: &str) -> u32 {
    let mut next = Vec::with_capacity(4_000_000);
    let mut current = input.as_bytes().to_vec();
    current.reserve(4_000_000);
    for _ in 0..40 {
        let bytes = &*current;
        let mut run = 1;
        let mut cb = bytes[0];
        for &b in &bytes[1..] {
            if b == cb {
                run += 1;
            } else {
                next.push(run + b'0');
                next.push(cb);
                cb = b;
                run = 1;
            }
        }
        next.push(run + b'0');
        next.push(cb);

        current.clear();
        current.extend(&next);
        next.clear();
    }

    current.len() as _
}

fn part2(input: &str) -> u32 {
    let mut next = Vec::with_capacity(4_000_000);
    let mut current = input.as_bytes().to_vec();
    current.reserve(4_000_000);
    for _ in 0..50 {
        let bytes = &*current;
        let mut run = 1;
        let mut cb = bytes[0];
        for &b in &bytes[1..] {
            if b == cb {
                run += 1;
            } else {
                next.push(run + b'0');
                next.push(cb);
                cb = b;
                run = 1;
            }
        }
        next.push(run + b'0');
        next.push(cb);

        current.clear();
        current.extend(&next);
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
