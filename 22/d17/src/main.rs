use std::collections::hash_map::Entry;

#[derive(Clone, Copy)]
enum Dir {
    L, R,
}

// u8
// 76543210
// .......
// .......
// this is a line on the grid
// we reserve bit 0, to shift into when moving left or right

type Shape = [u8; 4];

struct World {
    grid: Vec<u8>,
    shapes: [Shape; 5],
    jets: Vec<Dir>,
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct Fingerprint {
    depths: [usize; 7],
    jet: usize,
    rock: usize,
}

impl World {
    fn grid_collide(&self, shape: Shape, y: usize) -> bool {

        for (dy, &rock_row) in shape.iter().enumerate() {
            let y = y + dy;
            if let Some(grid_row) = self.grid.get(y) {
                // the grid is populated at this level
                if rock_row & grid_row != 0 {
                    // we hit something
                    return true;
                }
            }
        }

        false
    }

    // returns the shape as it was moved by the jet
    fn jet_move(&self, shape: Shape, y: usize, dir: Dir) -> Option<Shape> {
        // move the shape
        let shape = shape.map(|row| match dir {
            // rotate left so the high bit ends up in the first bit if set, indicating that we
            // hit the wall
            Dir::L => row.rotate_left(1),
            // shift right so the rightmost part ends up in the first bit if set, indicating that
            // we hit the wall
            Dir::R => row >> 1,
        });

        if shape.iter().any(|&row| row & 1 != 0) {
            // we hit a wall
            return None;
        }

        if self.grid_collide(shape, y) {
            return None;
        }

        // otherwise return the new shape
        Some(shape)
    }

    // returns the new y level if move was successful
    fn down_move(&self, shape: Shape, y: usize) -> Option<usize> {
        if y == 0 || self.grid_collide(shape, y - 1) {
            // we hit the bottom or we hit a previous rock
            return None;
        }

        Some(y - 1)
    }

    // place the rock in the grid in its current position
    fn settle(&mut self, shape: Shape, y: usize) {
        for (dy, &rock_row) in shape.iter().enumerate() {
            if rock_row > 0 {
                let y = y + dy;

                while self.grid.len() < y + 1 {
                    self.grid.push(0u8);
                }

                if let Some(grid_row) = self.grid.get_mut(y) {
                    // fix the rock row to the grid
                    *grid_row |= rock_row;
                } else {
                    panic!("{y}\n{:?}", self.grid);
                }
            }
        }
    }

    // let a new rock fall
    fn new_rock(&mut self, mut jet_idx: usize, rock_no: usize) -> usize {

        let mut shape = self.shapes[rock_no % self.shapes.len()];
        let mut y = self.grid.len() + 3;


        loop {
            let jet = self.jets[jet_idx % self.jets.len()];
            jet_idx += 1;

            // see if the jet will move us
            if let Some(newshape) = self.jet_move(shape, y, jet) {
                shape = newshape;
            }

            // try to move down
            if let Some(newy) = self.down_move(shape, y) {
                y = newy;
            } else {
                break;
            }
        }

        // we have failed to move down, place the shape in the grid
        self.settle(shape, y);

        jet_idx
    }

    // create "unique" fingerprint of the current board
    fn fingerprint(&self, rock_no: usize, jet_idx: usize) -> Fingerprint {
        // depths as seen from the top
        let depths = {
            let mut ds = [0; 7];

            for ii in 0..7 {
                let bit = 1u8 << (7 - ii);
                let pos = self.grid.iter().rev()
                    .position(|&row| row & bit != 0)
                    .unwrap_or(self.grid.len());
                ds[ii] = pos;
            }

            ds
        };

        Fingerprint {
            depths,
            rock: rock_no % self.shapes.len(),
            jet: jet_idx % self.jets.len(),
        }
    }

