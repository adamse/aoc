fn diff(i: &Vec<i32>) -> Vec<i32> {
    i.windows(2).map(|w| w[1] - w[0]).collect()
}
fn diff_mut(i: &mut Vec<i32>) {
    for ix in 0..i.len()-1 {
        i[ix] = i[ix+1] - i[ix];
    }
    i.pop();
}
fn solve(i: Vec<Vec<i32>>) -> (i32, i32) {
    let mut total = 0;
    let mut total2 = 0;

    let mut last = Vec::with_capacity(i[0].len());
    let mut first = Vec::with_capacity(i[0].len());

    for mut series in i {
        last.clear();
        last.push(series[series.len() - 1]);

        first.clear();
        first.push(series[0]);

        while !series.iter().all(|x| 0.eq(x)) {
            diff_mut(&mut series);
            last.push(series[series.len() - 1]);
            first.push(series[0]);
        }

        let mut c = 0;
        for &l in first.iter().rev() {
            c = l - c;
        }

        total += last.iter().sum::<i32>();
        total2 += c;
    }
    (total, total2)
}
fn main() {
    let i = std::fs::read_to_string("inp.txt").unwrap();
    // let i = std::fs::read_to_string("inp1.txt").unwrap();
    let i = i.trim();

    let st = std::time::Instant::now();
    let i = i
        .lines()
        .map(|line|
            line.split_whitespace()
                .filter_map(|x| x.parse::<i32>().ok()).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    eprintln!("{}us", st.elapsed().as_micros());

    let st = std::time::Instant::now();
    let (p1, p2) = solve(i);
    eprintln!("{}us", st.elapsed().as_micros());
    println!("{}", p1);
    println!("{}", p2);
}
