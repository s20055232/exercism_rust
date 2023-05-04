pub fn nth(n: u32) -> u32 {
    let mut times = 0;
    'outer: for i in 2.. {
        for j in 2..=(i as f32).sqrt().ceil() as u32 {
            if i != j && i % j == 0 {
                continue 'outer;
            }
        }
        if times == n {
            return i;
        } else {
            times += 1;
        }
    }
    unreachable!()
}
