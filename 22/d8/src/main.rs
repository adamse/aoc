#![feature(anonymous_lifetime_in_impl_trait)]

use std::fs;
use std::io;
use std::collections::HashSet;

fn parse(c: u8) -> Option<i32> {
    if c < b'0' || c > b'9' {
        None
    } else {
        Some((c - b'0') as i32)
    }
}

// const COLS: usize = 5usize;
// const ROWS: usize = 5usize;
// const FILE: &str = "small";
const COLS: usize = 99usize;
const ROWS: usize = 99usize;
const FILE: &str = "big";

fn count(it: impl Iterator<Item = (usize, i32)>, visible: &mut HashSet<(usize, usize)>) {

    let mut tallest_row = [-1i32; ROWS];
    let mut tallest_col = [-1i32; COLS];

    for (ii, tree) in it {

        let (row, col) = (ii / COLS, ii % COLS);

        // must only count a tree once

        if tree > tallest_col[col] {
            tallest_col[col] = tree;
            visible.insert((row, col));
        }

        if tree > tallest_row[row] {
            tallest_row[row] = tree;
            visible.insert((row, col));
        }
    }
}

fn main() -> io::Result<()> {
    let buf = fs::read_to_string(FILE)?;
    let buf = buf.trim().as_bytes();
    let buf: Vec<i32> = buf.iter().copied()
        .filter(|c| c.is_ascii_digit())
        .map(|t| parse(t).unwrap())
        .collect();
    let forest = &buf[..];

    // 2 passes:
    // 1. count and keep track of the tallest tree to the top and to the left
    // 2. count and keep track of the tallest tree to the botto and to the right

    let mut visible: HashSet<(usize, usize)> = HashSet::new();


    {
        let it =
            forest.iter().copied().enumerate();
        count(it, &mut visible);
    }
    {
        let it =
            forest.iter().copied().enumerate().rev();
        count(it, &mut visible);
    }

    let visible = visible.len();
    println!("part1: {visible}");

    fn idx(row: usize, col: usize) -> usize {
        row * ROWS + col
    }

    let mut best = 0u64;
    for (ii, me) in forest.iter().copied().enumerate() {
        let (row, col) = (ii / COLS, ii % COLS);

        // examples
        // if !((row, col) == (1,2) || (row, col) == (3, 2)) { continue; }
        // println!("{row} {col}");

        let mut score = 1u64;

        // check left
        let mut visible = 0u64;
        for col in (0..col).rev() {
            // stop if you reach an edge or at the first tree that is the same height or taller
            // than the tree under consideration

            // we can definitely see this tree
            visible += 1;

            // check if we can see further
            if me <= forest[idx(row, col)] {
                break;
            }
        }
        // println!("{visible}");
        score *= visible;

        // check right
        let mut visible = 0u64;
        for col in col+1..COLS {
            visible += 1;

            if me <= forest[idx(row, col)] {
                break;
            }
        }
        // println!("{visible}");
        score *= visible;

        // check up
        let mut visible = 0u64;
        for row in (0..row).rev() {
            visible += 1;

            if me <= forest[idx(row, col)] {
                break;
            }
        }
        // println!("{visible}");
        score *= visible;

        // check down
        let mut visible = 0u64;
        for row in row+1..ROWS {
            visible += 1;

            if me <= forest[idx(row, col)] {
                break;
            }
        }
        // println!("{visible}");
        score *= visible;

        // println!("{score}");
        best = best.max(score);
    }
    println!("part2: {best}");

    Ok(())
}
