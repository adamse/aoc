#![feature(iter_array_chunks)]

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Vec3 {
    x: i32,
    y: i32,
    z: i32,
}

impl Vec3 {
    fn x(mut self, add: i32) -> Vec3 {
        self.x += add;
        self
    }
    fn y(mut self, add: i32) -> Vec3 {
        self.y += add;
        self
    }
    fn z(mut self, add: i32) -> Vec3 {
        self.z += add;
        self
    }

    fn from(v: [i32;3]) -> Vec3 {
        Vec3 {
            x: v[0],
            y: v[1],
            z: v[2],
        }
    }

    fn lte(self, other: Vec3) -> bool {
        self.x <= other.x &&
            self.y <= other.y &&
            self.z <= other.z
    }

    fn gte(self, other: Vec3) -> bool {
        self.x >= other.x &&
            self.y >= other.y &&
            self.z >= other.z
    }
}

struct World {
    lava: std::collections::HashSet<Vec3>,
    maxbound: Vec3,
    minbound: Vec3,
}

impl World {
    fn search(&self) -> u64 {
        let mut surface = 0;

        // a bfs search through the volume

        // air coordinates that we don't need to revisit
        let mut air = std::collections::HashSet::new();

        // start at the minbound
        let mut q = std::collections::VecDeque::new();
        q.push_back(self.minbound);

        while let Some(v) = q.pop_front() {
            if air.contains(&v) {
                // skip if we'be already seen it and it is air
                continue;
            }
            if self.lava.contains(&v) {
                // count if we hit lava
                // this must mean we hit the lava from a novel direction! so definitely unseen
                // surface
                surface += 1;
                continue;
            }

            // mark current coord as air
            air.insert(v);

            // queue the neighbours exploration if they are not known to be air, or outside the box
            for &n in &[v.x(1), v.y(1), v.z(1), v.x(-1), v.y(-1), v.z(-1)] {
                if n.gte(self.minbound) && self.maxbound.gte(n) && !air.contains(&n) {
                    q.push_back(n);
                }
            }
        }

        surface
    }
}

use std::collections::HashSet;
fn main() {
    let cubes = std::fs::read_to_string("small").unwrap();
    let cubes = std::fs::read_to_string("big").unwrap();
    let cubes = cubes
        .trim()
        .split(&[',','\n'])
        .map(|s| s.parse::<i32>().unwrap())
        .array_chunks::<3>()
        .collect::<Vec<_>>();
    let cubes = &cubes;

    let cubes = cubes.iter().copied().collect::<HashSet<_>>();

    let ds = [-1, 1];

    let mut count = 0u32;

    // free faces
    for &[x, y, z] in cubes.iter() {
        for dx in ds {
            if !cubes.contains(&[x+dx, y, z]) {
                count += 1;
            }
        }
        for dy in ds {
            if !cubes.contains(&[x, y+dy, z]) {
                count += 1;
            }
        }
        for dz in ds {
            if !cubes.contains(&[x, y, z+dz]) {
                count += 1;
            }
        }
    }
    println!("{count}");

    let mut minbound = [i32::MAX; 3];
    let mut maxbound = [i32::MIN; 3];

    cubes.iter().for_each(|&[x, y, z]| {
        maxbound[0] = maxbound[0].max(x);
        maxbound[1] = maxbound[1].max(y);
        maxbound[2] = maxbound[2].max(z);

        minbound[0] = minbound[0].min(x);
        minbound[1] = minbound[1].min(y);
        minbound[2] = minbound[2].min(z);
    });

    let world = World {
        lava: cubes.iter().map(|&x| Vec3::from(x)).collect(),
        maxbound: Vec3::from(maxbound).x(1).y(1).z(1),
        minbound: Vec3::from(minbound).x(-1).y(-1).z(-1),
    };

    let count = world.search();
    println!("{count}");
}
