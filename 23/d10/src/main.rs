#![feature(byte_slice_trim_ascii)]
struct Input<'a> {
    grid: &'a [u8],
    start: (i32, i32),
    rows: i32,
    cols: i32,
    pipe: std::collections::HashSet<(i32,i32)>,
}
impl<'a> Input<'a> {
    fn new(i: &'a mut [u8], s: u8) -> Self {
        let cols = memchr::memchr(b'\n', i).unwrap() as i32;
        let rows = i.len() as i32 / cols;

        let start = memchr::memchr(b'S', i).unwrap() as i32;
        i[start as usize] = s;
        let start = (start / cols, start % (cols + 1));

        Input {
            grid: i,
            start,
            rows,
            cols,
            pipe: std::collections::HashSet::new(),
        }
    }
    fn get(&self, (r, c): (i32, i32)) -> Option<u8> {
        if 0 > r || r >= self.rows || 0 > c || c >= self.cols { return None; }
        let ix = (r * (self.cols + 1) + c) as usize;
        Some(self.grid[ix])
    }
    fn connections(&self, s: (i32,i32)) -> ((i32, i32), (i32, i32)) {
        let u = (-1,0);
        let d = (1,0);
        let l = (0,-1);
        let r = (0, 1);

        let me = self.get(s).unwrap();

        match me {
            b'|' => (add(s, u), add(s, d)),
            b'-' => (add(s, l), add(s, r)),
            b'L' => (add(s, u), add(s, r)),
            b'J' => (add(s, u), add(s, l)),
            b'7' => (add(s, d), add(s, l)),
            b'F' => (add(s, d), add(s, r)),
            _ => panic!("{me}"),
        }
    }
}
fn add(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
    (a.0 + b.0, a.1 + b.1)
}
fn p1(i: &mut Input) -> u32 {
    let mut previous = i.start;
    let mut current = i.connections(i.start).0;
    i.pipe.insert(i.start);
    let mut steps = 1;
    while current != i.start {
        i.pipe.insert(current);
        let (c1, c2) = i.connections(current);
        let tmp = previous;
        previous = current;
        current = if tmp == c1 { c2 } else { c1 };
        steps += 1;
    }
    steps / 2
}
fn p2(i: &Input) -> u32 {
    let mut area = 0;

    for r in 0..i.rows {
        let mut inside = false;
        for c in 0..i.cols {
            let tile = i.get((r, c)).unwrap();
            let ispipe = i.pipe.contains(&(r, c));
            if ispipe && matches!(tile, b'|' | b'J' | b'L') {
                inside = !inside;
            }
            if !ispipe && inside {
                area += 1;
            }
        }

    }

    area
}
fn main() {
    let mut i = std::fs::read("inp.txt").unwrap(); let s = b'|';
    // let mut i = std::fs::read("inp1.txt").unwrap(); let s = b'F';
    // let mut i = std::fs::read("inp2.txt").unwrap(); let s = b'F';
    // let mut i = std::fs::read("inp3.txt").unwrap(); let s = b'F';
    // let mut i = std::fs::read("inp4.txt").unwrap(); let s = b'F';
    // let mut i = std::fs::read("inp5.txt").unwrap(); let s = b'7';
    let x = i.trim_ascii();
    let x = x.len();
    let i = &mut i[..x];


    let st = std::time::Instant::now();
    let mut i = Input::new(i, s);
    eprintln!("{}us", st.elapsed().as_micros());
    println!("{}", p1(&mut i));
    eprintln!("{}us", st.elapsed().as_micros());
    println!("{}", p2(&i));
    eprintln!("{}us", st.elapsed().as_micros());
}
