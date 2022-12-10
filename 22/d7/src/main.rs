use std::fs;
use std::io;
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let buf = fs::read_to_string("big")?;
    // let buf = fs::read_to_string("small")?;
    let buf = buf.trim();

    // keep track of the current directory
    let mut pwd: Vec<&str> = vec![];

    // keep track of directory sizes
    let mut fs: HashMap<Vec<&str>, u64> = HashMap::new();

    for line in buf.lines() {
        match line.split_once(" ") {
            Some(("$", s)) => {
                if let Some((_, dir)) = s.split_once("cd ") {
                    match dir {
                        ".." => { pwd.pop(); },
                        "/" => { },
                        dir => { pwd.push(dir); },
                    }
                }
            },
            Some(("dir", _)) => {},
            Some((size, _)) => {
                let size = size.parse::<u64>().unwrap();

                // add size to root dir
                let mut p = Vec::new();
                let dir = fs.entry(p.clone()).or_insert(0u64);
                *dir = *dir + size;

                // add size to parent dirs and self
                for &dir in &pwd {
                    p.push(dir);
                    let dir = fs.entry(p.clone()).or_insert(0u64);
                    *dir = *dir + size;
                }
            },
            None => {},
        }
    }

    let sz: u64 = fs.values()
        .filter(|&&size| size < 100000)
        .sum();
    println!("{sz}");

    // part 2
    let used = fs[&Vec::new()];
    let free = 70000000 - used;
    let to_delete = 30000000 - free;

    let choosen = fs
        .values()
        .filter(|&&v| v >= to_delete)
        .min().unwrap();
    println!("{choosen}");

    Ok(())
}
