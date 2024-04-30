use std::collections::HashSet;

#[allow(dead_code)]
pub fn is_happy(mut n: i32) -> bool {
    let mut set: HashSet<i32> = HashSet::new();

    while n != 1 && !set.contains(&n) {
        set.insert(n);
        n = square_digits(n);
    }

    n == 1
}

fn square_digits(mut n: i32) -> i32 {
    let mut sum = 0;

    while n > 0 {
        let rem = n % 10;
        sum += rem * rem;
        n /= 10;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(is_happy(19));
    }

    #[test]
    fn test_2() {
        assert!(!is_happy(2));
    }

    #[test]
    fn test_3() {
        assert!(!is_happy(3));
    }

    #[test]
    fn test_4() {
        assert!(!is_happy(9));
    }
}
