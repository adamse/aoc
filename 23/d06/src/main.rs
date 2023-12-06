fn parse(i: &str) -> (Vec<u32>, Vec<u32>) {
    let mut it = i.lines();
    (it.next().unwrap().split_whitespace().filter_map(|x|x.parse().ok()).collect(),
        it.next().unwrap().split_whitespace().filter_map(|x|x.parse().ok()).collect())
}
fn p1(times: &[u32], dists: &[u32]) -> u64 {
    let mut possible_wins = 1;
    for (&time, &dist) in times.iter().zip(dists) {
        let mut wins = 0;
        for hold_time in 0..=time {
            let run_time = time - hold_time;
            let run_dist = run_time * hold_time;
            wins += (run_dist > dist) as u32;
        }
        if wins >= 1 {
            possible_wins *= wins as u64;
        }
    }
    possible_wins
}
fn parse2(i: &str) -> (u64, u64) {
    let mut it = i.lines();
    (it.next().unwrap().chars().filter(|x| x.is_ascii_digit()).collect::<String>().parse().unwrap(),
        it.next().unwrap().chars().filter(|x| x.is_ascii_digit()).collect::<String>().parse().unwrap())
}
fn parse2_fold(i: &str) -> (u64, u64) {
    let mut it = i.lines();
    (it.next().unwrap().bytes().fold(0, |acc, e| if e.is_ascii_digit() { acc * 10 + (e - b'0') as u64 } else { acc }),
        it.next().unwrap().bytes().fold(0, |acc, e| if e.is_ascii_digit() { acc * 10 + (e - b'0') as u64 } else { acc }))
}
fn p2(time: u64, dist: u64) -> u64 {
    let mut wins = 0;
    for hold_time in 0..=time {
        let run_time = time - hold_time;
        let run_dist = run_time * hold_time;
        wins += (run_dist > dist) as u64;
    }
    wins
}
fn p2_bisect(time: u64, dist: u64) -> u64 {
    let win = move |hold_time| {
        let run_time = time - hold_time;
        run_time * hold_time > dist
    };
    // find a win,
    let mid = time / 2;
    assert!(win(mid));

    // bisection to the left to find the root
    let mut a = 0;
    let mut b = mid;
    let low = loop {
        if b - a == 1 {
            break b;
        }

        let c = a + (b - a) / 2;
        if win(c) {
            b = c;
        } else {
            a = c;
        }
    };

    // bisection to the right to find the root
    let mut a = mid;
    let mut b = time;
    let hi = loop {
        if b - a == 1 {
            break b;
        }
        let c = a + (b - a) / 2;
        if win(c) {
            a = c;
        } else {
            b = c;
        }
    };

    hi - low
}
fn main() {
    let i = std::fs::read_to_string("inp.txt").unwrap();
    // let i = std::fs::read_to_string("inp1.txt").unwrap();
    let i = i.trim();

    let (times, dists) = parse(i);
    let times = &times[..];
    let dists = &dists[..];
    let st = std::time::Instant::now();
    println!("{}", p1(times, dists));
    eprintln!("{}us", st.elapsed().as_micros());

    let (times, dists) = parse2(i);
    let st = std::time::Instant::now();
    println!("{}", p2(times, dists));
    eprintln!("{}us", st.elapsed().as_micros());
    let st = std::time::Instant::now();
    println!("{}", p2_bisect(times, dists));
    eprintln!("{}ns", st.elapsed().as_nanos());
}
