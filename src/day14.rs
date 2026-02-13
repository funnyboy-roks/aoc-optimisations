fn part1(input: &str) -> usize {
    const DURATION: usize = if cfg!(debug_assertions) { 1000 } else { 2503 };
    input
        .lines()
        .map(|line| {
            let mut whitespace = line.split_whitespace();
            let _name = whitespace.next().unwrap();
            let speed: usize = whitespace.nth(2).unwrap().parse().unwrap();
            let duration: usize = whitespace.nth(2).unwrap().parse().unwrap();
            let rest: usize = whitespace.nth_back(1).unwrap().parse().unwrap();

            let mut seconds = 0;
            let mut distance = 0;
            while seconds < DURATION {
                let seconds_left = DURATION - seconds;
                let time = duration.min(seconds_left);
                distance += speed * time;
                seconds += time;
                if seconds < DURATION {
                    let seconds_left = DURATION - seconds;
                    seconds += rest.min(seconds_left);
                }
            }
            distance
        })
        .max()
        .unwrap()
}

fn part2(input: &str) -> usize {
    const DURATION: usize = if cfg!(debug_assertions) { 1000 } else { 2503 };
    let distances: Vec<(usize, usize, usize)> = input
        .lines()
        .map(|line| {
            let mut whitespace = line.split_whitespace();
            let _name = whitespace.next().unwrap();
            let speed: usize = whitespace.nth(2).unwrap().parse().unwrap();
            let duration: usize = whitespace.nth(2).unwrap().parse().unwrap();
            let rest: usize = whitespace.nth_back(1).unwrap().parse().unwrap();
            (speed, duration, rest)
        })
        .collect();

    #[derive(Default, Debug)]
    struct Reindeer {
        time_left: usize,
        moving: bool,
        distance: usize,
    }

    let mut state: Vec<Reindeer> = distances.iter().map(|_| Default::default()).collect();

    let mut count: Vec<usize> = distances.iter().map(|_| 0).collect();

    for _ in 0..DURATION {
        for (&(speed, duration, rest), reindeer) in distances.iter().zip(state.iter_mut()) {
            if reindeer.time_left == 0 {
                reindeer.moving = !reindeer.moving;
                reindeer.time_left = if reindeer.moving { duration } else { rest };
            }
            if reindeer.moving {
                reindeer.distance += speed;
            }
            reindeer.time_left -= 1;
        }
        let max = state.iter().map(|r| r.distance).max().unwrap();
        state
            .iter()
            .map(|r| r.distance)
            .enumerate()
            .filter(|(_, d)| *d == max)
            .for_each(|(i, _)| count[i] += 1);
    }
    *count.iter().max().unwrap()
}

fn main() {
    let path = if cfg!(debug_assertions) {
        concat!("./input/", module_path!(), "-ex.txt")
    } else {
        concat!("./input/", module_path!(), ".txt")
    };
    let file = std::fs::read_to_string(path).unwrap();
    let file = file.trim_end();

    if cfg!(debug_assertions) {
        println!("\x1b[;32mPART 1: \x1b[;33m{}\x1b[0m", part1(file));
        println!("\x1b[;32mPART 2: \x1b[;33m{}\x1b[0m", part2(file));
    } else {
        assert_eq!(part1(file), 2660, "Part 1");
        assert_eq!(part2(file), 1256, "Part 2");
    }
}
