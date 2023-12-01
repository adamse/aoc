#![feature(byte_slice_trim_ascii)]
// 56465
// 55902
fn p1(c: &[u8]) -> u64 {
    // assert!(c.is_ascii());
    let mut sum = 0u64;
    for line in c.split(|x| *x == b'\n') {
        sum += 10 * (line.iter().copied().find(u8::is_ascii_digit).unwrap() - b'0') as u64;
        sum += (line.iter().copied().rev().find(u8::is_ascii_digit).unwrap() - b'0') as u64;
    }
    sum
}
fn p2(c: &[u8]) -> u64 {
    // let c = std::fs::read("inp2.txt").unwrap();
    // let c = c.trim_ascii();

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
        // println!("{v}");
        sum += v;
    }
    sum
}
fn main() {
    let c = std::fs::read("inp.txt").unwrap();
    let c = c.trim_ascii();
    println!("{}", p1(c));
    println!("{}", p2(c));

}
