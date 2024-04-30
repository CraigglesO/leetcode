#[allow(dead_code)]
pub fn get_sum(a: i32, b: i32) -> i32 {
    if b == 0 {
        return a;
    }
    get_sum(a ^ b, (a & b) << 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(get_sum(1, 2), 3);
    }

    #[test]
    fn test_2() {
        assert_eq!(get_sum(2, 3), 5);
    }

    #[test]
    fn test_3() {
        assert_eq!(get_sum(0, 7), 7);
    }

    #[test]
    fn test_4() {
        assert_eq!(get_sum(7, 0), 7);
    }
}
