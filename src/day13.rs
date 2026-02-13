use std::collections::HashMap;

fn permute<'a>(names: &[&'a str]) -> Vec<Vec<&'a str>> {
    if names.len() <= 1 {
        return vec![names.to_vec()];
    }
    let mut out = Vec::<Vec<&str>>::new();
    for (i, name) in names.iter().enumerate() {
        let mut names = names.to_vec();
        names.swap_remove(i);
        let p = permute(&names);
        out.reserve(p.len());
        for mut p in p.into_iter() {
            p.push(name);
            out.push(p);
        }
    }
    out
}

fn part1(input: &str) -> i32 {
    let mut happiness = HashMap::<_, HashMap<_, _>>::new();
    for line in input.lines() {
        let line = line.trim_end_matches('.');
        let mut whitespace = line.split_whitespace();
        let source = whitespace.next().unwrap();
        whitespace.next();
        let loss_gain = whitespace.next().unwrap();
        let qty = whitespace.next().unwrap().parse::<i32>().unwrap()
            * if loss_gain == "lose" { -1 } else { 1 };
        let target = whitespace.next_back().unwrap();
        happiness.entry(source).or_default().insert(target, qty);
    }
    dbg!(&happiness);

    permute(&happiness.keys().copied().collect::<Vec<_>>())
        .into_iter()
        .map(|names| {
            let mut sum = 0;
            for (a, b) in names
                .iter()
                .copied()
                .zip(names[1..].iter().copied().chain(std::iter::once(names[0])))
            {
                sum += happiness[a][b];
                sum += happiness[b][a];
            }
            sum
        })
        .max()
        .unwrap()
}

fn part2(input: &str) -> i32 {
    let mut happiness = HashMap::<_, HashMap<_, _>>::new();
    for line in input.lines() {
        let line = line.trim_end_matches('.');
        let mut whitespace = line.split_whitespace();
        let source = whitespace.next().unwrap();
        whitespace.next();
        let loss_gain = whitespace.next().unwrap();
        let qty = whitespace.next().unwrap().parse::<i32>().unwrap()
            * if loss_gain == "lose" { -1 } else { 1 };
        let target = whitespace.next_back().unwrap();
        happiness.entry(source).or_default().insert(target, qty);
    }
    dbg!(&happiness);

    permute(&happiness.keys().copied().collect::<Vec<_>>())
        .into_iter()
        .map(|names| {
            let mut sum = 0;
            for (a, b) in names.iter().copied().zip(names[1..].iter()) {
                sum += happiness[a][b];
                sum += happiness[b][a];
            }
            sum
        })
        .max()
        .unwrap()
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
        dbg!(part1(file));
        dbg!(part2(file));
    } else {
        assert_eq!(part1(file), 664);
        assert_eq!(part2(file), 640);
    }
}
