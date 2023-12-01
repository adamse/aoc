fn main() {
    let c = std::fs::read_to_string("inp.txt").unwrap();
    let mut sum = 0u64;
    for l in c.lines() {
        sum += 10 * l.chars().find(char::is_ascii_digit).unwrap().to_digit(10).unwrap() as u64;
        sum += l.chars().rev().find(char::is_ascii_digit).unwrap().to_digit(10).unwrap() as u64;
    }
    println!("{sum}");

    // let c = std::fs::read_to_string("inp2.txt").unwrap();

    let vals = std::collections::HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ]);
    let matches = vals.keys().copied().collect::<Vec<_>>();

    assert!(c.is_ascii());

    let mut sum = 0u64;
    for line in c.lines() {
        let mut tails = vec![];
        for (ii, _) in line.char_indices() {
            tails.push(&line[ii..]);
        }
        let a = tails.iter()
            .find_map(|&tail|
                matches.iter()
                    .find_map(|&y| tail.starts_with(y).then_some(vals[y]))).unwrap();
        let b = tails.iter().rev()
            .find_map(|&tail|
                matches.iter()
                    .find_map(|&y| tail.starts_with(y).then_some(vals[y]))).unwrap();
        let v = 10 * a + b;
        // println!("{v}");
        sum += v;
    }
    println!("{sum}");
}
