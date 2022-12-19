use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
#[allow(dead_code)]
enum Bot {
    Ore = 0,
    Clay,
    Obsidian,
    Geode,
}

enum Choice {
    Build(Bot),
    Wait,
}

type Cost = [u16; 3];

struct Blueprint {
    cost: [Cost; 4],
    max_spend: [u16; 3],

    cache: HashMap<State, u32>,
}

#[derive(PartialEq, Eq, Hash)]
struct State {
    production: [u16; 3],
    inventory: [u16; 3],
    time_left: u8,
}

trait Graph: Sized {
    type Aux;

    fn next(&self, aux: &Self::Aux) -> Vec<Self>;
}

impl Blueprint {
    fn setup(costs: [Cost; 4]) -> Self {
        let mut max_spend = [0; 3];
        costs.iter().for_each(|&[ore, clay, obsidian]| {
            max_spend[0] = max_spend[0].max(ore);
            max_spend[1] = max_spend[1].max(clay);
            max_spend[2] = max_spend[2].max(obsidian);
        });

        Blueprint {
            cost: costs,
            max_spend,
            cache: HashMap::new(),
        }
    }

    fn can_build(&self, s: &State, bot: Bot) -> bool {
        for ii in 0..3 {
            if self.cost[bot as usize][ii] > s.inventory[ii] {
                return false;
            }
        }

        true
    }

    fn is_useful(&self, s: &State, bot: Bot) -> bool {
        s.production[bot as u8 as usize] <=
            self.max_spend[bot as u8 as usize]
    }

    fn run(&mut self, time_left: u8) -> u32 {
        let s = State {
            inventory: [0; 3],
            production: [1, 0, 0],
            time_left,
        };

        self.worker(s)
    }

    // fn next_states(&self, mut s: State

    fn worker(&mut self, mut s: State) -> u32 {
        // dfs search

        if s.time_left == 0 { return 0; }

        // see what choices we have
        let mut choices = vec![];

        use Choice::*;

        let geode = if self.can_build(&s, Bot::Geode) {
            choices.push(Build(Bot::Geode));
            true
        } else {
            for bot in 0u8..=2 {
                let bot = unsafe { *(&bot as *const u8 as *const Bot) };
                if self.can_build(&s, bot) && self.is_useful(&s, bot) {
                    choices.push(Build(bot));
                }
            }
            false
        };

        // if we're building a geode bot or if we already can build all bots there is no reason to
        // wait
        if !geode && choices.len() < 3 {
            choices.push(Wait);
        }

        // update inventory from production
        for ii in 0..3 {
            s.inventory[ii] += s.production[ii];

            // cap inventory to max we could possibly spend
            s.inventory[ii] = s.inventory[ii].min(s.time_left as u16 * self.max_spend[ii]);
        }

        // check if we've already seen this state
        if let Some(&max) = self.cache.get(&s) {
            return max;
        }

        // build new bots
        let max = choices.iter().map(|choice| {

            let mut s = State { time_left: s.time_left - 1, ..s };

            let extra = match *choice {
                Wait => 0,
                Build(bot) => {
                    // pay for the new robot
                    for ii in 0..3 {
                        s.inventory[ii] -= self.cost[bot as usize][ii];
                    }

                    if bot != Bot::Geode {
                        s.production[bot as usize] += 1;
                    }

                    if bot == Bot::Geode { s.time_left } else { 0 }
                },
            };

            extra as u32 + self.worker(s)
        }).max().unwrap();

        self.cache.insert(s, max);

        max
    }
}

mod big;

fn main() {

    #[allow(unused_variables)]
    let blueprints = &[
        [[4,0,0], [2,0,0], [3,14,0], [2,0,7]],
        [[2,0,0], [3,0,0], [3,8,0], [3,0,12]],
    ];
    let blueprints = big::BIG;

    let start = std::time::Instant::now();

    let part1: u32 = blueprints.iter().enumerate().map(|(i, bl)| {
        let res = Blueprint::setup(*bl).run(24);
        println!("{res}");
        (i as u32 + 1) * res
    }).sum();

    println!("p1: {part1}");
    println!("t: {}", start.elapsed().as_millis());

    let part2: u32 = blueprints[0..3].iter().map(|bl| {
        let res = Blueprint::setup(*bl).run(32);
        println!("{res}");
        res
    }).product();

    println!("p2: {part2}");
    println!("t: {}", start.elapsed().as_millis());
}
