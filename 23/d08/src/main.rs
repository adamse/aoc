#![feature(byte_slice_trim_ascii)]
#![feature(portable_simd)]

use bitvec::prelude::BitVec;

#[derive(Debug)]
struct Input<'a> {
    instrs: &'a [u8],
    lefts: Vec<u32>,
    rights: Vec<u32>,
    start: Option<u32>,
    end: Option<u32>,
    starts: BitVec,
    ends: BitVec,
}

fn parse(i: &[u8]) -> Input {
    let mut it = i.split(|x| *x == b'\n');
    let instrs = it.next().unwrap();
    it.next().unwrap();

    fn no(i: &[u8]) -> u32 {
        let mut bs = [0; 4];
        bs[0] = i[0];
        bs[1] = i[1];
        bs[2] = i[2];
        u32::from_be_bytes(bs)
    }

    let mut node_to_ix = std::collections::HashMap::<u32, u32>::new();
    let mut lefts = vec![];
    let mut rights = vec![];
    let mut starts = BitVec::new();
    let mut ends = BitVec::new();

    for (ix, line) in it.enumerate() {
        let ix = ix as u32;
        let node = no(line);
        let left = no(&line[7..]);
        let right = no(&line[12..]);
        node_to_ix.insert(node, ix);
        lefts.push(left);
        rights.push(right);
        starts.push(line[2] == b'A');
        ends.push(line[2] == b'Z');
    }

    let lefts = lefts.into_iter()
        .map(|node| node_to_ix[&node])
        .collect();
    let rights = rights.into_iter()
        .map(|node| node_to_ix[&node])
        .collect();

    Input {
        instrs,
        lefts,
        rights,
        start: node_to_ix.get(&no(b"AAA")).copied(),
        end: node_to_ix.get(&no(b"ZZZ")).copied(),
        starts,
        ends,
    }
}

fn p1(i: &Input) -> u32 {
    let mut steps = 0u32;
    let mut current = i.start.unwrap();
    while current != i.end.unwrap() {
        match i.instrs[steps as usize % i.instrs.len()] {
            b'L' => {
                steps += 1;
                current = i.lefts[current as usize];
            },
            b'R' => {
                steps += 1;
                current = i.rights[current as usize];
            },
            _ => panic!(),
        }
    }

    steps
}

#[inline(never)]
fn p2_brute(i: &Input) -> u32 {
    let mut steps = 0u32;
    let mut current = i.starts.iter().enumerate()
        .filter_map(|(ix, st)| st.then_some(ix as u32))
        .collect::<Vec<_>>();

    let st = std::time::Instant::now();

    while current.iter().any(|ix| !i.ends[*ix as usize]) {
        match i.instrs[steps as usize % i.instrs.len()] {
            b'L' => {
                steps += 1;
                for current in current.iter_mut() {
                    *current = i.lefts[*current as usize];
                }
            },
            b'R' => {
                steps += 1;
                for current in current.iter_mut() {
                    *current = i.rights[*current as usize];
                }
            },
            _ => unsafe { std::hint::unreachable_unchecked() },
        }

        if steps & ((1<<26) - 1) == 0 {
            progress(steps as u64, &st);
        }
    }

    steps
}

use std::simd::prelude::*;

#[inline(never)]
fn progress(steps: u64, st: &std::time::Instant) {
    let el = st.elapsed().as_millis();
    println!("{:012} ms {steps} {:.3} steps/s", el, 1000.0 * steps as f64 / el as f64);
}
#[inline(never)]
fn p2_brute_simd(i: &Input) -> u64 {
    // compress the data a little and use u16 for node ids
    // to identify end states we set the upper bit of the node id
    let topbit = (1 << 15) as u16;
    assert!(i.lefts.len() < topbit as usize);
    let lefts = i.lefts.iter().copied()
        .map(|x| if i.ends[x as usize] { topbit } else { 0 } | x as u16)
        .collect::<Vec<_>>();
    let rights = i.rights.iter().copied()
        .map(|x| if i.ends[x as usize] { topbit } else { 0 } | x as u16)
        .collect::<Vec<_>>();

    let mut starts = i.starts.iter().enumerate()
        .filter_map(|(ix, st)| st.then_some(ix as u16))
        .collect::<Vec<_>>();
    assert!(starts.len() <= 8);
    let ghosts = starts.len();
    // just copy the first start position to the remaining slots so we don't have to mask
    while starts.len() < 8 {
        starts.push(starts[0]);
    }

    type V = Simd<u16, 8>;
    let all_enable = Mask::splat(true);

    let mut current = V::from_slice(&starts[..8]);
    let endmask = V::splat(topbit);
    let ixmask = V::splat(topbit - 1);

    let st = std::time::Instant::now();

    let mut steps = 0u64;
    while current & endmask != endmask {
        let ixs = current & ixmask;
        let ixs = ixs.cast();
        match i.instrs[steps as usize % i.instrs.len()] {
            b'L' => {
                current = unsafe {
                    V::gather_select_unchecked(&lefts, all_enable, ixs, current)
                };
            },
            b'R' => {
                current = unsafe {
                    V::gather_select_unchecked(&rights, all_enable, ixs, current)
                };
            },
            _ => unsafe { std::hint::unreachable_unchecked() },
        }
        steps += 1;

        if steps & ((1<<26) - 1) == 0 {
            progress(steps, &st);
        }
    }

    steps
}

fn p2(i: &Input) -> u64 {
    let starts = i.starts.iter().enumerate()
        .filter_map(|(ix, st)| st.then_some(ix as u32))
        .collect::<Vec<_>>();

    // find end
    // find loop
    let mut looplens = vec![];
    for start in starts {
        let mut steps = 0;
        let mut current = start;
        let mut loopstart = None;
        loop {
            if i.ends[current as usize] {
                if let Some((node, start)) = loopstart {
                    assert!(node == current);
                    assert!(start == steps-start);
                    looplens.push(start);
                    println!("second time end; first={node},{start}, current={current}, looplen={}", steps-start);
                    break;
                } else {
                    loopstart = Some((current, steps));
                }
            }
            match i.instrs[steps as usize % i.instrs.len()] {
                b'L' => {
                    steps += 1;
                    current = i.lefts[current as usize];
                },
                b'R' => {
                    steps += 1;
                    current = i.rights[current as usize];
                },
                _ => panic!(),
            }
        }
    }

    looplens.into_iter().reduce(lcm).unwrap()
}
fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}
fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn main() {
    let i = std::fs::read("inp.txt").unwrap();
    // let i = std::fs::read("inp1.txt").unwrap();
    // let i = std::fs::read("inp2.txt").unwrap();
    // let i = std::fs::read("inp3.txt").unwrap();
    let i = i.trim_ascii();

    let inp = parse(i);
    println!("{}", p1(&inp));

    let i = std::fs::read("inp.txt").unwrap();
    // let i = std::fs::read("inp1.txt").unwrap();
    // let i = std::fs::read("inp2.txt").unwrap();
    // let i = std::fs::read("inp3.txt").unwrap();
    let i = i.trim_ascii();

    let inp = parse(i);
    // println!("{}", p2_brute(&inp));
    // println!("{}", p2_brute_simd(&inp));
    println!("{}", p2(&inp));
}
