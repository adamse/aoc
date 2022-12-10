#![feature(slice_group_by)]
#![feature(iter_array_chunks)]
#![feature(byte_slice_trim_ascii)]

use std::arch::asm;
use std::io::{self, Read};
use std::fs::File;

#[repr(C)]
struct Range(u32, u32);

impl Range {
    fn parse(a: &[u8], b: &[u8]) -> Option<Self> {
        let a = std::str::from_utf8(a).ok()?.parse::<u32>().ok()?;
        let b = std::str::from_utf8(b).ok()?.parse::<u32>().ok()?;

        Some(Range(a, b))
    }

    fn parse2(a: &[u8]) -> Option<[Self; 2]> {
        // 123456
        // 12-30,
        //
        // 123456
        // 12-13\
        //      n
        //
        // max 6 chars, we can easily fit this into a u64
        // or we can fit 2 into a u128
        // so lets parse 2 ranges at a time
        //
        // 1. find the ranges of the numbers
        // 2. align them?

        // Shuffle the digits of one range into place
        // 1-1   -> 01 01 (4 x u8?)
        // 1-22  -> 01 22
        // 22-22 -> 22 22
        // 22-1  -> 22 01
        //
        //                 mask                   length
        // 1-1,1-1     --> f0f0f0f     0f0f0f     8
        // 22-1,1-1    --> ff0f0f0f    f0f0f0f    9
        // 1-22,1-1    --> f0ff0f0f    0ff0f0f    9
        // 22-22,1-1   --> ff0ff0f0f   f0ff0f0f   10
        // 1-1,22-1    --> f0f0ff0f    0f0ff0f    9
        // 22-1,22-1   --> ff0f0ff0f   f0f0ff0f   10
        // 1-22,22-1   --> f0ff0ff0f   0ff0ff0f   10
        // 22-22,22-1  --> ff0ff0ff0f  f0ff0ff0f  11
        // 1-1,1-22    --> f0f0f0ff    0f0f0ff    9
        // 22-1,1-22   --> ff0f0f0ff   f0f0f0ff   10
        // 1-22,1-22   --> f0ff0f0ff   0ff0f0ff   10
        // 22-22,1-22  --> ff0ff0f0ff  f0ff0f0ff  11
        // 1-1,22-22   --> f0f0ff0ff   0f0ff0ff   10
        // 22-1,22-22  --> ff0f0ff0ff  f0f0ff0ff  11
        // 1-22,22-22  --> f0ff0ff0ff  0ff0ff0ff  11
        // 22-22,22-22 --> ff0ff0ff0ff f0ff0ff0ff 12
        //                 01234567890
        // ->
        //   11223344
        const SHUFFLE: [u8; 4] = [
            0,
            0,
            0,
            0,
        ];

        const MUL: [u8; 4] = [10, 1, 10, 1];

        unsafe {
            asm!(
                "",
            );
        }

        None
    }

    fn contains(&self, other: &Range) -> bool {
        self.0 <= other.0 && other.1 <= self.1
    }

    fn overlaps(&self, other: &Range) -> bool {
        let Range(a, b) = self;
        let Range(c, d) = other;

        // this one
        // a  b
        //   c  d
        // but not this
        //    a  b
        // c d
        c <= b && a <= d ||
        // this one
        //  a  b
        // c d
        // but not this
        // a  b
        //      c d
            a <= d && c <= b
    }
}

fn main() -> io::Result<()> {
    let filename = "big";

    let mut file = File::open(filename)?;
    let mut buf: Vec<u8> = vec![];
    file.read_to_end(&mut buf)?;

    let pairs =
        buf.trim_ascii()
            .split(|&x| x == b'-' || x == b',' || x == b'\n')
            .array_chunks::<4>();

    let mut n_redundant_elves = 0u32;
    let mut n_overlaps = 0u32;

    for [a, b, c, d] in pairs {
        let assignment1 = Range::parse(a, b).unwrap();
        let assignment2 = Range::parse(c, d).unwrap();

        if assignment1.contains(&assignment2) || assignment2.contains(&assignment1) {
            n_redundant_elves += 1;
        }

        if assignment1.overlaps(&assignment2) {
            n_overlaps += 1;
        }
    }

    println!("{n_redundant_elves}");
    println!("{n_overlaps}");

    {
        // make some bitmasks
        //          123456...
        // 2-4 -> 0b011100...
        fn make_mask(a: u32, b: u32) -> u128 {
            !((1u128 << (a-1)) - 1) &
                ((1u128 << b) - 1)
        }
        let m = make_mask(2, 4);
        println!("{m} {m:b}");
    }

    Ok(())
}
