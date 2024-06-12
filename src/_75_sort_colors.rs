#[allow(dead_code)]
pub fn sort_colors(nums: &mut [i32]) {
    let n = nums.len();
    for i in 0..n {
        for j in 0..nums.len() - i - 1 {
            if nums[j] > nums[j + 1] {
                nums.swap(j, j + 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut nums = vec![2, 0, 2, 1, 1, 0];
        sort_colors(&mut nums);
        assert_eq!(nums, vec![0, 0, 1, 1, 2, 2]);
    }

    #[test]
    fn test_2() {
        let mut nums = vec![2, 0, 1];
        sort_colors(&mut nums);
        assert_eq!(nums, vec![0, 1, 2]);
    }

    #[test]
    fn test_3() {
        let mut nums = vec![2];
        sort_colors(&mut nums);
        assert_eq!(nums, vec![2]);
    }

    #[test]
    fn test_4() {
        let mut nums = vec![2, 2];
        sort_colors(&mut nums);
        assert_eq!(nums, vec![2, 2]);
    }
}
