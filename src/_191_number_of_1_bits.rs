#[allow(dead_code)]
pub fn hamming_weight(mut n: i32) -> i32 {
    let mut count = 0;
    while n > 0 {
        count += n & 1;
        n >>= 1;
    }

    count
}

// NOTE: You can just do `n.count_ones() as i32` but that's less interesting

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(hamming_weight(11), 3);
    }

    #[test]
    fn test_2() {
        assert_eq!(hamming_weight(128), 1);
    }

    #[test]
    fn test_3() {
        assert_eq!(hamming_weight(2147483645), 30);
    }
}
