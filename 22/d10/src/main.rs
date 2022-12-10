use std::io;

/// micro ops, each take 1 cycle
enum MuOp {
    Nop,
    Addx(i64),
}

impl MuOp {
    #[allow(dead_code)]
    fn parse(instr: &str) -> Vec<MuOp> {
        let mut muops = Vec::new();
        if instr.starts_with("noop") {
            muops.push(MuOp::Nop);
        } else if let Some(("addx", val)) = instr.split_once(" ") {
            let val = val.parse::<i64>().unwrap();
            muops.push(MuOp::Nop);
            muops.push(MuOp::Addx(val));
        } else {
            panic!("unknown instruction: {instr}");
        }

        return muops;
    }
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Noop,
    Addx(i64),
}

impl Op {
    fn parse(instr: &str) -> Self {
        if instr.starts_with("noop") {
            return Op::Noop;
        } else if let Some(("addx", val)) = instr.split_once(" ") {
            let val = val.parse::<i64>().unwrap();
            return Op::Addx(val)
        } else {
            panic!("unknown instruction: {instr}");
        }
    }
}

/// Iterate over the micro ops in an instruction
impl IntoIterator for Op {
    type Item = MuOp;
    type IntoIter = MuOpIter;

    fn into_iter(self) -> MuOpIter {
        MuOpIter {
            instr: self,
            cycle: OpCycle(0),
        }
    }
}

struct OpCycle(u64);

struct MuOpIter {
    instr: Op,
    cycle: OpCycle,
}

impl Iterator for MuOpIter {
    type Item = MuOp;
    fn next(&mut self) -> Option<MuOp> {
        let muop = match self.instr {
            Op::Noop =>
                match self.cycle.0 {
                    0 => Some(MuOp::Nop),
                    _ => None,
                },
            Op::Addx(val) =>
                match self.cycle.0 {
                    0 => Some(MuOp::Nop),
                    1 => Some(MuOp::Addx(val)),
                    _ => None,
                },
        };

        (*self).cycle.0 += 1;

        muop
    }
}

fn main() -> io::Result<()> {
    let buf = std::fs::read_to_string("big")?;

    {
        let mut cycle = 0u64;
        let mut x = 1i64;
        let mut strength = 0i64;

        for instr in buf.trim().lines().map(Op::parse).flatten() {

            // start cycle
            cycle += 1;

            // check strength
            match cycle {
                20 | 60 | 100 | 140 | 180 | 220 => {
                    let update = (cycle as i64) * x;
                    strength += update;
                },
                _ => {},
            }

            // instruction effects
            match instr {
                MuOp::Nop => { },

                MuOp::Addx(val) => {
                    x += val;
                }
            }
        }

        println!("{strength}");
    }

    {
        let mut x = 1i64;

        const CRT_COLS: u64 = 40;
        const CRT_ROWS: u64 = 6;
        let mut crt_pixel = 0u64;

        for instr in buf.trim().lines().map(Op::parse).flatten() {
            // start cycle

            // crt works
            // draw
            let crt_col = (crt_pixel % CRT_COLS) as i64;
            if crt_col == 0 {
                println!("");
            }
            if x - 1 <= crt_col && crt_col <= x + 1 {
                print!("█");
            } else {
                print!(" ");
            }

            // move to next pixel
            crt_pixel += 1;
            crt_pixel %= CRT_ROWS * CRT_COLS;

            // instruction effects
            match instr {
                MuOp::Nop => { },

                MuOp::Addx(val) => {
                    x += val;
                }
            }
        }

        println!("");
    }

    Ok(())
}
