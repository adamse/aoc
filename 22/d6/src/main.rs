#![feature(array_windows)]

use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let buf = fs::read_to_string("big")?;
    let buf = buf.as_bytes();
    // let buf = "mjqjpqmgbljsphdztnvjfqwrcgsmlb".as_bytes();

    for (ii, bytes) in buf.array_windows::<4>().enumerate() {
        let set = bytes.into_iter().map(|byte| byte - b'a')
            .fold(0u32, |acc, byte| acc | 1u32 << byte);
        if set.count_ones() == 4 {
            let ii = ii + 4; // to account for window
            println!("{ii}");
            break;
        }
    }

    for (ii, bytes) in buf.array_windows::<14>().enumerate() {
        let set = bytes.into_iter().map(|byte| byte - b'a')
            .fold(0u32, |acc, byte| acc | 1u32 << byte);

        if set.count_ones() == 14 {
            let ii = ii + 14; // to account for window
            println!("{ii}");
            break;
        }
    }

    Ok(())
}
