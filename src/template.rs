fn part1(input: &str) -> u32 {
    todo!("part1");
}

fn part2(_input: &str) -> u32 {
    todo!("part2");
}

fn main() {
    let file = std::fs::read_to_string("./input/day05.txt").unwrap();
    let file = file.trim_end();

    assert_eq!(part1(file), 254575);
    assert_eq!(part2(file), 1038736);
}
