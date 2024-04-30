use std::cmp::Ordering;

#[allow(dead_code)]
pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let (mut left, mut right) = (0, nums.len());

    while left < right {
        let mid = (left + right) / 2;
        match nums[mid].cmp(&target) {
            Ordering::Equal => {
                return mid as i32;
            }
            Ordering::Less => {
                left = mid + 1;
            }
            Ordering::Greater => {
                right = mid;
            }
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
    }

    #[test]
    fn test_2() {
        assert_eq!(search(vec![2, 5], 0), -1);
    }

    #[test]
    fn test_3() {
        assert_eq!(search(vec![-1, 0, 5], 5), 2);
    }
}
