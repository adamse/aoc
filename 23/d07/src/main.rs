#![feature(byte_slice_trim_ascii)]
#![feature(slice_partition_dedup)]
#![feature(slice_group_by)]

use std::cmp::Ordering;

use arrayvec::ArrayVec;

fn parse_num(i: &[u8]) -> u32 {
    i.iter().fold(0, |acc,e| acc * 10 + (e - b'0') as u32)
}
fn parse_hand1(mut hand: [u8; 5]) -> [u8; 5] {
    for i in 0..5 {
        hand[i] = match hand[i] {
            b'A' => 14,
            b'K' => 13,
            b'Q' => 12,
            b'J' => 11,
            b'T' => 10,
            x => x - b'0',
        };
    }
    hand
}
fn parse(i: &[u8]) -> Vec<([u8; 5], u32)> {
    i.split(|x|*x==b'\n').map(|l| {
        (parse_hand1(l[..5].try_into().unwrap()), parse_num(&l[6..]))
    }).collect()
}
fn hand_type(mut i: [u8; 5]) -> u32 {
    i.sort();
    let mut i = i
        .group_by(u8::eq)
        .map(|x| x.len())
        .collect::<ArrayVec<_, 5>>();
    i.sort();
    match &i[..] {
        [5] => 6,
        [1,4] => 5,
        [2,3] => 4,
        [1,1,3] => 3,
        [1,2,2] => 2,
        [1,1,1,2] => 1,
        [1,1,1,1,1] => 0,
        _ => panic!(),
    }
}
fn p1(i: Vec<([u8; 5], u32)>) -> u32 {
    let mut i = i.into_iter()
        .map(|x| (x.0, hand_type(x.0), x.1))
        .collect::<Vec<_>>();

    i.sort_by(|a,b|
        match a.1.cmp(&b.1) {
            Ordering::Equal => { a.0.cmp(&b.0) },
            x => x,
        });

    i.into_iter()
        .enumerate()
        .map(|(ix, x)| (ix+1) as u32 * x.2)
        .sum()
}
fn parse_hand2(mut hand: [u8; 5]) -> [u8; 5] {
    for i in 0..5 {
        hand[i] = match hand[i] {
            b'A' => 14,
            b'K' => 13,
            b'Q' => 12,
            b'J' => 1,
            b'T' => 10,
            x => x - b'0',
        };
    }
    hand
}
fn parse2(i: &[u8]) -> Vec<([u8; 5], u32)> {
    i.split(|x|*x==b'\n').map(|l| {
        (parse_hand2(l[..5].try_into().unwrap()), parse_num(&l[6..]))
    }).collect()
}
fn hand_type2(mut i: [u8; 5]) -> u32 {
    i.sort();

    // count no of Jokers in the hand
    let mut j = 0;
    while let Some(1) = i.get(j) { j+=1; }

    if j >= 4 { return 6; }

    let mut i = i[j..]
        .group_by(u8::eq)
        .map(|x| x.len() as u8)
        .collect::<ArrayVec<_, 5>>();
    i.sort();

    // add Jokers to the biggest group for more win
    *i.last_mut().unwrap() += j as u8;

    match &i[..] {
        [5] => 6,
        [1,4] => 5,
        [2,3] => 4,
        [1,1,3] => 3,
        [1,2,2] => 2,
        [1,1,1,2] => 1,
        [1,1,1,1,1] => 0,
        _ => panic!(),
    }
}
fn p2(i: Vec<([u8; 5], u32)>) -> u32 {
    let mut i = i.into_iter()
        .map(|x| (x.0, hand_type2(x.0), x.1))
        .collect::<Vec<_>>();

    i.sort_by(|a,b|
        match a.1.cmp(&b.1) {
            Ordering::Equal => { a.0.cmp(&b.0) },
            x => x,
        });

    i.into_iter()
        .enumerate()
        .map(|(ix, x)| (ix+1) as u32 * x.2)
        .sum()
}
fn main() {
    let i = std::fs::read("inp.txt").unwrap();
    // let i = std::fs::read("inp1.txt").unwrap();
    // let i = std::fs::read("inp2.txt").unwrap();
    let i = i.trim_ascii();

    {
        let st = std::time::Instant::now();
        let i = parse(&i[..]);
        println!("{}", p1(i));
        eprintln!("{}us", st.elapsed().as_micros());
    }

    let st = std::time::Instant::now();
    let i = parse2(&i[..]);
    println!("{}", p2(i));
    eprintln!("{}us", st.elapsed().as_micros());
}
