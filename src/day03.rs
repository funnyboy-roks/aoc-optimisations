use std::collections::HashSet;

fn part1(input: &str) -> u32 {
    let mut s = (0, 0);
    let mut set = HashSet::with_capacity(2500);
    set.insert(s);

    for b in input.bytes() {
        match b {
            b'^' => s.1 -= 1,
            b'v' => s.1 += 1,
            b'<' => s.0 -= 1,
            b'>' => s.0 += 1,
            _ => unreachable!(),
        }
        set.insert(s);
    }

    set.len() as u32
}

fn part2(input: &str) -> u32 {
    let mut s = (0, 0);
    let mut r = (0, 0);
    let mut set = HashSet::with_capacity(2500);
    set.insert(s);

    for (i, b) in input.bytes().enumerate() {
        let t = if i % 2 == 0 { &mut s } else { &mut r };
        match b {
            b'^' => t.1 -= 1,
            b'v' => t.1 += 1,
            b'<' => t.0 -= 1,
            b'>' => t.0 += 1,
            _ => unreachable!(),
        }
        set.insert(*t);
    }

    set.len() as u32
}

fn main() {
    let file = std::fs::read_to_string("./input/day03.txt").unwrap();
    let file = file.trim_end();

    assert_eq!(part1(file), 2081);
    assert_eq!(part2(file), 2341);
}
