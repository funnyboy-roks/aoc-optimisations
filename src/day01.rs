fn part1(input: &str) -> i32 {
    let mut sum = 0i32;
    for b in input.bytes() {
        sum += i32::from(b == b'(') * 2 - 1;
    }

    sum
}

fn part2(input: &str) -> usize {
    let mut sum = 0i32;
    for (i, b) in input.bytes().enumerate() {
        sum += i32::from(b == b'(') * 2 - 1;

        if sum < 0 {
            return i + 1;
        }
    }
    unreachable!();
}

fn main() {
    let file = std::fs::read_to_string("./input/day01.txt").unwrap();
    let file = file.trim_end();

    assert_eq!(part1(file), 232);
    assert_eq!(part2(file), 1783);
}
