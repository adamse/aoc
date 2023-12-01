use std::collections::hash_map::Entry;
use std::collections::HashMap;

#[derive(Clone, Copy)]
enum Dir {
    L, R,
}

// u8
// 76543210
//  .......
//  .......
// this is a line on the grid
// we reserve bit 7, to shift into when moving left or right

type Shape = [u8; 4];

struct World {
    grid: Vec<u8>,
    top: usize,
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
    fn grid_collide(&self, shape: u32, y: usize) -> bool {
        // compare the whole
        let grid = self.grid[y..y+4].try_into().unwrap();
        let grid = u32::from_be_bytes(grid);

        grid & shape > 0
    }

    // returns the shape as it was moved by the jet
    fn jet_move(&self, shape: Shape, y: usize, dir: Dir) -> Option<Shape> {
        let shape = u32::from_be_bytes(shape);
        let shape = match dir {
            Dir::L => shape >> 1,
            Dir::R => shape.rotate_left(1),
        };

        const MASK: u32 = u32::from_be_bytes([1 << 7; 4]);

        if shape & MASK > 0 {
            return None;
        }

        if self.grid_collide(shape, y) {
            return None;
        }

        Some(shape.to_be_bytes())
    }

    // returns the new y level if move was successful
    fn down_move(&self, shape: Shape, y: usize) -> Option<usize> {
        if y == 0 || self.grid_collide(u32::from_be_bytes(shape), y - 1) {
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

                // update top row
                self.top = self.top.max(y + 1);

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
        let mut y = self.top + 3;

        // make space for us
        while self.grid.len() < y + 4 {
            self.grid.push(0u8);
        }

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

            let diff = self.grid.len() - self.top;

            for ii in 1..7 {
                let bit = 1u8 << ii;
                let depth = self.grid.iter().rev()
                    .position(|&row| row & bit != 0)
                    .map(|x| x - diff)
                    .unwrap_or(self.top);
                ds[ii] = depth;
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

        let mut cycle_detector = HashMap::new();

        let mut jet_idx = 0usize;
        let mut rock_no = 0usize;


        let (cycle_jets, cycle_rocks, cycle_height) = loop {
            jet_idx = self.new_rock(jet_idx, rock_no);

            rock_no += 1;

            // check if we have a cycle

            let fp = self.fingerprint(rock_no, jet_idx);

            // <= 3 to make weird cave-y shapes at the top unlikely, not sure if needed
            if fp.depths.iter().all(|x| *x <= 3) {

                let cycle = cycle_detector.entry(fp);

                if let Entry::Occupied(v) = cycle {

                    let (prev_jet_idx, prev_rock_no, prev_grid_len) = v.get();

                    let cycle_jets = jet_idx - *prev_jet_idx;
                    let cycle_rocks = rock_no - *prev_rock_no;
                    let cycle_height = self.top - *prev_grid_len;

                    break (cycle_jets, cycle_rocks, cycle_height);
                } else {
                    cycle.or_insert((jet_idx, rock_no, self.top));
                }
            }


        };

        // part 1
        {
            const MAX: usize = 2022;

            let grid = self.grid.clone();
            let top = self.top;

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

            assert!(3119 == self.top + total_cycle_height);
            // println!("{}", self.top + total_cycle_height);

            // reset
            self.grid = grid;
            self.top = top;
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

        assert!(1536994219669 == self.top + total_cycle_height);
        // println!("{}", self.top + total_cycle_height);
        // println!("{}", cycle_rocks);

    }

    #[allow(dead_code)]
    fn print_grid(&self, limit: usize) {
        println!("");
        for (x, &row) in self.grid.iter().rev().enumerate() {
            if x > limit { break; }
            print!("{:>5} ", self.grid.len() - x);
            for ii in 0..7 {
                if row & (1u8 << ii) == 0 {
                    print!(".");
                } else {
                    print!("█");
                }
            }
            println!("");
        }
    }

}

pub fn main() {
    // ........
    // ........
    // ........
    // ..####..
    let em = [0b00111100u8, 0, 0, 0];

    // ........
    // ...#....
    // ..###...
    // ...#....
    let cross = [0b00001000u8, 0b00011100, 0b00001000, 0];

    // ........
    // ....#...
    // ....#...
    // ..###...
    let ell = [0b00011100u8, 0b00010000, 0b00010000, 0];

    // ..#.....
    // ..#.....
    // ..#.....
    // ..#.....
    let ii = [0b00000100u8, 0b00000100, 0b00000100, 0b00000100];

    // ........
    // ........
    // ..##....
    // ..##....
    let sqr = [0b00001100u8, 0b00001100, 0, 0];

    let shapes = [em, cross, ell, ii, sqr];

    #[allow(unused_variables)]
    let jets = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
    let jets = std::fs::read_to_string("big").unwrap();
    let jets = jets.trim().as_bytes();

    let jets: Vec<_> = jets.iter().map(|x| match x {
        b'<' => Dir::L,
        b'>' => Dir::R,
        _ => panic!("{x}"),
    }).collect();


    for _ in 0..100 {
        let start = std::time::Instant::now();
        let mut w = World {
            grid: vec![],
            top: 0,
            shapes,
            jets: jets.clone(),
        };
        w.main();
        println!("micros: {}", start.elapsed().as_micros());
    }
}
