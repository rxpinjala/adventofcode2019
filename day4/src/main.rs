static PW_MIN: u32 = 130254;
static PW_MAX: u32 = 678275;
static PART2: bool = true;

fn main() {
    let mut c = 0;
    for pw in PW_MIN..PW_MAX {
        if valid_password(pw) {
            c = c + 1;
        }
    }

    println!("Result: {}", c);
}

fn has_equal_consecutive_pair(digits: &[u8]) -> bool {
    for w in digits.windows(2) {
        if w[0] == w[1] {
            if PART2 {
                if digits.iter().filter(|x| **x == w[0]).count() == 2 {
                    return true;
                }
            } else {
                return true;
            }
        };
    };

    false
}

fn has_increasing_digits(digits: &[u8]) -> bool {
    for w in digits.windows(2) {
        if w[0] > w[1] {
            return false;
        };
    };

    true
}

fn valid_password(n: u32) -> bool {
    let digits = digits(n);

    if digits.len() != 6 {
        return false;
    };

    if !has_equal_consecutive_pair(&digits[..]) {
        return false;
    };

    if !has_increasing_digits(&digits[..]) {
        return false;
    };

    true
}

#[test]
fn test_valid_password() {
    if !PART2 {
        assert!(valid_password(111111));
    }
    assert!(!valid_password(223450));
    assert!(!valid_password(123789));
}

fn digits(mut n: u32) -> Vec<u8> {
    let mut result = Vec::new();
    while n > 0 {
        result.push((n % 10) as u8);
        n = n / 10;
    };

    result.reverse();
    result
}

#[test]
fn test_digits() {
    assert_eq!(digits(12345), vec!(1, 2, 3, 4, 5));
}