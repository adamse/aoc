#[derive(Default, Debug, Clone, Copy)]
struct R {
    /// inclusive
    start: u64,
    /// not inclusive
    end: u64,
}
impl std::fmt::Display for R {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.start, self.end)
    }
}
impl R {
    fn from_start_count(start: u32, count: u32) -> R {
        R {
            start: start as u64,
            end: start as u64 + count as u64,
        }
    }
    fn empty() -> Self {
        R { start: 0, end: 0, }
    }
    fn is_empty(self) -> bool {
        self.start == self.end
    }
    fn contains(self, item: u32) -> bool {
        self.start <= item as u64 && (item as u64) < self.end
    }
}
#[derive(Default, Debug)]
struct Mapping {
    ranges: Vec<(R, u64)>,
}
impl Mapping {
    fn map(&self, i: u32) -> u32 {
        for &(r, target) in &self.ranges {
            if r.contains(i) {
                return i - r.start as u32 + target as u32;
            }
        }
        i
    }
    fn map_range(&self, mut ir: R) -> Vec<R> {
        let mut out = vec![];
        for &(tr, target) in &self.ranges {
            if ir.is_empty() { break; }
            // ir .|---|.....
            // tr .......|--|
            if ir.end <= tr.start {
                // no overlap, identity map
                out.push(ir);
                // done
                ir = R::empty();
                break;
            }
            // ir .|---|......
            // tr ...|---|....
            if ir.start <= tr.start && tr.start <= ir.end && ir.end <= tr.end {
                // no overlap part, identity map
                out.push(R {
                    start: ir.start,
                    end: tr.start
                });
                // overlap part, map it
                out.push(R {
                    start: target,
                    end: target + ir.end - tr.start,
                });
                // done
                ir = R::empty();
                break;
            }
            // ir .|----|..
            // tr ...|-|...
            if ir.start <= tr.start && tr.end <= ir.end {
                // no overlap part, identity map
                out.push(R {
                    start: ir.start,
                    end: tr.start
                });
                // overlap, map it
                out.push(R {
                    start: target,
                    end: target + tr.end - tr.start,
                });
                // remaining
                ir = R {
                    start: tr.end,
                    end: ir.end,
                };
                continue;
            }
            // ir ...|--|...
            // tr ..|----|..
            if tr.start < ir.start &&
                    ir.end <= tr.end {
                // overlap, map it
                out.push(R {
                    start: target + ir.start - tr.start,
                    end: target + ir.end - tr.start,
                });
                // done
                ir = R::empty();
                break;
            }
            // ir ....|---|.
            // tr ..|---|...
            if tr.start < ir.start && ir.start < tr.end {
                // overlap part, map it
                out.push(R {
                    start: target + ir.start - tr.start,
                    end: target + tr.end - tr.start,
                });
                // rest
                ir = R {
                    start: tr.end,
                    end: ir.end,
                };
                // continue
                continue;
            }
            // ir .....|---|
            // tr .|--|.....
            if tr.end <= ir.start {
                continue;
            }

            println!("ir {ir}");
            println!("tr {tr}");

            panic!();
        }
        if !ir.is_empty() { out.push(ir); }
        out.iter().copied().filter(|x| !(*x).is_empty()).collect()
    }
}
fn parse_all_numbers(i: &str) -> Vec<u32> {
    i.split_whitespace().filter_map(|i| i.parse().ok()).collect()
}
fn parse(i: &str) -> (Vec<u32>, Vec<Mapping>) {
    let mut it = i.split("\n\n");
    let seeds = it.next().unwrap();
    let seeds = parse_all_numbers(seeds);

    let mut mappings = vec![];
    for map in it {
        let mut it = map.lines();
        it.next().unwrap(); // skip map name line
        let mut mapping = Mapping::default();
        for line in it {
            let nums = parse_all_numbers(line);
            mapping.ranges.push((R::from_start_count(nums[1], nums[2]), nums[0] as u64));
        }
        mapping.ranges.sort_by_key(|x| x.0.start);
        mappings.push(mapping);
    }

    (seeds, mappings)
}
fn p1(seeds: &[u32], mappings: &[Mapping]) -> u32 {
    seeds.iter()
        .map(|seed| {
            // println!("path: {seed}");
            mappings.iter().fold(*seed, |seed, mapping| {
                // println!("{seed}");
                mapping.map(seed)
            })
        })
        .min().unwrap()
}
fn p2(seeds: &[R], mappings: &[Mapping]) -> u64 {
    let mut res = seeds.iter()
        .map(|r|
           mappings.iter().fold(vec![*r], |r, mapping| {
               // println!("{r:?}");
               // println!("{mapping:?}");
               let r = r.iter().map(|r|mapping.map_range(*r)).flatten().collect();
               // println!("{r:?}");
               r
           }))
        .flatten()
        .collect::<Vec<_>>();
    // res.iter().for_each(|r| println!("{r}"));
    res.sort_by_key(|r| r.start);
    res[0].start
}
fn main() {
    let i = std::fs::read_to_string("inp.txt").unwrap();
    // let i = std::fs::read_to_string("inp1.txt").unwrap();
    let i = i.trim();
    let (seeds, mappings) = parse(i);
    // println!("{seeds:?}");
    // for mapping in &mappings { println!("{mapping:?}"); }
    println!("{}", p1(&seeds[..], &mappings[..]));
    // println!("{}", mappings[0].map(79));
    let seeds = seeds.chunks_exact(2).map(|x| R::from_start_count(x[0], x[1])).collect::<Vec<_>>();
    // let seeds = [R { start: 82, end: 83 }];

    println!("{}", p2(&seeds[..], &mappings[..]));
}
