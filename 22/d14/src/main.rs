#![feature(array_windows)]

mod big;

fn main() {
    let path1 = [[498,4],[498,6],[496,6i32]];
    let path2 = [[503,4],[502,4],[502,9],[494,9i32]];
    let small = &[&path1[..], &path2[..]];
    let big = big::big();

    let now = std::time::Instant::now();

    let mut grid = [[0u8; 1000]; 1000];
    let mut maxy = 0i32;
    let mut minx = i32::MAX;
    let mut maxx = i32::MIN;

    for path in big {
        for &[[ax, ay], [bx, by]] in path.array_windows::<2>() {
            minx = minx.min(ax).min(bx);
            maxx = maxx.max(ax).max(bx);

            // update lowest level seen so we know where the abyss starts
            maxy = maxy.max(ay).max(by);


            for x in ax.min(bx)..=ax.max(bx) {
                for y in ay.min(by)..=ay.max(by) {
                    grid[x as usize][y as usize] = 1;
                    // grid.insert((x, y));
                }
            }
            /*
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
            */
        }
    }
    println!("{}", now.elapsed().as_micros());

    println!("{} {} {}", minx, maxx, maxy);

    /*
    for y in 0..=maxy {
        for x in minx..=maxx {
            let sym = if grid.contains(&(x, y)) { "█" } else { " " };
            print!("{sym}");
        }
        println!("");
    }
    */

    {
        let now = std::time::Instant::now();

        let mut grid = grid.clone();

        let mut total = 0u32;

        'next_grain: loop {
            let mut x = 500;
            let mut y = 0usize;

            loop {
                if y > maxy as usize {
                    // if we fall into the abyss we're done
                    break 'next_grain;
                }

                if grid[x][y+1] == 0 {
                    // down
                    y += 1;
                } else if grid[x-1][y+1] == 0 {
                    // down-left
                    x -= 1;
                    y += 1;
                } else if grid[x+1][y+1] == 0 {
                    // down-right
                    x += 1;
                    y += 1;
                } else {
                    // came to a stop
                    grid[x][y] = 2;
                    total += 1;
                    continue 'next_grain;
                }
                /*
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
                */
            }
        }

        println!("{}", now.elapsed().as_micros());
        println!("{total}");
    }

    {
        let now = std::time::Instant::now();

        let mut sand = grid.clone();

        let mut total = 0u32;

        'next_grain: loop {
            let mut x = 500;
            let mut y = 0;

            loop {
                if y+1 == maxy as usize +2 {
                    // we're on the floor, stop
                    sand[x][y] = 2;
                    total += 1;
                    continue 'next_grain;
                } else if sand[x][y+1] == 0 {
                    // down
                    y += 1;
                } else if sand[x-1][y+1] == 0 {
                    // down-left
                    x -= 1;
                    y += 1;
                } else if sand[x+1][y+1] == 0 {
                    // down-right
                    x += 1;
                    y += 1;
                } else if (x, y) == (500, 0) {
                    // we couldn't move but we're at the starting position
                    total += 1;
                    break 'next_grain;
                } else {
                    // we couldn't move down, stop
                    sand[x][y] = 2;
                    total += 1;
                    continue 'next_grain;
                }
            }
        }
        println!("{}", now.elapsed().as_micros());

        println!("{total}");

        /*
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
        */
    }
}
