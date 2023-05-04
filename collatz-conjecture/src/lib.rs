pub fn collatz(n: u64) -> Option<u64> {
    let mut result = n;
    let mut times = 0;
    loop {
        if result == 0 {
            return None;
        } else if result == 1 {
            return Some(times);
        } else if let Some(x) = div_2(result) {
            times += 1;
            result = x;
        } else {
            return None;
        }
    }
}

fn div_2(num: u64) -> Option<u64> {
    let (remainder, flag) = num.overflowing_rem(2);
    if flag {
        None
    } else if remainder == 0 {
        Some(num / 2)
    } else {
        mul_3(num)
    }
}

fn mul_3(num: u64) -> Option<u64> {
    let (num, flag) = num.overflowing_mul(3);
    if flag {
        None
    } else {
        plus_1(num)
    }
}

fn plus_1(num: u64) -> Option<u64> {
    let (num, flag) = num.overflowing_add(1);
    if flag {
        None
    } else {
        Some(num)
    }
}
