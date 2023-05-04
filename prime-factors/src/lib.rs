pub fn factors(n: u64) -> Vec<u64> {
    // don't know why timeout
    if n == 1 {
        return Vec::new();
    }
    let mut result: Vec<u64> = Vec::new();
    let mut n = n;
    loop {
        let tmp = get_prime_number(n);
        result.push(tmp);
        if tmp == n {
            break;
        }
        n /= tmp;
    }
    result
}

fn get_prime_number(n: u64) -> u64 {
    for i in 2..=((n as f64).sqrt().floor() as u64) {
        if n % i == 0 {
            return i;
        }
    }
    n
}
