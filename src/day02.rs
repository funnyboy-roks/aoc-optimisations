fn parse(s: &str) -> u32 {
    let mut out = 0;
    for b in s.bytes() {
        out *= 10;
        out += (b - b'0') as u32;
    }
    out
}

fn part1(input: &str) -> u32 {
    let mut sum = 0;
    for l in input.lines() {
        let (l, rest) = l.split_once('x').unwrap();
        let (w, h) = rest.split_once('x').unwrap();

        let l: u32 = parse(l);
        let w: u32 = parse(w);
        let h: u32 = parse(h);

        let smallest = std::cmp::min(std::cmp::min(l * w, w * h), h * l);

        sum += 2 * l * w + 2 * w * h + 2 * h * l + smallest;
    }

    sum
}

fn part2(input: &str) -> u32 {
    let mut sum = 0;
    for l in input.lines() {
        let (l, rest) = l.split_once('x').unwrap();
        let (w, h) = rest.split_once('x').unwrap();

        let l: u32 = parse(l);
        let w: u32 = parse(w);
        let h: u32 = parse(h);

        let smallest = std::cmp::min(std::cmp::min(2 * l + 2 * w, 2 * w + 2 * h), 2 * h + 2 * l);

        sum += smallest + l * w * h;
    }
    sum
}

fn main() {
    let file = std::fs::read_to_string("./input/day02.txt").unwrap();
    let file = file.trim_end();

    assert_eq!(part1(file), 1598415);
    assert_eq!(part2(file), 3812909);
}
