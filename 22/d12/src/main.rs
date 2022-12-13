use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::cmp::Ordering;

#[derive(Copy, Clone, Eq, PartialEq)]
struct N(i32, (i32, i32));

impl Ord for N {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.cmp(&self.0).then_with(|| self.1.cmp(&other.1))

    }
}

impl PartialOrd for N {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

type C = (i32, i32);

fn minpath(grid: &[&[u8]], start: C, goal: C) -> Option<i32> {
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    let mut q = BinaryHeap::new();
    q.push(N(0, start));

    let mut visited = HashSet::new();

    while let Some(N(dist, node)) = q.pop() {
        if visited.contains(&node) {
            continue;
        }

        if node == goal {
            return Some(dist);
        }

        visited.insert(node);

        for (dr, dc) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let (r, c) = node;
            let (rn, cn) = (r + dr, c + dc);

            let current = grid[r as usize][c as usize];

            if (rn < 0) || (rn >= rows) || (cn < 0) || (cn >= cols) {
                continue;
            }

            let next = grid[rn as usize][cn as usize];
            if next as i32 - current as i32 > 1 {
                continue;
            }

            q.push(N(dist + 1, (rn, cn)));
        }
    }

    None
}

fn main() -> std::io::Result<()> {
    let buf = std::fs::read_to_string("big")?;
    let grid = buf.lines().map(|x| x.as_bytes()).collect::<Vec<_>>();

    let start = (0, 0);
    let goal = (2, 5);

    let start = (20, 0);
    let goal = (20, 136);

    let a = minpath(&grid, start, goal).unwrap();
    println!("{a}");

    let mut paths = Vec::new();

    for (r, &row) in grid.iter().enumerate() {
        for (c, &node) in row.iter().enumerate() {
            if node == b'a' {
                paths.push(minpath(&grid, (r as i32, c as i32), goal));
            }
        }
    }
    let &best = paths.iter().flatten().min().unwrap();
    println!("{best}");

    Ok(())
}
