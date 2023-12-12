#![feature(byte_slice_trim_ascii)]
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum Condition {
    Operational,
    Damaged,
    Unknown
}
use Condition::*;
impl Condition {
    fn parse(x: u8) -> Self {
        match x {
            b'.' => Operational,
            b'#' => Damaged,
            b'?' => Unknown,
            _ => panic!(),
        }
    }
}
fn can_place(grp: u32, conds: &[Condition]) -> Option<&[Condition]> {
    let grp = grp as usize;
    if grp > conds.len() { return None; }
    if conds[..grp].iter().any(|x| Operational.eq(x)) { return None; }

    if grp == conds.len() {
        Some(&conds[grp..])
    } else {
        if conds[grp] == Damaged {
            None
        } else {
            Some(&conds[grp+1..])
        }
    }
}
type Memo<'a> = std::collections::HashMap<(&'a[u32], &'a[Condition]), Option<u64>>;
fn count_arrangements_worker<'a>(memo: *mut Memo<'a>, groups: &'a [u32], conds: &'a [Condition]) -> Option<u64> {
    let Some((&grp, groups)) = groups.split_first() else {
        // no groups left to place, ensure we have no known damaged springs left
        if conds.iter().any(|x|matches!(x, Damaged)) { return None; }
        return Some(1);
    };
    let mut arrangements = 0;
    for skip in 0..conds.len() {
        // println!("");
        // print!("{grp} {:?} {:?} {} ", groups, &conds[skip..], skip);
        if (&conds[..skip]).iter().any(|x|Damaged.eq(x)) { continue; }
        let Some(conds) = can_place(grp, &conds[skip..]) else { continue; };
        // print!("placed ");
        let Some(rest) = count_arrangements_worker_memo(memo, groups, conds) else { continue; };
        arrangements += rest;
    }
    if arrangements == 0 {
        None
    } else {
        Some(arrangements)
    }
}
fn count_arrangements_worker_memo<'a>(memo: *mut Memo<'a>, groups: &'a [u32], conds: &'a [Condition]) -> Option<u64> {
    if let Some(res) = unsafe { &*memo }.get(&(groups, conds)) {
        *res
    } else {
        let res = { count_arrangements_worker(memo, groups, conds) };
        unsafe { &mut *memo }.insert((groups, conds), res);
        res
    }
}
fn count_arrangements(groups: &[u32], conds: &[Condition]) -> u64 {
    let mut memo = Memo::new();
    count_arrangements_worker_memo(&mut memo, groups, conds).unwrap_or(0)
}
type I = Vec<(Vec<u32>, Vec<Condition>)>;
fn parse(i: &[u8]) -> I {
    i.split(|x| b'\n'.eq(x)).map(|line| {
        let spc = memchr::memchr(b' ', line).unwrap();
        let (conds, groups) = line.split_at(spc);
        let groups = std::str::from_utf8(&groups[1..]).unwrap();
        (groups.split(',').filter_map(|x|x.parse().ok()).collect(),
            conds.iter().copied().map(Condition::parse).collect())
    }).collect()
}
fn p1(i: &I) -> u64 {
    let mut total = 0;
    for (groups, conds) in i {
        total += count_arrangements(&groups[..], &conds[..]);
    }
    total
}
fn p2(i: &I) -> u64 {
    let mut total = 0;
    for (groups, conds) in i {
        let mut extended_groups = groups.clone();
        let mut extended_conds = conds.clone();
        for _ in 0..4 {
            extended_groups.extend_from_slice(&groups[..]);
            extended_conds.push(Unknown);
            extended_conds.extend_from_slice(&conds[..]);
        }
        total += count_arrangements(&extended_groups[..], &extended_conds[..]) as u64;
        eprint!(".");
    }
    eprintln!("");
    total
}
// const III: &[u8] = include_bytes!("../inp1.txt");
fn main() {
    // let conds = b"???.###";
    // let conds = conds.map(Condition::parse);
    // let groups = [1,1,3];
    // println!("{}", count_arrangements(&groups[..], &conds[..]));
    let i = std::fs::read("inp.txt").unwrap();
    // let i = std::fs::read("inp1.txt").unwrap();
    // let i = std::fs::read("inp2.txt").unwrap();
    // let i = III;
    let i = i.trim_ascii();
    let i = parse(i);
    println!("{}", p1(&i));
    println!("{}", p2(&i));
}
