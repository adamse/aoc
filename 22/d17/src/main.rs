use std::collections::hash_map::Entry;

#[derive(Clone, Copy)]
enum Dir {
    L, R, D,
}

type Pos = (i64, i64);
type Shape = [[u8; 4]; 4];

struct World {
    grid: Vec<[u8; 7]>,
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
    // try to move the shape in the given direction, returns new position if successful
    fn try_move(&self, shape: &Shape, pos: Pos, dir: Dir) -> Option<Pos> {
        // position is the bottom left of the shape

        let newpos = match dir {
            Dir::L => (pos.0 - 1, pos.1),
            Dir::R => (pos.0 + 1, pos.1),
            Dir::D => (pos.0, pos.1 - 1),
        };

        for (y, row) in shape.iter().enumerate() {
            for (x, &part) in row.iter().enumerate() {
                if part > 0 {
                    let x = newpos.0 + x as i64;
                    let y = newpos.1 + y as i64;
                    if x < 0 || x >= 7  || y < 0 {
                        // collided with wall, or bottom
                        return None;
                    }
                    if let Some(grid_row) = self.grid.get(y as usize) {
                        if grid_row[x as usize] > 0 {
                            // collided with something
                            return None;
                        }
                    }
                }
            }
        }

        Some(newpos)
    }

    // place the rock in the grid in its current position
    fn settle(&mut self, shape: &Shape, pos: Pos) {
        for (y, row) in shape.iter().enumerate() {
            for (x, &part) in row.iter().enumerate() {
                if part > 0 {
                    let x = pos.0 as usize + x;
                    let y = pos.1 as usize + y;

                    while self.grid.len() < y + 1 {
                        self.grid.push([0u8, 0, 0, 0, 0, 0, 0]);
                    }

                    if let Some(grid_row) = self.grid.get_mut(y) {
                        // put the part in the grid
                        grid_row[x] = part;
                    } else {
                        panic!("{x} {y}\n{:?}", self.grid);
                    }
                }
            }
        }
    }

    // let a new rock fall
    fn new_rock(&mut self, mut jet_idx: usize, rock_no: usize) -> usize {
        let shape = self.shapes[rock_no % self.shapes.len()];

        let mut pos = (2i64, self.grid.len() as i64 + 3);

        loop {
            let jet = self.jets[jet_idx % self.jets.len()];
            jet_idx += 1;

            // see if the jet will move us
            if let Some(newpos) = self.try_move(&shape, pos, jet) {
                pos = newpos;
            }

            // try to move down
            if let Some(newpos) = self.try_move(&shape, pos, Dir::D) {
                pos = newpos;
            } else {
                break;
            }
        }

        // we have failed to move down, place the shape in the grid
        self.settle(&shape, pos);

        jet_idx
    }

    // create "unique" fingerprint of the current board
    fn fingerprint(&self, rock_no: usize, jet_idx: usize) -> Fingerprint {
        // depths as seen from the top
        let depths = {
            let mut ds = [0; 7];

            for ii in 0..7 {
                let pos = self.grid.iter().rev()
                    .position(|row| row[ii] > 0)
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
    }

    fn print_grid(&self, limit: usize) {
        println!("");
        for (x, row) in self.grid.iter().rev().enumerate() {
            if x > limit { break; }
            print!("{:>5} ", self.grid.len() - x);
            for &p in row {
                if p == 0 {
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
// the contour of the top
// ....
    // ....
    // ....
    // ####
    let em = [[1u8, 1, 1, 1], [0,0,0,0,], [0,0,0,0,], [0,0,0,0,]];

    // ....
    // .#..
    // ###.
    // .#..
    let cross = [[0u8, 1, 0, 0], [1, 1, 1, 0], [0, 1, 0, 0], [0,0,0,0,]];

    // ....
    // ..#.
    // ..#.
    // ###.
    let ell = [[1u8, 1, 1, 0], [0,0,1,0,], [0,0,1,0,], [0,0,0,0,]];

    // #...
    // #...
    // #...
    // #...
    let ii = [[1u8,0,0,0], [1,0,0,0], [1,0,0,0], [1,0,0,0]];

    // ....
    // ....
    // ##..
    // ##..
    let sqr = [[1u8, 1,0,0], [1, 1,0,0], [0,0,0,0,], [0,0,0,0,]];
    let shapes = [em, cross, ell, ii, sqr];

    let jets = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
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
