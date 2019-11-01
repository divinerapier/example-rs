fn main() {
    // let mut cnt1 = 0;
    // let mut cnt2 = 0;
    // for i in (0..10000) {
    //     let a = 8 - i % 8;
    //     let b = 8 - (i & 7); // 优先级低
    //     let c = (8 ^ (i & 8)) as i32;
    //     // println!("{} % 8 = {}, {} & 7 = {}", i, a, i, b);
    //     if a == b {
    //         cnt1 += 1;
    //     }
    //     if a == c {
    //         cnt2 += 1;
    //     }
    // }
    // println!("{:?}, {:?}", cnt1, cnt2);
    // for i in 0..8 {
    //     for j in 0..8 {
    //         println!("{:03b} ^ {:03b} = {:03b}", i, j, i ^ j);
    //     }
    // }

    for i in 0..256 {
        println!("{:3?} -> {:3?}", i, pad(i));
    }
}

fn pad(i: u32) -> u32 {
    match i & 7 {
        0 => i,
        d @ _ => i + 8 - d,
    }
}
