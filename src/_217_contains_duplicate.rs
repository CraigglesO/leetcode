use std::collections::HashSet;

#[allow(dead_code)]
/// Checks if a vector contains duplicates or not
pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut set: HashSet<i32> = HashSet::new();

    for num in nums {
        if set.contains(&num) {
            return true;
        }
        set.insert(num);
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums: Vec<i32> = vec![1, 2, 3, 1];
        assert!(contains_duplicate(nums))
        // assert_eq!(result, expected);
    }

    #[test]
    fn test_2() {
        let nums: Vec<i32> = vec![1, 2, 3, 4];
        assert!(!contains_duplicate(nums));
    }

    #[test]
    fn test_3() {
        let nums: Vec<i32> = vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2];
        assert!(contains_duplicate(nums));
    }
}