    fn main(&mut self) {
        let start = std::time::Instant::now();

        let mut cycle_detector = std::collections::HashMap::new();

        let mut jet_idx = 0usize;
        let mut rock_no = 0usize;


        let (cycle_jets, cycle_rocks, cycle_height) = loop {

            // check if we have a cycle

            let fp = self.fingerprint(rock_no, jet_idx);

            // <= 3 to make weird cave-y shapes at the top unlikely, not sure if needed
            if fp.depths.iter().all(|x| *x <= 3) {

                let cycle = cycle_detector.entry(fp);

                if let Entry::Occupied(v) = cycle {

                    let (prev_jet_idx, prev_rock_no, prev_grid_len) = v.get();

                    let cycle_jets = jet_idx - *prev_jet_idx;
                    let cycle_rocks = rock_no - *prev_rock_no;
                    let cycle_height = self.grid.len() - *prev_grid_len;

                    break (cycle_jets, cycle_rocks, cycle_height);
                } else {
                    cycle.or_insert((jet_idx, rock_no, self.grid.len()));
                }
            }

            jet_idx = self.new_rock(jet_idx, rock_no);

            rock_no += 1;

        };

        // part 1
        {
            const MAX: usize = 2022;

            let grid = self.grid.clone();

            let mut jet_idx = jet_idx;
            let mut rock_no = rock_no;

            let rocks_remaining = MAX - rock_no;

            let total_cycle_repeats = rocks_remaining / cycle_rocks;
            let total_cycle_height = cycle_height * total_cycle_repeats;

            jet_idx += total_cycle_repeats * cycle_jets;
            rock_no += total_cycle_repeats * cycle_rocks;

            // do the last few rocks
            while rock_no < MAX {
                jet_idx = self.new_rock(jet_idx, rock_no);

                rock_no += 1;
            }
            println!("{}", self.grid.len() + total_cycle_height);

            // reset
            self.grid = grid;
        }

        // part 2

        const MAX: usize = 1_000_000_000_000;

        let rocks_remaining = MAX - rock_no;

        let total_cycle_repeats = rocks_remaining / cycle_rocks;
        let total_cycle_height = cycle_height * total_cycle_repeats;

        jet_idx += total_cycle_repeats * cycle_jets;
        rock_no += total_cycle_repeats * cycle_rocks;

        // do the last few rocks
        while rock_no < MAX {
            jet_idx = self.new_rock(jet_idx, rock_no);

            rock_no += 1;
        }

        println!("{}", self.grid.len() + total_cycle_height);
        // println!("{}", cycle_rocks);

        println!("micros: {}", start.elapsed().as_micros());
    }

    #[allow(dead_code)]
    fn print_grid(&self, limit: usize) {
        println!("");
        for (x, &row) in self.grid.iter().rev().enumerate() {
            if x > limit { break; }
            print!("{:>5} ", self.grid.len() - x);
            for ii in (1..8).rev() {
                if row & (1u8 << ii) == 0 {
                    print!(".");
                    // print!(" ");
                } else {
                    // print!("#");
                    print!("█");
                }
            }
            println!("");
        }
    }

}

pub fn main() {
    // ....
    // ....
    // ....
    // ####
    // let em = [[1u8, 1, 1, 1], [0,0,0,0,], [0,0,0,0,], [0,0,0,0,]];
    let em = [0b00111100u8, 0, 0, 0];

    // ....
    // .#..
    // ###.
    // .#..
    // let cross = [[0u8, 1, 0, 0], [1, 1, 1, 0], [0, 1, 0, 0], [0,0,0,0,]];
    let cross = [0b00010000u8, 0b00111000, 0b00010000, 0];

    // ....
    // ..#.
    // ..#.
    // ###.
    // let ell = [[1u8, 1, 1, 0], [0,0,1,0,], [0,0,1,0,], [0,0,0,0,]];
    let ell = [0b00111000u8, 0b00001000, 0b00001000, 0];

    // #...
    // #...
    // #...
    // #...
    // let ii = [[1u8,0,0,0], [1,0,0,0], [1,0,0,0], [1,0,0,0]];
    let ii = [0b00100000u8, 0b00100000, 0b00100000, 0b00100000];

    // ....
    // ....
    // ##..
    // ##..
    // let sqr = [[1u8, 1,0,0], [1, 1,0,0], [0,0,0,0,], [0,0,0,0,]];
    let sqr = [0b00110000u8, 0b00110000, 0, 0];

    let shapes = [em, cross, ell, ii, sqr];

    // let jets = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
    let jets = std::fs::read_to_string("big").unwrap();
    let jets = jets.trim().as_bytes();

    let jets = jets.iter().map(|x| match x {
        b'<' => Dir::L,
        b'>' => Dir::R,
        _ => panic!("{x}"),
    }).collect();

    let mut w = World {
        grid: vec![],
        shapes,
        jets,
    };

    w.main();
}
