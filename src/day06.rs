#[derive(Clone, Copy)]
enum Instruction {
    TurnOn,
    TurnOff,
    Toggle,
}
fn part1(input: &str) -> u32 {
    let lines: Vec<_> = input
        .lines()
        .map(|l| {
            let (inst, rest) = if let Some(rest) = l.strip_prefix("turn on ") {
                (Instruction::TurnOn, rest)
            } else if let Some(rest) = l.strip_prefix("turn off ") {
                (Instruction::TurnOff, rest)
            } else if let Some(rest) = l.strip_prefix("toggle ") {
                (Instruction::Toggle, rest)
            } else {
                panic!("line = {:?}", l);
            };

            let (start, rest) = rest.split_once(' ').unwrap();
            let (_, end) = rest.rsplit_once(' ').unwrap();

            let start = start.split_once(',').unwrap();
            let start: (u16, u16) = (start.0.parse().unwrap(), start.1.parse().unwrap());

            let end = end.split_once(',').unwrap();
            let end: (u16, u16) = (end.0.parse().unwrap(), end.1.parse().unwrap());

            (inst, start.0..=end.0, start.1..=end.1)
        })
        .collect();

    let mut sum = 0;
    for y in 0..1000 {
        for x in 0..1000 {
            let mut state = false;
            for (inst, xrange, yrange) in &lines {
                if xrange.contains(&x) && yrange.contains(&y) {
                    match inst {
                        Instruction::TurnOn => state = true,
                        Instruction::TurnOff => state = false,
                        Instruction::Toggle => state = !state,
                    }
                }
            }
            sum += if state { 1 } else { 0 };
        }
    }

    sum
}

fn part2(input: &str) -> u32 {
    let lines: Vec<_> = input
        .lines()
        .map(|l| {
            let (inst, rest) = if let Some(rest) = l.strip_prefix("turn on ") {
                (Instruction::TurnOn, rest)
            } else if let Some(rest) = l.strip_prefix("turn off ") {
                (Instruction::TurnOff, rest)
            } else if let Some(rest) = l.strip_prefix("toggle ") {
                (Instruction::Toggle, rest)
            } else {
                panic!("line = {:?}", l);
            };

            let (start, rest) = rest.split_once(' ').unwrap();
            let (_, end) = rest.rsplit_once(' ').unwrap();

            let start = start.split_once(',').unwrap();
            let start: (u16, u16) = (start.0.parse().unwrap(), start.1.parse().unwrap());

            let end = end.split_once(',').unwrap();
            let end: (u16, u16) = (end.0.parse().unwrap(), end.1.parse().unwrap());

            (inst, start.0..=end.0, start.1..=end.1)
        })
        .collect();

    let mut sum = 0;
    for y in 0..1000 {
        for x in 0..1000 {
            let mut brightness: u32 = 0;
            for (inst, xrange, yrange) in &lines {
                if xrange.contains(&x) && yrange.contains(&y) {
                    match inst {
                        Instruction::TurnOn => brightness += 1,
                        Instruction::TurnOff => brightness = brightness.saturating_sub(1),
                        Instruction::Toggle => brightness += 2,
                    }
                }
            }
            sum += brightness;
        }
    }

    sum
}

fn main() {
    let file = std::fs::read_to_string("./input/day06.txt").unwrap();
    let file = file.trim_end();

    assert_eq!(part1(file), 400410);
    assert_eq!(part2(file), 15343601);
}
