pub fn square(s: u32) -> u64 {
    if !(1..=64).contains(&s) {
        panic!("Square must be between 1 and 64")
    }
    2_u64.pow(s - 1)
    // unimplemented!("grains of rice on square {s}");
}

pub fn total() -> u64 {
    let mut result = 0;
    for i in 1..=64 {
        result += square(i);
    }
    result
    // unimplemented!();
}
