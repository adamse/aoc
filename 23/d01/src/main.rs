#![feature(byte_slice_trim_ascii)]
// 56465
// 55902
fn p1(c: &[u8]) -> u64 {
    // assert!(c.is_ascii());
    let st = std::time::Instant::now();
    let mut sum = 0u64;
    for line in c.split(|x| *x == b'\n') {
        sum += 10 * (line.iter().copied().find(u8::is_ascii_digit).unwrap() - b'0') as u64;
        sum += (line.iter().copied().rev().find(u8::is_ascii_digit).unwrap() - b'0') as u64;
    }
    println!("{}us", st.elapsed().as_micros());
    sum
}
fn p2(c: &[u8]) -> u64 {
    let st = std::time::Instant::now();

    const NUMS: [&[u8]; 9] = [
        b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
    ];


    let mut sum = 0u64;
    for line in c.split(|x| *x == b'\n') {
        let a = (0..line.len())
            .find_map(|start| {
                if line[start].is_ascii_digit() {
                    Some((line[start] - b'0') as u64)
                } else if let Some((_, val)) =
                        NUMS.iter().zip(1u64..=9).find(|(n, _)| line[start..].starts_with(n)) {
                    Some(val)
                } else {
                    None
                }
            }).unwrap();

        let b = (0..line.len()).rev()
            .find_map(|start| {
                if line[start].is_ascii_digit() {
                    Some((line[start] - b'0') as u64)
                } else if let Some((_, val)) =
                        NUMS.iter().zip(1u64..=9).find(|(n, _)| line[start..].starts_with(n)) {
                    Some(val)
                } else {
                    None
                }
            }).unwrap();

        let v = 10 * a + b;
        sum += v;
    }
    println!("{}us", st.elapsed().as_micros());
    sum
}
const DIGI: [u64; 18] = {
    let arr = [
        u64::from_le_bytes(*b"1\0\0\0\0\0\0\0"),
        u64::from_le_bytes(*b"2\0\0\0\0\0\0\0"),
        u64::from_le_bytes(*b"3\0\0\0\0\0\0\0"),
        u64::from_le_bytes(*b"4\0\0\0\0\0\0\0"),
        u64::from_le_bytes(*b"5\0\0\0\0\0\0\0"),
        u64::from_le_bytes(*b"6\0\0\0\0\0\0\0"),
        u64::from_le_bytes(*b"7\0\0\0\0\0\0\0"),
        u64::from_le_bytes(*b"8\0\0\0\0\0\0\0"),
        u64::from_le_bytes(*b"9\0\0\0\0\0\0\0"),
        u64::from_le_bytes(*b"one\0\0\0\0\0"),
        u64::from_le_bytes(*b"two\0\0\0\0\0"),
        u64::from_le_bytes(*b"three\0\0\0"),
        u64::from_le_bytes(*b"four\0\0\0\0"),
        u64::from_le_bytes(*b"five\0\0\0\0"),
        u64::from_le_bytes(*b"six\0\0\0\0\0"),
        u64::from_le_bytes(*b"seven\0\0\0"),
        u64::from_le_bytes(*b"eight\0\0\0"),
        u64::from_le_bytes(*b"nine\0\0\0\0"),
    ];
    arr
};
const MASK: [u64; 18] = [
    0xff,
    0xff,
    0xff,
    0xff,
    0xff,
    0xff,
    0xff,
    0xff,
    0xff,
    0xffffff,
    0xffffff,
    0xffffffffff,
    0xffffffff,
    0xffffffff,
    0xffffff,
    0xffffffffff,
    0xffffffffff,
    0xffffffff,
];

const VAL: [u8; 18] = [
    1,
    2,
    3,
    4,
    5,
    6,
    7,
    8,
    9,
    1,
    2,
    3,
    4,
    5,
    6,
    7,
    8,
    9,
];
fn p2swar(c: &[u8]) -> u64 {
    let st = std::time::Instant::now();
    fn mku64(i: &[u8]) -> u64 {
        let mut a = [0u8; 8];
        let c = 8.min(i.len());
        (&mut a[..c]).copy_from_slice(&i[..c]);
        u64::from_le_bytes(a)
    }

    let mut sum = 0u64;
    for line in c.split(|x| *x == b'\n') {
        let mut ll = [0u8; 54 + 7];
        (&mut ll[..line.len()]).copy_from_slice(line);
        // let mut vals = [0u8; 54 + 7];

        let mut a = 0;
        let mut b = 0;

        for s in 0..line.len() {
            let digit = u64::from_le_bytes(<[u8; 8]>::try_from(&ll[s..s+8]).unwrap());
            let val = (0..18).find_map(|i| {
                if MASK[i] & digit == DIGI[i] {
                    Some(VAL[i])
                } else {
                    None
                }
            }).unwrap_or(0) as u64;
            if a == 0 {
                a = val;
            }
            if val != 0 {
                b = val;
            }
        }

        sum += 10 * a + b;
    }

    println!("{}us", st.elapsed().as_micros());
    sum
}
fn main() {
    let c = std::fs::read("inp.txt").unwrap();
    let c = c.trim_ascii();
    println!("{}", p1(c));
    // let c = std::fs::read("inp2.txt").unwrap(); let c = c.trim_ascii();
    println!("{}", p2(c));
    // not faster
    println!("{}", p2swar(c));
}
