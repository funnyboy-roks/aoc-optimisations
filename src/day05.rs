fn part1(input: &str) -> u32 {
    let mut count = 0;
    for l in input.lines() {
        let bytes = l.as_bytes();

        let mut vowels = 0;
        for b in bytes {
            if b"aeiou".contains(b) {
                vowels += 1;
                if vowels == 3 {
                    break;
                }
            }
        }

        let double = bytes.iter().zip(bytes[1..].iter()).any(|(a, b)| a == b);

        let has_bad = l.contains("ab") || l.contains("cd") || l.contains("pq") || l.contains("xy");

        count += u32::from(vowels >= 3) & u32::from(double) & (1 - u32::from(has_bad));
    }

    count
}

fn part2(input: &str) -> u32 {
    let mut count = 0;
    for l in input.lines() {
        let bytes = l.as_bytes();

        let mut cond1 = false;
        'outer: for i in 0..bytes.len() - 3 {
            let cs = [bytes[i], bytes[i + 1]];
            for j in i + 2..bytes.len() - 1 {
                if cs == [bytes[j], bytes[j + 1]] {
                    cond1 = true;
                    break 'outer;
                }
            }
        }

        let cond2 = bytes.iter().zip(bytes[2..].iter()).any(|(a, b)| a == b);

        count += u32::from(cond1) & u32::from(cond2);
    }

    count
}

fn main() {
    let file = std::fs::read_to_string("./input/day05.txt").unwrap();
    let file = file.trim_end();

    assert_eq!(part1(file), 258);
    assert_eq!(part2(file), 53);
}
