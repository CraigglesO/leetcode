#[allow(dead_code)]
pub fn min_increment_for_unique(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable();
    let mut res = 0;
    for i in 1..nums.len() {
        if nums[i] <= nums[i - 1] {
            res += nums[i - 1] - nums[i] + 1;
            nums[i] = nums[i - 1] + 1;
        }
    }

    res
}

// https://leetcode.com/problems/minimum-increment-to-make-array-unique/description/
// NOTE: My take on these kinds of problems is the solution is always just sort the array and the rest is just busy work.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(min_increment_for_unique(vec![1, 2, 2]), 1);
    }

    #[test]
    fn test_2() {
        assert_eq!(min_increment_for_unique(vec![3, 2, 1, 2, 1, 7]), 6);
    }

    #[test]
    fn test_3() {
        assert_eq!(min_increment_for_unique(vec![1, 1, 2, 2, 3]), 6);
    }
}
