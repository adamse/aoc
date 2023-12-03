#![feature(byte_slice_trim_ascii)]
#![feature(slice_internals)]
use std::collections::{HashMap, HashSet};

fn partmap(i: &[u8]) -> HashMap<(i32, i32), u8> {
    let mut parts = HashMap::new();

    for (y, line) in i.split(|x| *x == b'\n').enumerate() {
        for (x, &s) in line.iter().enumerate() {
            if !(s == b'.' || s.is_ascii_digit()) {
                parts.insert((y as i32, x as i32), s);
            }
        }
    }

    parts
}
fn a(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
    (a.0 + b.0, a.1 + b.1)
}
fn cartesian_prod<A: Copy, B: Copy>(a: impl Iterator<Item = A>, b: impl Iterator<Item = B> + Clone) -> impl Iterator<Item = (A, B)> {
    a.flat_map(move |y| b.clone().map(move |x| (y, x)))
}
fn check_ispart(parts: &HashMap<(i32, i32), u8>, yx: (i32, i32)) -> bool {
    cartesian_prod(-1i32..=1, -1i32..=1)
        .any(|yxp| parts.contains_key(&a(yx,yxp)))
}
fn p1(parts: &HashMap<(i32, i32), u8>, i: &[u8]) -> u32 {
    let mut sum = 0;
    for (y, l) in i.split(|x|*x==b'\n').enumerate() {
        let mut ispart = false;
        let mut num = 0;
        for (x, c) in l.iter().enumerate() {
            if c.is_ascii_digit() {
                num = num * 10 + (c - b'0') as u32;
                ispart |= check_ispart(parts, (y as i32, x as i32));
            } else {
                if ispart {
                    sum += num;
                }
                num = 0;
                ispart = false;
            }
        }
        if ispart {
            sum += num;
        }
    }
    sum
}
fn adjacent_gears(parts: &HashMap<(i32, i32), u8>, yx: (i32, i32)) -> Vec<(i32, i32)> {
    cartesian_prod(-1i32..=1, -1i32..=1)
        .filter_map(|yxp| {
            let j = a(yx, yxp);
            parts.get(&j).and_then(|x| (*x==b'*').then_some(j))
        }).collect()
}
fn p2(parts: &HashMap<(i32, i32), u8>, i: &[u8]) -> u32 {
    let mut gear_partnos = HashMap::new();
    for (y, l) in i.split(|x|*x==b'\n').enumerate() {
        let mut partno = 0;
        let mut gears = vec![];
        for (x, c) in l.iter().enumerate() {
            if c.is_ascii_digit() {
                partno = partno * 10 + (c - b'0') as u32;
                gears.extend(adjacent_gears(parts, (y as i32, x as i32)));
            } else {
                if !gears.is_empty() {
                    for gear in gears {
                        gear_partnos.entry(gear).or_insert(HashSet::new()).insert(partno);
                    }
                }
                partno = 0;
                gears = vec![];
            }
        }
        if !gears.is_empty() {
            for gear in gears {
                gear_partnos.entry(gear).or_insert(HashSet::new()).insert(partno);
            }
        }
    }
    gear_partnos.iter().filter_map(|(_pos, partnos)|
        if partnos.len() == 2 {
            Some(partnos.iter().product::<u32>())
        } else {
            None
        }).sum()
}
fn main() {
    let i = std::fs::read("inp.txt").unwrap();
    // let i = std::fs::read("inp1.txt").unwrap();
    let i = i.trim_ascii();

    let st = std::time::Instant::now();
    let parts = partmap(i);
    eprintln!("{}us", st.elapsed().as_micros());
    let st = std::time::Instant::now();
    println!("{}", p1(&parts, i));
    eprintln!("{}us", st.elapsed().as_micros());
    let st = std::time::Instant::now();
    println!("{}", p2(&parts, i));
    eprintln!("{}us", st.elapsed().as_micros());
}
