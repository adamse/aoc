#![feature(slice_group_by)]

mod small;
mod big;

struct RangeSet {
    min: i32,
    max: i32,
    // ranges: [(i32, i32); N],
    ranges: Vec<(i32, i32)>,
}

impl RangeSet {
    fn insert(&mut self, start: i32, end: i32) {
        if end < self.min || start > self.max {
            // ended before min clamp, or started after max clamp
            return;
        }

        match self.ranges.binary_search(&(start, end)) {
            Ok(_) => {},
            Err(idx) => { self.ranges.insert(idx, (start.max(self.min), end.min(self.max))); },
        }
    }

    fn finalize(&mut self) {
        let mut ranges = vec![];

        let last = self.ranges[1..].iter()
            .fold(self.ranges[0], |(a, b), &(c, d)| {
                if b + 1 >= c {
                    (a, b.max(d))
                } else {
                    ranges.push((a, b));
                    (c, d)
                }
            });

        ranges.push(last);
        self.ranges = ranges;
    }

    fn free(&self) -> Option<(i32, i32)> {
        None
    }
}

fn find_free(segments: &[(i32, i32)], min: i32, max: i32) -> Option<i32> {
    //  -----      min..max
    // ---- ----   segments

    for &(a, b) in segments {
        // hole at the start?
        //  ---- min..max
        //   --- a..b
        if a > min { return Some(a-1) };
        // hole at the end?
        //  ---- min..max
        //  ---  a..b
        if b < max { return Some(b+1) };
    }

    None
}

fn segments_at_y<const N: usize>(data: &[[i32; 4]; N], y: i32) -> Vec<(i32, i32)> {
    let mut segments = vec![];

    for [sx, sy, bx, by] in data {
        // if (sx, sy) != (8,7) { continue; }

        // if line y=2000000 is in this square
        let diag = (sx - bx).abs() + (sy - by).abs();
        if sy - diag <= y && y <= sy + diag {
            let dy = (y - sy).abs();
            let dx = diag - dy;

            let xmin = sx - dx;
            let xmax = sx + dx;

            let segment = (xmin, xmax);
            match segments.binary_search(&segment) {
                Ok(_) => {},
                Err(idx) => { segments.insert(idx, segment); },
            }
        }
    }

    let mut segments2 = vec![];

    let last = segments[1..].iter()
        .fold(segments[0], |(a, b), &(c, d)| {
            if b + 1 >= c {
                (a, b.max(d))
            } else {
                segments2.push((a, b));
                (c, d)
            }
        });

    segments2.push(last);

    segments2
}

fn part2<const N: usize>(data: &[[i32; 4]; N], max: i32) -> u64 {
    let (y, x) = (0..=max).map(|y| (y, segments_at_y(data, y)))
        .map(|(y, segs)| (y, find_free(&segs[..], 0, max)))
        .find_map(|(y, free)| free.map(|x| (y, x)))
        .unwrap();

    4000000 * x as u64 + y as u64
}

fn main() {
    // let end = segments_at_y(&small::SMALL, 10);
    let end = segments_at_y(&big::BIG, 2000000);
    let res: i32 = end.iter()
        .map(|seg| seg.1 - seg.0)
        .sum();

    println!("{res}");

    // let res = part2(&small::SMALL, 20);
    let res = part2(&big::BIG, 4000000);

    println!("{res:?}");

}
