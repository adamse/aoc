#![feature(byte_slice_trim_ascii)]

use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq)]
enum List {
    Val(u32),
    List(Vec<List>),
}

#[must_use]
fn assume(x: bool) -> Option<()> {
    x.then(|| ())
}

#[must_use]
fn parse_num(buf: &[u8]) -> Option<(&[u8], u32)> {
    // find first non-digit
    let last_digit = buf.iter().position(|x| !x.is_ascii_digit())?;

    let num = unsafe {
        // SAFE: only ascii digits in the range
        std::str::from_utf8_unchecked(&buf[0..last_digit])
    }.parse::<u32>().ok()?;

    Some((&buf[last_digit..], num))
}

#[must_use]
fn parse_list(buf: &[u8]) -> Option<(&[u8], Vec<List>)> {
    let mut contents = vec![];

    // a list must start with [
    assume(buf[0] == b'[')?;

    let mut buf = &buf[1..];

    // parse list items
    loop {
        if let Some((rest, num)) = parse_num(buf) {
            contents.push(List::Val(num));
            buf = rest;
        } else if buf[0] == b'[' {
            // another list
            let (rest, list) = parse_list(buf)?;
            contents.push(List::List(list));
            buf = rest;
        }

        if buf[0] == b']' {
            // end of list
            buf = &buf[1..];
            break;
        }

        assume(buf[0] == b',')?;
        buf = &buf[1..];
    }

    Some((buf, contents))
}

#[must_use]
fn parse_packet(mut buf: &[u8]) -> Option<(&[u8], List)> {
    buf = buf.trim_ascii_start();

    // fail if no input
    if buf.is_empty() { return None; }

    let (rest, packet) = parse_list(buf)?;

    Some((rest, List::List(packet)))
}

#[derive(Debug)]
#[derive(PartialEq)]
enum Cmp {
    DontCare,
    Good,
    Bad,
}

fn compare_many(a: &Vec<List>, b: &Vec<List>) -> Cmp {
    let mut a = a.iter();
    let mut b = b.iter();

    loop {
        match (a.next(), b.next()) {
            (Some(a), Some(b)) => match compare(a, b) {
                Cmp::DontCare => continue,
                res => break res,
            }
            (None, Some(_)) => break Cmp::Good,
            (Some(_), None) => break Cmp::Bad,
            (None, None) => break Cmp::DontCare,
        }
    }
}

fn compare(a: &List, b: &List) -> Cmp {
    use crate::List::*;
    match (a, b) {
        (&Val(a), &Val(b)) =>
            if a == b { Cmp::DontCare
            } else if a < b { Cmp::Good
            } else { Cmp::Bad },
        (Val(a), b) => compare(&List(vec![Val(*a)]), b),
        (List(_), Val(b)) => compare(a, &List(vec![Val(*b)])),
        (List(a), List(b)) => compare_many(a, b),
    }
}

fn main() -> std::io::Result<()> {
    let buf = std::fs::read_to_string("big")?;
    let mut buf = buf.as_bytes();

    let mut packet_no = 1;
    let mut total = 0;

    let mut all_packets = vec![];

    while let Some((rest, a)) = parse_packet(buf) {
        let (rest, b) = parse_packet(rest).unwrap();
        buf = rest;
        let cmp = compare(&a, &b);
        if cmp == Cmp::Good {
            // println!("{packet_no} {cmp:?}");
            total += packet_no;
        }
        all_packets.push(a);
        all_packets.push(b);
        packet_no += 1;
    }

    println!("{total}");

    let sep1 = parse_packet("[[2]]".as_bytes()).unwrap().1;
    let sep2 = parse_packet("[[6]]".as_bytes()).unwrap().1;
    all_packets.push(sep1.clone());
    all_packets.push(sep2.clone());

    all_packets.sort_by(|a, b|
        if compare(a, b) == Cmp::Good { Ordering::Less } else { Ordering::Greater } );

    // for p in &all_packets {
    //     println!("{p:?}");
    // }

    let i1 = all_packets.iter().position(|x| x == &sep1).unwrap();
    let i2 = all_packets.iter().position(|x| x == &sep2).unwrap();

    println!("{}", (i1 + 1) * (i2 + 1));

    Ok(())
}
