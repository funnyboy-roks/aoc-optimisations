fn part1(input: &str) -> u32 {
    let mut sum = 0;
    for code in input.as_bytes().split(|b| *b == b'\n') {
        sum += 2;

        let mut i = 1;
        while i < code.len() - 1 {
            if code[i] == b'\\' {
                debug_assert!(i < code.len() - 1);
                match code[i + 1] {
                    b'x' => {
                        sum += 3;
                        i += 3;
                    }
                    b'\\' | b'"' => {
                        sum += 1;
                        i += 1;
                    }
                    c => panic!("{:?}", char::from(c)),
                };
            }
            i += 1;
        }
    }
    sum as _
}

fn part2(input: &str) -> u32 {
    let mut sum = 2;
    for b in input.bytes() {
        match b {
            b'"' | b'\\' => sum += 1,
            b'\n' => sum += 2,
            _ => (),
        }
    }
    sum as _
}

fn main() {
    let file = std::fs::read_to_string(concat!("./input/", module_path!(), ".txt")).unwrap();
    let file = file.trim_end();

    assert_eq!(part1(file), 1333);
    assert_eq!(part2(file), 2046);
}
