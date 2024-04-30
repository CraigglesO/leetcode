#[allow(dead_code)]
pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut res = vec![1; nums.len()];
    let mut curr = 1;

    // prefix
    for i in 0..nums.len() {
        res[i] *= curr;
        curr *= nums[i];
    }

    // suffix
    curr = 1;
    for i in (0..nums.len()).rev() {
        res[i] *= curr;
        curr *= nums[i];
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(product_except_self(vec![1, 2, 3, 4]), vec![24, 12, 8, 6]);
    }

    #[test]
    fn test_2() {
        assert_eq!(
            product_except_self(vec![-1, 1, 0, -3, 3]),
            vec![0, 0, 9, 0, 0]
        );
    }
}
