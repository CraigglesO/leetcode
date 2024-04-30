// add a HashSet
use std::collections::HashMap;

#[allow(dead_code)]
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut sums: HashMap<i32, i32> = HashMap::new();

    for (i, num) in nums.iter().enumerate() {
        let ias32: i32 = i.try_into().unwrap_or_default();
        let diff = target - num;
        if let Some(&prev) = sums.get(&diff) {
            return vec![prev, ias32];
        }
        sums.insert(*num, ias32);
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(vec![0, 1], two_sum(vec![2, 7, 11, 15], 9))
    }

    #[test]
    fn test_2() {
        assert_eq!(vec![1, 2], two_sum(vec![3, 2, 4], 6))
    }

    #[test]
    fn test_3() {
        assert_eq!(vec![0, 1], two_sum(vec![3, 3], 6))
    }
}
