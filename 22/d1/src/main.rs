use std::fs::File;
use std::io::{self, BufRead};
use std::cmp::Reverse;

// goal: get used to the rust standard library

fn main() -> io::Result<()> {
    // storage all elf calories
    let mut all_elfs = vec![];

    // parse the input
    let all_elfs = {
        let lines =
            io::BufReader::new(File::open("big")?).lines();

        let mut current_elf = 0i64;

        for line in lines {
            let line = line?;
            if line == "" {
                // empty line means we have a new elf
                all_elfs.push(current_elf);
                current_elf = 0;
            } else {
                let num = line.parse::<i64>();
                let num =
                    num.map_err(|_|
                        io::Error::new(io::ErrorKind::Other, "failed to parse"))?;
                current_elf += num;
            }
        }

        // add the last elf
        all_elfs.push(current_elf);

        // all_elfs.sort();
        &mut all_elfs[..]
    };


    let (_, maxelf, _) = all_elfs.select_nth_unstable_by_key(0, |x|Reverse(*x));
    println!("{maxelf}");

    let (top3, _, _) = all_elfs.select_nth_unstable_by_key(3, |x|Reverse(*x));
    let top3: i64 = top3.iter().sum();
    println!("{top3}");

    Ok(())
}
