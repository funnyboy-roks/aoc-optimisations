use md5::{
    Digest, Md5,
    digest::{consts::U16, generic_array::GenericArray},
};

fn write10(mut n: u32, buf: &mut [u8]) -> usize {
    let mut i = 0;
    while n > 0 {
        buf[i] = (n % 10) as u8 + b'0';
        n /= 10;
        i += 1;
    }
    buf[..i].reverse();
    i
}

fn part1(input: &str) -> u32 {
    // create a Md5 hasher instance
    let mut hasher = Md5::new();

    let mut hash = GenericArray::<u8, U16>::default();
    let mut buf = [0u8; 10];
    for i in 1u32.. {
        hasher.update(input);
        let n = write10(i, &mut buf);
        hasher.update(&buf[..n]);
        // write!(hasher, "{}", i).unwrap();

        hasher.finalize_into_reset(&mut hash);
        if hash[0..2] == [0, 0] && hash[2] >> 4 == 0 {
            return i;
        }
    }

    unreachable!()
}

fn part2(input: &str) -> u32 {
    // create a Md5 hasher instance
    let mut hasher = Md5::new();

    let mut hash = GenericArray::<u8, U16>::default();
    let mut buf = [0u8; 10];
    for i in 1u32.. {
        hasher.update(input);
        let n = write10(i, &mut buf);
        hasher.update(&buf[..n]);
        // write!(hasher, "{}", i).unwrap();

        hasher.finalize_into_reset(&mut hash);
        if hash[0..3] == [0, 0, 0] {
            return i;
        }
    }

    unreachable!()
}

fn main() {
    let file = std::fs::read_to_string("./input/day04.txt").unwrap();
    let file = file.trim_end();

    assert_eq!(part1(file), 254575);
    assert_eq!(part2(file), 1038736);
}
