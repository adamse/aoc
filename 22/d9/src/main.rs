use std::fs;
use std::io;

use std::collections::HashSet;

#[derive(Debug, Copy, Clone)]
enum Dir {
    R,
    L,
    U,
    D,
}

struct Move {
    dir: Dir,
    count: i32,
}

fn parse(mov: &str) -> Option<Move> {
    let (dir, count) = mov.split_once(" ")?;

    use Dir::*;

    let dir = match dir {
        "R" => R,
        "L" => L,
        "U" => U,
        "D" => D,
        _ => { return None; },
    };

    let count = count.parse::<i32>().ok()?;

    Some(Move{ dir, count })
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Pos {
    x: i64,
    y: i64,
}

fn do_move(p: Pos, dir: Dir) -> Pos {
    match dir {
        Dir::U => Pos { x: p.x,     y: p.y + 1 },
        Dir::D => Pos { x: p.x,     y: p.y - 1 },
        Dir::R => Pos { x: p.x + 1, y: p.y },
        Dir::L => Pos { x: p.x - 1, y: p.y },
    }
}

fn adjacent(h: Pos, t: Pos) -> bool {
    (h.x - t.x).abs() <= 1 &&
        (h.y -t.y).abs() <= 1
}

fn follow(head: Pos, tail: Pos) -> Pos {
    if adjacent(head, tail) {
        return tail;
    }

    Pos {
        x: tail.x + (head.x - tail.x).signum(),
        y: tail.y + (head.y - tail.y).signum(),
    }
}


fn main() -> io::Result<()> {
    let buf = fs::read_to_string("big")?;
    let buf = buf.trim();

    let mut head = Pos { x: 0i64, y: 0i64 };
    let mut tail = head;

    let mut visited = HashSet::new();
    visited.insert(tail);

    // println!("{},{} {},{}", head.0, head.1, tail.0, tail.1);

    for mov in buf.lines().map(parse) {
        let Move {dir, count} = mov.unwrap();
        // println!("{dir:?} {count}");
        for _ in 0..count {
            head = do_move(head, dir);
            tail = follow(head, tail);
            visited.insert(tail);
            // println!("{},{} {},{}", head.0, head.1, tail.0, tail.1);
        }
    }

    let count = visited.len();
    println!("part1: {count}");

    let mut rope = [Pos { x: 0, y: 0 }; 10];

    let mut visited = HashSet::new();
    visited.insert(rope[9]);

    for mov in buf.lines().map(parse) {
        let Move {dir, count} = mov.unwrap();
        // println!("{dir:?} {count}");
        for _ in 0..count {
            rope[0] = do_move(rope[0], dir);

            for ii in 0..9 {
                rope[ii+1] = follow(rope[ii], rope[ii+1]);
            }

            visited.insert(rope[9]);
        }
    }

    let count = visited.len();
    println!("part2: {count}");

    Ok(())
}
