fn inc(s: &mut [u8]) {
    let last = s.last_mut().unwrap();
    match last {
        b'z' => {
            *last = b'a';
            let len = s.len();
            inc(&mut s[..len - 1]);
        }
        _ => {
            *last += 1;
        }
    }
    // for b in s.iter_mut().rev() {
    //     match *b {
    //         b'z' if !carry => {
    //             *b = b'a';
    //             carry = true;
    //         }
    //         b'z' if carry => {
    //             *b = b'a';
    //             carry = true;
    //         }
    //         b'y' if carry => {
    //             *b = b'a';
    //             carry = true;
    //         }
    //         _ => {
    //             *b += 1;
    //         }
    //     }
    // }
}

fn part1(input: &str) -> String {
    let mut input: Vec<u8> = input.into();
    for _ in 0.. {
        inc(&mut input);
        if input.iter().any(|b| matches!(b, b'i' | b'o' | b'l')) {
            continue;
        }

        if !input
            .windows(3)
            .any(|x| x[0] + 1 == x[1] && x[1] + 1 == x[2])
        {
            continue;
        }

        let mut i = 0;
        let mut pairs = 0;
        while i < input.len() - 1 {
            if input[i] == input[i + 1] {
                pairs += 1;
                if pairs == 2 {
                    break;
                }
                i += 2;
            } else {
                i += 1;
            }
        }

        if pairs != 2 {
            continue;
        }

        return unsafe { String::from_utf8_unchecked(input) };
    }

    unreachable!()
}

fn part2(input: &str) -> String {
    let mut input: Vec<u8> = input.into();
    for r in 1..=2 {
        for _ in 0.. {
            inc(&mut input);
            if input.iter().any(|b| matches!(b, b'i' | b'o' | b'l')) {
                continue;
            }

            if !input
                .windows(3)
                .any(|x| x[0] + 1 == x[1] && x[1] + 1 == x[2])
            {
                continue;
            }

            let mut i = 0;
            let mut pairs = 0;
            while i < input.len() - 1 {
                if input[i] == input[i + 1] {
                    pairs += 1;
                    if pairs == 2 {
                        break;
                    }
                    i += 2;
                } else {
                    i += 1;
                }
            }

            if pairs != 2 {
                continue;
            }

            if r == 2 {
                return unsafe { String::from_utf8_unchecked(input) };
            }
            break;
        }
    }

    unreachable!()
}

fn main() {
    let path = if cfg!(debug_assertions) {
        concat!("./input/", module_path!(), "-ex.txt")
    } else {
        concat!("./input/", module_path!(), ".txt")
    };
    let file = std::fs::read_to_string(path).unwrap();
    let file = file.trim_end();

    // assert_eq!(part1(file), "hepxxyzz");
    // assert_eq!(part2(file), "heqaabcc");
    part1(file);
    part2(file);
}
