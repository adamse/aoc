use std::collections::HashMap;

struct Args<'a> {
    rates: &'a [u32],
    dist: &'a [u8],
}

fn best(
    args@Args { rates, dist }: &Args,
    memo: &mut HashMap<(u64, u8, u8), u64>,
    stop_state: u64,
    state: u64,
    pos: u8,
    time_left: u8) -> u64
{
    if time_left == 0 {
        0u64
    } else if state == stop_state {
        // all valves are on
        0u64
    } else {
        if let Some(val) = memo.get(&(state, pos, time_left)) {
            return *val;
        }

        // if we haven't turned on this one yet we choose to do so if the rate is > 0
        let bit = 1u64 << pos;
        let res = if stop_state & bit != 0 && state & bit == 0 {
            let rate = rates[pos as usize];
            let score = (time_left as u32 - 1) * rate;
            let score = score as u64 +
                 best(args, memo, stop_state, state | (1 << pos), pos, time_left - 1);

            score
        } else {
            let n = rates.len();

            let mut res = 0;

            let mut valves_to_check = stop_state ^ state;
            while valves_to_check != 0 {
                let next = valves_to_check.trailing_zeros();
                // println!("{next}");
                // println!("{valves_to_check:064b}");
                valves_to_check -= 1 << next;
                // println!("{valves_to_check:064b}");
                let d = dist[pos as usize + n * next as usize];
                // panic!();

                if d <= time_left {
                    res = res.max(best(args, memo, stop_state, state, next as u8, time_left - d));
                }
            }

            res
        };

        if time_left > 0 {
            memo.insert((state, pos, time_left), res);
        }

        res
    }
}


fn main() {
    let start = std::time::Instant::now();
    // let inp = std::fs::read_to_string("small.proc").unwrap();
    let inp = std::fs::read_to_string("big.proc").unwrap();

    let mut rates = vec![];
    let mut tunnels = vec![];
    let mut names = HashMap::new();

    for (i, str) in inp.trim().lines().enumerate() {
        let mut fields = str.split(",");
        let name = fields.next().unwrap();
        let rate = fields.next().unwrap().parse::<u32>().unwrap();

        names.insert(name, i as u8);
        rates.push(rate);
        tunnels.push(fields.collect::<Vec<_>>());
    }

    let tunnels = tunnels.iter()
        .map(|tunnels|
             tunnels.iter()
                 .map(|name| names[name])
                 .collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // distance from all nodes to interesting nodes
    let dist = {
        let mut dist = vec![u8::MAX; rates.len().pow(2)];

        let n = rates.len();

        // calculate distances
        // "Floyd-Warshall"
        for (ii, tunnels) in tunnels.iter().enumerate() {
            // distance to self is 0
            dist[ii + n * ii] = 0;

            for &jj in tunnels {
                let jj = jj as usize;

                dist[ii + n * jj] = 1;
                dist[jj + n * ii] = 1;
            }
        }

        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    let new = dist[i + n * k] as u32 + dist[k + n * j] as u32;
                    if dist[i + n * j] as u32 > new {
                        dist[i + n * j] = new as u8;
                    }
                }
            }
        }

        dist
    };

    let setup = start.elapsed();
    println!("setup: {:>8}", setup.as_millis());


    // println!("{tunnels:?}\n{rates:?}\n{names:?}");

    // recursive solution: take each move and select the best
    // state: flows that are on
    // pos: current position

    let mut memo = HashMap::new();

    let stop_state: u64 = rates.iter().enumerate()
        .filter_map(|(i, rate)| (*rate > 0).then_some(1u64 << i as u64))
        .sum::<u64>();
    // println!("{stop_state:064b}");

    let args = Args {
        rates: &rates,
        dist: &dist,
    };

    let res = best(&args, &mut memo, stop_state, 0, names["AA"], 30);


    println!("{res}");
    let part1 = start.elapsed() - setup;
    println!("part1: {:>8}", part1.as_millis());

    // part 2
    // try all partitions of the valves
    // let leading0 = stop_state.leading_zeros();
    // let c = (1u64 << (64 - leading0)) - 1;
    // println!("{leading0}");
    // println!("{c:064b}");

    struct Closure<'a> {
        args: Args<'a>,
        names: &'a HashMap<&'a str, u8>,
        global_stop_state: u64,
    }

    impl<'a> Closure<'a> {
        fn call(&self, my_stop_state: u64, choices: &[u64]) -> u64 {
            if choices.is_empty() {
                // do the work
                let ele_stop_state = self.global_stop_state ^ my_stop_state;
                // println!("");
                // println!("{:064b}", self.global_stop_state);
                // println!("{my_stop_state:064b}");
                // println!("{ele_stop_state:064b}");

                let mut memo = HashMap::new();
                let me = best(&self.args, &mut memo, my_stop_state, 0, self.names["AA"], 26);

                let mut memo = HashMap::new();
                let ele = best(&self.args, &mut memo, ele_stop_state, 0, self.names["AA"], 26);

                me + ele
            } else {
                // we turn on this valve
                let pick = self.call(my_stop_state | choices[0], &choices[1..]);
                // the elephant turns on this valve
                let nopick = self.call(my_stop_state, &choices[1..]);

                pick.max(nopick)
            }
        }
    }

    let closure = Closure {
        args: Args {
            rates: &rates,
            dist: &dist,
        },
        names: &names,
        global_stop_state: stop_state,
    };

    let choices = rates.iter().enumerate()
        .filter_map(|(i, rate)| (*rate > 0).then_some(1u64 << i as u64))
        .collect::<Vec<_>>();

    let res = closure.call(0, &choices);

    println!("{res}");
    let part2 = start.elapsed() - part1 - setup;
    println!("part2: {:>8}", part2.as_millis());

}
