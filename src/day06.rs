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
            let start: (u16, u16) = (parse(start.0), parse(start.1));

            let end = end.split_once(',').unwrap();
            let end: (u16, u16) = (parse(end.0), parse(end.1));

            (inst, start, end)
        })
        .collect();

    let mut sum = 0;
    for y in 0..1000 {
        for x in 0..1000 {
            let mut state = false;
            let mut toggle = false;
            for (inst, start, end) in lines.iter().rev() {
                if start.0 <= x && x <= end.0 && start.1 <= y && y <= end.1 {
                    match inst {
                        Instruction::TurnOn => {
                            state = true;
                            break;
                        }
                        Instruction::TurnOff => {
                            state = false;
                            break;
                        }
                        Instruction::Toggle => toggle = !toggle,
                    }
                }
            }
            if toggle {
                state = !state
            }
            sum += u32::from(state);
        }
    }

    sum
}

fn parse(s: &str) -> u16 {
    let mut out = 0;
    for i in s.bytes().take(3) {
        out *= 10;
        out += u16::from(i - b'0');
    }
    out
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
            let start: (u16, u16) = (parse(start.0), parse(start.1));

            let end = end.split_once(',').unwrap();
            let end: (u16, u16) = (parse(end.0), parse(end.1));

            (inst, start, end)
        })
        .collect();

    let mut sum = 0;
    for y in 0..1000 {
        for x in 0..1000 {
            let mut brightness: u32 = 0;
            for (inst, start, end) in &lines {
                if start.0 <= x && x <= end.0 && start.1 <= y && y <= end.1 {
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
