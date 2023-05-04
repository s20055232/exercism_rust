pub fn is_armstrong_number(num: u32) -> bool {
    let mut length = 0;
    let mut tmp_num = num;
    while tmp_num > 0 {
        tmp_num = tmp_num.div_euclid(10);
        length += 1;
    }
    let mut result: u32 = 0;
    tmp_num = num;
    while tmp_num > 0 {
        let digit = tmp_num % 10;
        tmp_num = tmp_num.div_euclid(10);
        let pow = match digit.checked_pow(length) {
            Some(x) => x,
            None => return false,
        };
        result = match result.checked_add(pow) {
            Some(x) => x,
            None => return false,
        };
    }
    result == num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(is_armstrong_number(9));
    }
    #[test]
    fn test_2() {
        assert!(!is_armstrong_number(10));
    }

    #[test]
    fn test_3() {
        assert!(is_armstrong_number(153));
    }

    #[test]
    fn test_4() {
        assert!(!is_armstrong_number(154));
    }
}
