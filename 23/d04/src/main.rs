#![feature(byte_slice_trim_ascii)]
#![feature(slice_partition_dedup)]
fn parse(i: &str) -> Vec<u32> {
    i.lines()
        .map(|l|
             l.split_whitespace()
                .filter_map(|x| x.parse::<u32>().ok())
                .collect::<Vec<_>>())
        .map(|mut card| {
            card.sort();
            let (_, dups) = card.partition_dedup();
            dups.len() as u32
        })
        .collect()
}
fn p1(i: &[u32]) -> u32 {
    i.iter()
        .map(|&c| if c > 0 { 1 << (c - 1) } else { 0 })
        .sum()
}
fn p2(cards: &[u32]) -> u32 {
    let mut counts = vec![1; cards.len()];
    for (i, &wins) in cards.iter().enumerate() {
        let count = counts[i];
        for di in 1..=wins {
            counts[i+di as usize] += count;
        }
    }
    counts.iter().sum()
}
fn main() {
    let i = std::fs::read_to_string("inp.txt").unwrap();
    // let i = std::fs::read_to_string("inp1.txt").unwrap();
    // let i = std::fs::read_to_string("inp2.txt").unwrap();
    let i = i.trim();

    let st = std::time::Instant::now();
    let i = parse(i);
    eprintln!("{}us", st.elapsed().as_micros());
    let st = std::time::Instant::now();
    println!("{}", p1(&i[..]));
    eprintln!("{}ns", st.elapsed().as_nanos());
    let st = std::time::Instant::now();
    println!("{}", p2(&i[..]));
    eprintln!("{}ns", st.elapsed().as_nanos());
}
