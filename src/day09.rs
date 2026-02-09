use std::collections::{HashMap, HashSet};

fn part1(input: &str) -> u32 {
    let mut map = HashMap::<&str, HashMap<&str, u16>>::new();
    for l in input.lines() {
        let mut iter = l.split_whitespace();
        let src = iter.next().unwrap();
        let "to" = iter.next().unwrap() else {
            unreachable!()
        };
        let dst = iter.next().unwrap();
        let "=" = iter.next().unwrap() else {
            unreachable!()
        };
        let weight = iter.next().unwrap();
        let weight: u16 = weight.parse().unwrap();

        map.entry(src).or_default().insert(dst, weight);
        map.entry(dst).or_default().insert(src, weight);
    }

    fn traverse<'a>(
        city: &'a str,
        map: &HashMap<&'a str, HashMap<&'a str, u16>>,
        mut history: HashSet<&'a str>,
        cost: u32,
    ) -> u32 {
        let dests = &map[city];
        history.insert(city);
        dests
            .iter()
            .filter(|(d, _)| !history.contains(**d))
            .map(|(dest, weight)| {
                //
                traverse(dest, map, history.clone(), cost + u32::from(*weight))
            })
            .min()
            .unwrap_or(cost)
    }

    map.keys()
        .map(|city| {
            let cost = traverse(city, &map, Default::default(), 0);
            eprintln!("city: {}, cost: {}", city, cost);
            cost
        })
        .min()
        .unwrap()
}

fn part2(input: &str) -> u32 {
    let mut map = HashMap::<&str, HashMap<&str, u16>>::new();
    for l in input.lines() {
        let mut iter = l.split_whitespace();
        let src = iter.next().unwrap();
        let "to" = iter.next().unwrap() else {
            unreachable!()
        };
        let dst = iter.next().unwrap();
        let "=" = iter.next().unwrap() else {
            unreachable!()
        };
        let weight = iter.next().unwrap();
        let weight: u16 = weight.parse().unwrap();

        map.entry(src).or_default().insert(dst, weight);
        map.entry(dst).or_default().insert(src, weight);
    }

    fn traverse<'a>(
        city: &'a str,
        map: &HashMap<&'a str, HashMap<&'a str, u16>>,
        mut history: HashSet<&'a str>,
        cost: u32,
    ) -> u32 {
        let dests = &map[city];
        history.insert(city);
        dests
            .iter()
            .filter(|(d, _)| !history.contains(**d))
            .map(|(dest, weight)| {
                //
                traverse(dest, map, history.clone(), cost + u32::from(*weight))
            })
            .max()
            .unwrap_or(cost)
    }

    map.keys()
        .map(|city| {
            let cost = traverse(city, &map, Default::default(), 0);
            eprintln!("city: {}, cost: {}", city, cost);
            cost
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

    assert_eq!(part1(file), 117);
    assert_eq!(part2(file), 909);
}
