fn mov(msg: &mut Vec<(usize, i64)>, index: usize) {
    let x@(_, val) = msg.remove(index);

    let new = (index as i64 + val) % msg.len() as i64;

    let new = if new < 0 {
        (msg.len() as i64 - new.abs()) as usize
    } else {
        new as usize
    };

    msg.insert(new, x);
}

fn mix(msg: &[i64]) -> Vec<i64> {
    let msg: Vec<_> = msg
        .iter()
        .cloned()
        .enumerate()
        .collect();

    let mut out = msg.clone();

    for (pos, val) in msg {
        let cur = out.iter().position(|&x| x == (pos, val)).unwrap();
        mov(&mut out, cur);
    }

    out.iter().map(|&x| x.1).collect()
}

fn main() {
    let buf = std::fs::read_to_string("small").unwrap();
    // let buf = std::fs::read_to_string("big").unwrap();
    let buf = buf.trim();

    // println!("{:?}", mix(&[3, 1, 0]));
    // println!("{:?}", mix(&[5,1,0]));
    // println!("{:?}", mix(&[7,1,0]));
    // println!("{:?}", mix(&[-3,1,0]));
    // println!("{:?}", mix(&[-5,1,0]));

    let msg: Vec<_> = buf
        .lines()
        .map(|l| l.parse::<i64>().unwrap())
        .collect();

    {
        let out = mix(&msg);

        let zero_pos = out.iter().position(|&val| val == 0).unwrap();

        println!("{}",
            out[(zero_pos + 1000) % out.len()] +
            out[(zero_pos + 2000) % out.len()] +
            out[(zero_pos + 3000) % out.len()]);
    }

    {
        let mut msg: Vec<_> = msg.iter().map(|&x| x * 811589153).collect();
        for _ in 0..10 {
            msg = mix(&msg);
        }
        let zero_pos = msg.iter().position(|&val| val == 0).unwrap();

        println!("{}",
            msg[(zero_pos + 1000) % msg.len()] +
            msg[(zero_pos + 2000) % msg.len()] +
            msg[(zero_pos + 3000) % msg.len()]);
    }

}
