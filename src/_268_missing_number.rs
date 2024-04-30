#[allow(dead_code)]
pub fn missing_number(nums: Vec<i32>) -> i32 {
    nums.into_iter()
        .enumerate()
        .fold(0, |acc, (i, x)| acc ^ (i as i32 + 1) ^ x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(missing_number(vec![3, 0, 1]), 2);
    }

    #[test]
    fn test_2() {
        assert_eq!(missing_number(vec![0, 1]), 2);
    }

    #[test]
    fn test_3() {
        assert_eq!(missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]), 8);
    }
}
