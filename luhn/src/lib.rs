/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let tmp: Vec<char> = code.replace(' ', "").chars().collect();
    if tmp.len() <= 1 {
        return false;
    }
    let mut tmp2: Vec<u32> = Vec::new();
    for i in tmp.iter() {
        match i.to_digit(10) {
            Some(y) => tmp2.push(y),
            None => return false,
        }
    }

    let mut sum = 0;
    for (idx, value) in tmp2.iter().rev().enumerate() {
        if idx % 2 == 1 {
            let value = value * 2;
            if value > 9 {
                sum += value - 9;
            } else {
                sum += value;
            }
        } else {
            sum += value;
        }
    }

    sum % 10 == 0
}
