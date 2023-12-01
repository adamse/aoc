use num_complex::Complex;
use std::collections::HashSet;
use std::collections::HashMap;

type Pos = Complex<i32>;
type Move = Complex<i32>;

fn component_min(a: Pos, b: Pos) -> Pos {
    Pos::new(
        a.re.min(b.re),
        a.im.min(b.im)
    )
}

fn component_max(a: Pos, b: Pos) -> Pos {
    Pos::new(
        a.re.max(b.re),
        a.im.max(b.im)
    )
}

fn bounds(positions: &HashSet<Pos>) -> (Pos, Pos) {
    let mut min_bounds = Pos::new(i32::MAX, i32::MAX);
    let mut max_bounds = Pos::new(i32::MIN, i32::MIN);

    // find the bounding box for the elves
    for &pos in positions {
        min_bounds = component_min(min_bounds, pos);
        max_bounds = component_max(max_bounds, pos);
    }

    (min_bounds, max_bounds)
}

fn print_grid(positions: &HashSet<Pos>) {
    let (min, max) = bounds(positions);

    for y in (min.im..=max.im).rev() {
        for x in min.re..=max.re {
            if positions.contains(&Pos::new(x,y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn main() {
    #[allow(unused_variables)]
    let inp = std::fs::read_to_string("small").unwrap();
    let inp = std::fs::read_to_string("big").unwrap();
    let inp = inp.trim();

    let mut positions = HashSet::new();

    // parse map
    for (y, line) in inp.lines().enumerate() {
        for (x, pos) in line.chars().enumerate() {
            match pos {
                '.' => {},
                '#' => { positions.insert(Pos::new(x as i32, -(y as i32))); },
                _ => { panic!(); },
            }
        }
    }

    // possible move directions
    let n = Move::new(0, 1);
    let s = Move::new(0, -1);
    let w = Move::new(-1, 0);
    let e = Move::new(1, 0);

    let ne = Move::new(1, 1);
    let nw = Move::new(-1, 1);
    let se = Move::new(1, -1);
    let sw = Move::new(-1, -1);

    // direction to try to move and directions to check
    let mut move_order = std::collections::VecDeque::from([
        [n, ne, nw],
        [s, se, sw],
        [w, nw, sw],
        [e, ne, se],
    ]);

    // do 10 rounds of moves
    for round in 0.. {

        // println!("{round}");
        // print_grid(&positions);

        // map from new position to current elves
        let mut proposed_moves: HashMap<Pos, Vec<Pos>> = HashMap::new();

        let mut all_elfs_ok_positions = true;

        // each elf proposes a new move
        'next_elf: for &elf in positions.iter() {
            // check if we're good where we are
            let ok_pos = [n, ne, e, se, s, sw, w, nw].iter().all(|&check|
                !positions.contains(&(elf + check))
            );

            if ok_pos {
                proposed_moves.entry(elf).or_default().push(elf);
                continue 'next_elf;
            }

            all_elfs_ok_positions = false;

            // check if we can move
            for &checks@[proposed_move, _, _] in &move_order {

                // check if we can move to the new position on the current board
                let ok_pos = checks.iter().all(|&check|
                    !positions.contains(&(elf + check))
                );

                if ok_pos {
                    // new position
                    let new_elf = elf + proposed_move;

                    let new = proposed_moves.entry(new_elf).or_default();
                    new.push(elf);

                    continue 'next_elf;
                }
            }

            // couldn't find a new position, stay where we were
            proposed_moves.entry(elf).or_default().push(elf);
        }

        // try to execute the moves if there are no conflicts, otherwise keep the old position
        let mut new_positions = HashSet::new();
        for (&new_position, elfs) in proposed_moves.iter() {
            if elfs.len() == 1 {
                // only 1 elf tried to move to the new position, let them
                new_positions.insert(new_position);
            } else {
                // there was a conflict, the elves stay in their current positions
                for &elf in elfs {
                    new_positions.insert(elf);
                }
            }
        }

        // update current positions
        positions = new_positions;

        // move first direction to the end of the try ordering
        move_order.rotate_left(1);

        // part 1
        if round == 9 {
            let (min, max) = bounds(&positions);
            let grid = max - min;
            let total = (grid.re + 1) * (grid.im + 1);
            println!("{:?}", total - positions.len() as i32);
        }

        // part 2
        if all_elfs_ok_positions {
            println!("{}", round + 1);
            break;
        }
    }
}
