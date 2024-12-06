pub fn luhn(cc_number: &str) -> bool {
    let mut sum = 0;
    let mut double = false;
    let mut digit_count = 0;

    // TODO: CC numbers have to be 16 digits
    // TODO: in luhn, we should ignore the check digit; we should only check the first 15 digits

    for c in cc_number.chars().rev() {
        if let Some(digit) = c.to_digit(10) {
            digit_count += 1;
            if double {
                let double_digit = digit * 2;
                sum +=
                    if double_digit > 9 { double_digit % 10 + double_digit / 10 } else { double_digit };
            } else {
                sum += digit;
            }
            double = !double;
        } else {
            continue;
        }
    }

    digit_count > 2 && sum % 10 == 0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid_cc_number() {
        assert!(luhn("4263 9826 4026 9299"));
        assert!(luhn("4539 3195 0343 6467"));
        assert!(luhn("7992 7398 713"));
    }

    #[test]
    fn test_invalid_cc_number() {
        assert!(!luhn("4223 9826 4026 9299"));
        assert!(!luhn("4539 3195 0343 6476"));
        assert!(!luhn("8273 1232 7352 0569"));
        assert!(!luhn("4")); // reject less than 2 digits
        assert!(!luhn("4 ")); // reject less than 2 digits ignoring whitespace
    }
}

