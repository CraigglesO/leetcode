#[allow(dead_code)]
pub fn single_number(nums: Vec<i32>) -> i32 {
    nums.into_iter().fold(0, |a, b| a ^ b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(single_number(vec![2, 2, 1]), 1);
    }

    #[test]
    fn test_2() {
        assert_eq!(single_number(vec![4, 1, 2, 1, 2]), 4);
    }

    #[test]
    fn test_3() {
        assert_eq!(single_number(vec![1]), 1);
    }
}
