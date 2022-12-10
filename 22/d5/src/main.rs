#![feature(try_blocks)]

use std::fs;
use std::io;

const FILE: &str = "big";
const NSTACKS: usize = 9;

type Stacks = [Vec<char>; NSTACKS];

///           11
/// 012345678901
///     [D]
/// [N] [C]
/// [Z] [M] [P]         columns with stacks:
///  1   2   3          1 5 9 15 ...
fn parse_stacks(str: &str) -> Stacks
    where
        Stacks: Default
{
    let mut stacks: Stacks = Default::default();

    for line in str.lines() {
        for (col, char) in line.chars().enumerate() {
            match char {
                // A-Z, we have a crate
                c if c.is_ascii_uppercase() => {
                    let stack_no = (col - 1) / 4;
                    // println!("{c} {stack_no}");
                    assert!(stack_no < NSTACKS);

                    stacks[stack_no].push(c);
                },
                // spaces, [, ], numbers
                _ => {},
            }
        }
    }

    for stack in &mut stacks {
        stack.reverse();
    }

    stacks
}

/// "move A from B to C"
fn parse_instruction(str: &str) -> Option<(usize, usize, usize)> {
    let mut pieces = str.split_whitespace();

    let no = {
        // "move"
        pieces.next()?;
        // number
        pieces.next()?.parse::<usize>().ok()?
    };

    let from = {
        // "from"
        pieces.next()?;
        // number
        pieces.next()?.parse::<usize>().ok()? - 1
    };

    let to = {
        // "to"
        pieces.next()?;
        // number
        pieces.next()?.parse::<usize>().ok()? - 1
    };

    Some((no, from, to))
}

fn execute_instruction(stacks: &mut Stacks, (no, from, to): (usize, usize, usize)) {
    let from = &mut stacks[from];
    let mut moved_stack = from.split_off(from.len() - no);

    moved_stack.reverse();

    stacks[to].append(&mut moved_stack);
}

fn execute_instruction2(stacks: &mut Stacks, (no, from, to): (usize, usize, usize)) {
    let from = &mut stacks[from];
    let mut moved_stack = from.split_off(from.len() - no);

    stacks[to].append(&mut moved_stack);
}

fn main() -> io::Result<()> {
    let buf = fs::read_to_string(FILE)?;
    let buf = buf.trim_end();

    if let Some((stacks, instructions)) = buf.split_once("\n\n") {
        let mut stacks = parse_stacks(stacks);
        let mut stacks2 = stacks.clone();
        // println!("{stacks:?}");

        for instruction in instructions.lines().map(parse_instruction) {
            let instruction = instruction.unwrap();
            execute_instruction(&mut stacks, instruction);
            execute_instruction2(&mut stacks2, instruction);
            // println!("{instruction:?} {stacks:?}");
        }

        // println!("{stacks:?}");

        for stack in &mut stacks {
            let top = stack.pop().unwrap();
            print!("{top}");
        }
        println!("");

        for stack in &mut stacks2 {
            let top = stack.pop().unwrap();
            print!("{top}");
        }
        println!("");

    }

    Ok(())
}
