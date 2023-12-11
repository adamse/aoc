#![feature(byte_slice_trim_ascii)]
#[derive(Debug)]
struct Input {
    galaxies1: Vec<(i64, i64)>,
    galaxies2: Vec<(i64, i64)>,
}
fn parse(i: &[u8], exp: usize) -> Input {
    let cc = memchr::memchr(b'\n', i).unwrap();
    let rc = i.len() / cc;
    let mut cols = vec![];
    let mut empties = 0;
    for c in 0..cc {
        cols.push(empties);
        let mut expand = true;
        for r in 0..rc {
            expand &= i[r * (cc + 1) + c] == b'.';
        }
        if expand { empties += 1; }

    }
    let mut rows = vec![];
    let mut empties = 0;
    for r in 0..rc {
        rows.push(empties);
        let expand = memchr::memchr(b'#', &i[r * (cc + 1)..][..cc]).is_none();
        if expand { empties += 1; }
    }
    let mut galaxies1 = vec![];
    let mut galaxies2 = vec![];
    for r in 0..rc {
        for c in 0..cc {
            if i[r * (cc + 1) + c] == b'#' {
                let rx = rows[r];
                let cx = cols[c];
                galaxies1.push(((r + rx) as i64, (c + cx) as i64));
                galaxies2.push((((r - rx) + rx * exp) as i64, ((c - cx) + cx * exp) as i64));
            }
        }
    }
    Input {
        galaxies1,
        galaxies2,
    }
}
fn p1(i: &Vec<(i64, i64)>) -> i64 {
    let mut s = 0;
    for (r, c) in i.iter() {
        for (r1, c1) in i.iter() {
            s += (r - r1).abs() + (c - c1).abs();
        }
    }
    s / 2
}
fn main() {
    let i = std::fs::read("inp.txt").unwrap(); let exp = 1_000_000;
    // let i = std::fs::read("inp1.txt").unwrap(); let exp = 10;
    // let i = std::fs::read("inp1.txt").unwrap(); let exp = 100;
    let i = i.trim_ascii();

    let i = parse(i, exp);
    println!("{}", p1(&i.galaxies1));
    println!("{}", p1(&i.galaxies2));
}
