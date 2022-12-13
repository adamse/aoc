use std::io;

struct Monkey {
    items: Vec<u64>,
    update: Box<dyn Fn(u64) -> u64>,
    test: u64,
    target1: u32,
    target2: u32,
}

fn main() -> io::Result<()> {

    let mut monkeys = [
        Monkey { items: vec![79, 98], update: Box::new(|x| x * 19), test: 23, target1: 2, target2: 3 },
        Monkey { items: vec![54, 65, 75, 74], update: Box::new(|x| x + 6), test: 19, target1: 2, target2: 0 },
        Monkey { items: vec![79, 60, 97], update: Box::new(|x| x * x), test: 13, target1: 1, target2: 3 },
        Monkey { items: vec![74], update: Box::new(|x| x + 3), test: 17, target1: 0, target2: 1 },
    ];

    let mut monkeys = [
        Monkey { items: vec![83, 88, 96, 79, 86, 88, 70], update: Box::new(|old| old * 5), test: 11, target1: 2, target2: 3},
        Monkey { items: vec![59, 63, 98, 85, 68, 72], update: Box::new(|old| old * 11), test: 5, target1: 4, target2: 0},
        Monkey { items: vec![90, 79, 97, 52, 90, 94, 71, 70], update: Box::new(|old| old + 2), test: 19, target1: 5, target2: 6},
        Monkey { items: vec![97, 55, 62], update: Box::new(|old| old + 5), test: 13, target1: 2, target2: 6},
        Monkey { items: vec![74, 54, 94, 76], update: Box::new(|old| old * old), test: 7, target1: 0, target2: 3},
        Monkey { items: vec![58], update: Box::new(|old| old + 4), test: 17, target1: 7, target2: 1},
        Monkey { items: vec![66, 63], update: Box::new(|old| old + 6), test: 2, target1: 7, target2: 5},
        Monkey { items: vec![56, 56, 90, 96, 68], update: Box::new(|old| old + 7), test: 3, target1: 4, target2: 1},
    ];

    let mut inspect_count = [0u64; 4];
    let mut inspect_count = [0u64; 8];

    let mod_: u64 = monkeys.iter().map(|x| x.test).product();

    for _ in 0..10_000 {
        for monkey in 0..monkeys.len() {
            let Monkey { items, update, test, target1, target2 } = &monkeys[monkey];

            // inspect items
            items.iter().map(|&item| {
                inspect_count[monkey] += 1;
                // let item = update(item) / 3;
                let item = update(item % mod_);
                if item % test == 0 {
                    (*target1, item)
                } else {
                    (*target2, item)
                }
            }).collect::<Vec<_>>().into_iter().for_each(|(target, item)| {
                monkeys[target as usize].items.push(item);
            });

            monkeys[monkey].items.truncate(0);
        }
    }

    println!("{inspect_count:?}");
    inspect_count.sort();
    let answer = inspect_count[inspect_count.len() - 1] * inspect_count[inspect_count.len() - 2];
    println!("{answer}");


    Ok(())
}
