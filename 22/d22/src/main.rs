#![feature(slice_group_by)]

use std::collections::HashMap;

const SMALL: bool = false;

#[derive(Debug, Clone, Copy)]
enum Tile {
    Empty,
    Wall,
    Floor,
}

type Pos = (usize, usize);

#[derive(Debug)]
struct Map {
    grid: Vec<Vec<Tile>>,

    /// maps edges to the next face of the cube
    facebounds: HashMap<Pos, (Pos, D)>,
}

impl Map {
    fn parse(inp: &str) -> (Self, &str) {
        let mut grid = vec![];
        for line in inp.lines() {
            // empty line means we have parsed the map
            if line == "" {
                break;
            }
            let mut row = vec![Tile::Empty];
            for c in line.chars() {
                let tile = match c {
                    '#' => Tile::Wall,
                    '.' => Tile::Floor,
                    // ' '
                    _ => Tile::Empty,
                };
                row.push(tile);
            }
            row.push(Tile::Empty);
            grid.push(row);
        }

        let longest = grid.iter()
            .fold(0, |longest, current| longest.max(current.len()));

        // put a guard of empty tiles at the top and bottom
        let guard = vec![Tile::Empty; longest];
        grid.insert(0, guard.clone());
        grid.push(guard.clone());

        // extend each row to be the same length
        for row in &mut grid {
            while row.len() < longest {
                row.push(Tile::Empty);
            }
        }

        let mut facebounds = HashMap::new();
        if SMALL {
            /*
            // top face to bottom
            for col in 9..=12 {
                let row = 1;
                facebounds.insert((row, col), (13, col));
            }
            */
        } else {
            let col = 50;
            for row in 1..=50 {
                facebounds.insert((col, row), ((1, 100 + row), D::E));
            }

            let row = 0;
            for col in 51..=100 {
                facebounds.insert((col, row), ((1, 100 + row), D::E));
            }

        }

        let map = Map {
            grid,
            facebounds,
        };

        let instr = inp.split("\n\n").nth(1).unwrap();

        (map, instr)
    }

    fn print(&self) {
        for row in &self.grid {
            for tile in row {
                let c = match tile {
                    Tile::Empty => '_',
                    Tile::Wall => '#',
                    Tile::Floor => '.',
                };
                print!("{c}");
            }
            println!("");
        }
    }

    fn print_path(&self, path: &HashMap<Pos, D>) {
        for (row, line) in (&self.grid).iter().enumerate() {
            for (col, tile) in line.iter().enumerate() {
                if let Some(facing) = path.get(&(row, col)) {
                    print!("{}", match facing {
                        D::N => '^',
                        D::E => '>',
                        D::S => 'v',
                        D::W => '<',
                    });
                } else {
                    let c = match tile {
                        Tile::Empty => '_',
                        Tile::Wall => '#',
                        Tile::Floor => '.',
                    };
                    print!("{c}");
                }
            }
            println!("");
        }
    }

    fn start(&self) -> Pos {
        for (row, line) in self.grid.iter().enumerate() {
            for (col, tile) in line.iter().enumerate() {
                match tile {
                    Tile::Floor => { return (row, col); },
                    _ => {},
                }
            }
        }

        panic!();
    }

    fn wrap2(&self, (row, col): Pos, facing: D) -> Option<Pos> {
        todo!()
    }

    fn wrap(&self, (row, col): Pos, facing: D) -> Option<Pos> {
        use D::*;
        match facing {
            N => {
                // find the bottommost floor tile in this column
                for (row, line) in self.grid.iter().enumerate().rev() {
                    match line[col] {
                        Tile::Empty => continue,
                        Tile::Wall => return None,
                        Tile::Floor => return Some((row, col)),
                    }
                }
            },
            S => {
                // find the topmost floor tile in this column
                for (row, line) in self.grid.iter().enumerate() {
                    match line[col] {
                        Tile::Empty => continue,
                        Tile::Wall => return None,
                        Tile::Floor => return Some((row, col)),
                    }
                }
            },
            E => {
                // find the leftmost floor tile in this row
                for (col, tile) in self.grid[row].iter().enumerate() {
                    match tile {
                        Tile::Empty => continue,
                        Tile::Wall => return None,
                        Tile::Floor => return Some((row, col)),
                    }
                }
            },
            W => {
                // find the rightmost floor tile in this row
                for (col, tile) in self.grid[row].iter().enumerate().rev() {
                    match tile {
                        Tile::Empty => continue,
                        Tile::Wall => return None,
                        Tile::Floor => return Some((row, col)),
                    }
                }
            },
        }

        None
    }

    fn step(&self, mut pos: Pos, facing: D, steps: usize) -> Pos {
        for _ in 0..steps {
            use D::*;
            let newpos = match facing {
                N => (pos.0 - 1, pos.1),
                E => (pos.0, pos.1 + 1),
                S => (pos.0 + 1, pos.1),
                W => (pos.0, pos.1 - 1),
            };
            match self.grid[newpos.0][newpos.1] {
                Tile::Empty => {
                    // wrap around if possible
                    if let Some(newpos) = self.wrap2(newpos, facing) {
                        // we were not blocked by a wall when wrapping
                        pos = newpos;
                    } else {
                        // we were blocked by a wall
                        break;
                    }
                },
                Tile::Floor => {
                    // new position is good, update
                    pos = newpos;
                },
                Tile::Wall => {
                    // stop, don't update the position
                    break;
                },
            };
        }
        pos
    }
}

#[derive(Debug, Copy, Clone)]
enum D {
    N, E, S, W,
}

impl D {
    fn turn_right(self) -> Self {
        use D::*;
        match self {
            N => E,
            E => S,
            S => W,
            W => N,
        }
    }

    fn turn_left(self) -> Self {
        use D::*;
        match self {
            N => W,
            E => N,
            S => E,
            W => S,
        }
    }
}

fn solve1(map: &Map, instructions: &str) -> usize {
    let mut path = HashMap::new();

    let mut pos = map.start();
    let mut facing = D::E;

    println!("{pos:?}");

    let instr = instructions.trim().bytes().collect::<Vec<_>>();
    for instr in instr[..].group_by(|a, b| a.is_ascii_digit() && b.is_ascii_digit()) {
        let instr = std::str::from_utf8(instr).unwrap();
        // println!("{}", instr);

        path.insert(pos, facing);

        if let Ok(steps) = instr.parse::<usize>() {
            pos = map.step(pos, facing, steps);
        } else {
            match instr {
                "R" => facing = facing.turn_right(),
                "L" => facing = facing.turn_left(),
                _ => panic!(),
            }
        }

    }
    println!("{facing:?}");
    println!("{pos:?}");

    map.print_path(&path);

    1000 * pos.0 + 4 * pos.1 + match facing {
        D::E => 0,
        D::S => 1,
        D::W => 2,
        D::N => 4,
    }
}


fn main() {
    #[allow(unused_variables)]
    let inp = if SMALL {
        std::fs::read_to_string("small").unwrap()
    } else {
        std::fs::read_to_string("big").unwrap()
    };
    let inp = &inp;

    let (map, instructions) = Map::parse(inp);

    map.print();
    println!("");
    println!("{}", instructions);

    println!("{}", solve1(&map, instructions));
}
