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

    let mut sum = 0u32;
    for line in c.split(|x| *x == b'\n') {
        let mut ll = [0u8; 54 + 7];
        (&mut ll[..line.len()]).copy_from_slice(line);
        // let mut vals = [0u8; 54 + 7];

        let mut a = 0;
        let mut b = 0;

        for s in 0..line.len() {
            let digit = u64::from_le_bytes(<[u8; 8]>::try_from(&ll[s..s+8]).unwrap());
            let val = (0..18).map(|i| {
                (MASK[i] & digit == DIGI[i]) as u8 * VAL[i] as u8
            }).sum::<u8>();

            if a == 0 {
                a = val;
            }
            if val != 0 {
                b = val;
            }
        }

        sum += (10 * a + b) as u32;
    }

    println!("{}us", st.elapsed().as_micros());
    sum as u64
}
/*
fn p2swar2(c: &[u8]) -> u64 {
    let st = std::time::Instant::now();
    let mut vals = vec![0u8; c.len()];
    let r = c.as_ptr();

    for s in 0..c.len() {
        // can read a little out of bounds as a treat :3
        let line = unsafe { r.offset(s as isize).cast::<u64>().read_unaligned() };
        for i in 0..18 {
            let mask = MASK[i];
            let digi = DIGI[i];
            let val = VAL[i];
            vals[s] += (mask & line == digi) as u8 * val;
        }
    }

    let sum = 0;

    println!("{}us", st.elapsed().as_micros());
    sum as u64
}
*/
fn p2table(c: &[u8]) -> u64 {
    let st = std::time::Instant::now();
    let mut sum = 0;
    for line in c.split(|x| b'\n'.eq(x)) {
        let mut state = Fwd::Start as usize;
        let mut ix = 0;
        while state >= 10 {
            let byte = line[ix];
            state = FWD[byte as usize][state] as usize;
            ix += 1;
        }
        /*
        // check if we did wrong
        let old = unsafe { RES1[ii] };
        if state as u64 != old {
            println!("{old}, {state}, {}", std::str::from_utf8(line).unwrap());
            let mut state = Fwd::Start as usize;
            let mut ix = 0;
            while state >= 10 {
                let byte = line[ix];
                let start = state;
                state = FWD[byte as usize][state] as usize;
                println!("{} {:?} {:?}",
                    std::char::from_u32(byte as u32).unwrap(),
                    unsafe { (&(start as u8) as *const u8).cast::<Fwd>().read() },
                    unsafe { (&(state as u8) as *const u8).cast::<Fwd>().read() });
                ix += 1;
            }

            break;
        }*/
        sum += state as u64;
    }
    println!("{}us", st.elapsed().as_micros());

    sum
}
fn main() {
    let c = std::fs::read("inp.txt").unwrap();
    let c = c.trim_ascii();
    println!("{}", p1(c));
    // let c = std::fs::read("inp2.txt").unwrap(); let c = c.trim_ascii();
    println!("p2 {}", p2(c));
    // not faster
    // println!("p2 {}", p2swar(c));
    // println!("p2 {}", p2swar2(c));
    // barely faster?
    println!("p2 {}", p2table(c));
}

#[derive(Debug)]
#[repr(u8)]
enum Fwd {
    Invalid = 0,
    F1 = 1,
    F2 = 2,
    F3 = 3,
    F4 = 4,
    F5 = 5,
    F6 = 6,
    F7 = 7,
    F8 = 8,
    F9 = 9,
    Start = 10,

    E, EI, EIG, EIGH,
    F, FI, FIV,
       FO, FOU,
    N, NI, NIN,
    O, ON,
    S, SE, SEV, SEVE,
       SI,
    T, TH, THR, THRE,
       TW,
    END,
}
const FWD_COUNT: usize = Fwd::END as usize;


// FWD[input byte][last state] = current state
const FWD: [[u8; FWD_COUNT]; 256] = {

    use Fwd::*;
    let mut val = [[Start as u8; FWD_COUNT]; 256];

    macro_rules! tr {
        ($input:expr, $state:expr) => {
            val[$input as usize] = [$state as u8; FWD_COUNT];
        }
    }
    tr!(b'1', F1);
    tr!(b'2', F2);
    tr!(b'3', F3);
    tr!(b'4', F4);
    tr!(b'5', F5);
    tr!(b'6', F6);
    tr!(b'7', F7);
    tr!(b'8', F8);
    tr!(b'9', F9);

    tr!(b'o', O);
    tr!(b't', T);
    tr!(b'f', F);
    tr!(b's', S);
    tr!(b'e', E);
    tr!(b'n', N);

    macro_rules! tr {
        ($input:expr, $start:expr, $end:expr) => {
            val[$input as usize][$start as usize] = $end as u8;
        }
    }
    // one
    tr!(b'o', Start, O);
    tr!(b'n', O,     ON);
    tr!(b'e', ON,    F1);
    tr!(b'i', ON,    NI);
    // two
    tr!(b't', Start, T);
    tr!(b'w', T,     TW);
    tr!(b'o', TW,    F2);
    // three
    tr!(b't', Start, T);
    tr!(b'h', T,     TH);
    tr!(b'r', TH,    THR);
    tr!(b'e', THR,   THRE);
    tr!(b'e', THRE,  F3);
    tr!(b'i', THRE,  EI);
    // four
    tr!(b'f', Start, F);
    tr!(b'o', F,     FO);
    tr!(b'u', FO,    FOU);
    tr!(b'r', FOU,   F4);
    tr!(b'n', FO,    ON);
    // five
    tr!(b'f', Start, F);
    tr!(b'i', F,     FI);
    tr!(b'v', FI,    FIV);
    tr!(b'e', FIV,   F5);
    // six
    tr!(b's', Start, S);
    tr!(b'i', S,     SI);
    tr!(b'x', SI,    F6);
    // seven
    tr!(b's', Start, S);
    tr!(b'e', S,     SE);
    tr!(b'v', SE,    SEV);
    tr!(b'e', SEV,   SEVE);
    tr!(b'n', SEVE,  F7);
    tr!(b'i', SE,    EI);
    tr!(b'i', SEVE,  EI);
    // eight
    tr!(b'e', Start, E);
    tr!(b'i', E,     EI);
    tr!(b'g', EI,    EIG);
    tr!(b'h', EIG,   EIGH);
    tr!(b't', EIGH,  F8);
    // nine
    tr!(b'n', Start, N);
    tr!(b'i', N,     NI);
    tr!(b'n', NI,    NIN);
    tr!(b'e', NIN,   F9);
    tr!(b'i', NIN,   NI);

    val
};
