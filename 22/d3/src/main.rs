#![feature(array_chunks)]
#![feature(byte_slice_trim_ascii)]
#![feature(iter_array_chunks)]

use std::fs::File;
use std::io::{self, Read, BufRead};

use std::collections::HashSet;

fn get_prio(char: char) -> u64 {
    if ('A'..='Z').contains(&char) {
        (char as u8 - b'A') as u64 + 27
    } else if ('a'..='z').contains(&char) {
        (char as u8 - b'a') as u64 + 1
    } else {
        panic!("not good");
    }
}

fn find_common2(strs: &[&str]) -> Option<char> {
    let mut intersection = None;
    for &str in strs {
        let chars: HashSet<_> = str.chars().collect();
        if let Some(a) = intersection {
            intersection = Some(&a & &chars);
        } else {
            intersection = Some(chars);
        }
    }

    intersection?.into_iter().next()
}

fn find_matching(a: &str, b: &str) -> Option<char> {
    let a: HashSet<_> = a.chars().collect();
    let b: HashSet<_> = b.chars().collect();

    (&a & &b).into_iter().next()
}

fn find_common(a: &str, b: &str, c: &str) -> Option<char> {
    let a: HashSet<_> = a.chars().collect();
    let b: HashSet<_> = b.chars().collect();
    let c: HashSet<_> = c.chars().collect();

    (&(&a & &b) & &c).into_iter().next()
}

mod quick {
    use std::ops::BitOr;
    use std::ops::BitAnd;

    pub fn get_prio_bit(char: u8) -> u64 {
        match char {
            b'A'..=b'Z' => 1 << ((char - b'A') as u64 + 27),
            b'a'..=b'z' => 1 << ((char - b'a') as u64 + 1),
            _ => unsafe { std::hint::unreachable_unchecked() },
        }
    }

    fn bag_to_bitmask(bag: &[u8]) -> u64 {
        bag.iter()
            .copied()
            .map(get_prio_bit)
            .fold(0, BitOr::bitor)
    }

    pub fn solve1(str: &[u8]) -> u32 {
        str.trim_ascii()
            .split(|&chr| chr == b'\n')
            .map(|bag| {
                let (a, b) = bag.split_at(bag.len() / 2);
                bag_to_bitmask(a) & bag_to_bitmask(b)
            })
            .map(u64::trailing_zeros)
            .sum()
    }

    pub fn solve2(str: &[u8]) -> u32 {
        str.trim_ascii()
            .split(|&chr| chr == b'\n')
            .array_chunks::<3>()
            .map(|group|
                group.into_iter()
                    .map(bag_to_bitmask)
                    .fold(!0, BitAnd::bitand))
            .map(u64::trailing_zeros)
            .sum()
    }
}

fn main() -> io::Result<()> {
    let filename = "big";
    let file = File::open(filename)?;

    let mut sum = 0u64;

    for line in io::BufReader::new(file).lines() {
        let line = line?;
        let mid = line.len() / 2;
        let (a, b) = line.split_at(mid);

        if let Some(matching) = find_common2(&[a, b]) {
            let prio = get_prio(matching);
            sum += prio;

            // println!("{matching} {prio}");
        }
    }
    println!("{sum}");
    {
        let mut file = File::open(filename)?;
        let mut buf: Vec<u8> = vec![];
        file.read_to_end(&mut buf)?;
        let res = quick::solve1(&buf);
        println!("{res}");
    }

    let mut sum = 0u64;

    let file = File::open(filename)?;
    let groups: Vec<String> =
        io::BufReader::new(file)
            .lines()
            .map(|x|x.unwrap())
            .collect();

    for [a, b ,c] in groups.array_chunks::<3>() {
        if let Some(common) = find_common2(&[a, b, c]) {
            let prio = get_prio(common);
            sum += prio;
            // println!("{common} {prio}");
        }
    }

    println!("{sum}");
    {
        let mut file = File::open(filename)?;
        let mut buf: Vec<u8> = vec![];
        file.read_to_end(&mut buf)?;
        let res = quick::solve2(&buf);
        println!("{res}");
    }

    Ok(())
}
