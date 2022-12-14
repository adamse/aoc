#![feature(array_windows)]

mod big;

fn main() {
    let path1 = [[498,4],[498,6],[496,6i32]];
    let path2 = [[503,4],[502,4],[502,9],[494,9i32]];
    let small = &[&path1[..], &path2[..]];
    let big = big::big();

    let mut grid = std::collections::HashSet::<(i32, i32)>::new();
    let mut maxy = 0i32;
    let mut minx = i32::MAX;
    let mut maxx = i32::MIN;

    for path in big {
        for &[[ax, ay], [bx, by]] in path.array_windows::<2>() {
            minx = minx.min(ax).min(bx);
            maxx = maxx.max(ax).max(bx);

            // update lowest level seen so we know where the abyss starts
            maxy = maxy.max(ay).max(by);


            // println!("{},{} {},{}", ax, ay, bx, by);
            if ax == bx {
                for y in ay.min(by)..=ay.max(by) {
                    grid.insert((ax, y));
                }
            } else if ay == by {
                for x in ax.min(bx)..=ax.max(bx) {
                    grid.insert((x, ay));
                }
            } else {
                panic!("diagonal line: {ax},{ay} {bx},{by}");
            }
        }
    }

    for y in 0..=maxy {
        for x in minx..=maxx {
            let sym = if grid.contains(&(x, y)) { "█" } else { " " };
            print!("{sym}");
        }
        println!("");
    }

    println!("{maxy}");

    {
        let now = std::time::Instant::now();

        let mut grid = grid.clone();

        let mut total = 0u32;

        'next_grain: loop {
            let mut x = 500i32;
            let mut y = 0i32;

            loop {
                if y > maxy {
                    // if we fall into the abyss we're done
                    break 'next_grain;
                }

                if !grid.contains(&(x, y+1)) {
                    // down
                    y += 1;
                } else if !grid.contains(&(x-1, y+1)) {
                    // down-left
                    x -= 1;
                    y += 1;
                } else if !grid.contains(&(x+1, y+1)) {
                    // down-right
                    x += 1;
                    y += 1;
                } else {
                    // came to a stop
                    grid.insert((x, y));
                    total += 1;
                    continue 'next_grain;
                }
            }
        }

        println!("{}ms", now.elapsed().as_millis());
        println!("{total}");
    }

    {
        let now = std::time::Instant::now();

        let mut sand = grid.clone();

        let mut total = 0u32;

        'next_grain: loop {
            let mut x = 500i32;
            let mut y = 0i32;

            loop {
                if y+1 == maxy+2 {
                    // we're on the floor, stop
                    sand.insert((x, y));
                    total += 1;
                    continue 'next_grain;
                } else if !sand.contains(&(x, y+1)) {
                    // down
                    y += 1;
                } else if !sand.contains(&(x-1, y+1)) {
                    // down-left
                    x -= 1;
                    y += 1;
                } else if !sand.contains(&(x+1, y+1)) {
                    // down-right
                    x += 1;
                    y += 1;
                } else if (x, y) == (500, 0) {
                    // we couldn't move but we're at the starting position
                    total += 1;
                    break 'next_grain;
                } else {
                    // we couldn't move down, stop
                    sand.insert((x, y));
                    total += 1;
                    continue 'next_grain;
                }
            }
        }
        println!("{}ms", now.elapsed().as_millis());

        println!("{total}");

        for y in 0..=maxy {
            for x in minx..=maxx {
                let sym = if grid.contains(&(x, y)) {
                    "█"
                } else if sand.contains(&(x, y)) {
                    "░"
                } else {
                    " "
                };
                print!("{sym}");
            }
            println!("");
        }
    }
}
