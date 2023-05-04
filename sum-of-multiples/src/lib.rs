use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut result: HashSet<u32> = HashSet::new();
    for i in factors.iter() {
        if *i == 0 {
            continue;
        }
        let mut quotient = limit / i;
        let remainder = limit % i;
        if remainder == 0 {
            quotient -= 1;
        }
        let tmp: HashSet<u32> = HashSet::from_iter((1..=quotient).map(|n| n * i));
        result.extend(tmp);
    }
    result.iter().sum::<u32>()
}
